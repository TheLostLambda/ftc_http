extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate uuid;
extern crate walkdir;

use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::{Core, Timeout};
use hyper::{Client, Method, Request};
use std::io::{stdout, Read, Write};
use walkdir::{DirEntry, WalkDir};
use futures::{Future, Stream};
use futures::future::Either;
use hyper::error::Error;
use std::time::Duration;
use std::result::Result;
use std::process;
use uuid::Uuid;
use std::fs;

const HOSTS: [&'static str; 2] = ["http://192.168.43.1:8080", "http://192.168.49.1:8080"];
const TIMEOUT: u64 = 3;

pub fn up(src: &std::path::Path) -> Result<(), Error> {
    let host = conn_test()?;

    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    fn is_src_file(entry: &DirEntry) -> bool {
        entry.file_name().to_str().unwrap().contains(".java")
    }

    let src_tree = WalkDir::new(&src)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(is_src_file);

    for file in src_tree.map(|f| f.path().to_owned()) {
        print!("Pushing {}...", file.display());
        stdout().flush()?;

        let mut file_data = String::new();
        fs::File::open(&file)?.read_to_string(&mut file_data)?;

        let boundary = Uuid::new_v4().simple().to_string();

        let mut upload_request = Request::new(
            Method::Post,
            (host.to_string() + "/java/file/upload").parse()?,
        );

        let body = format!(
            "--{}\nContent-Disposition: form-data; name=\"file\"; \
             filename=\"{}\"\nContent-Type: text/x-java\n\n{}\n\n--{}--",
            boundary,
            &file.file_name().unwrap().to_str().unwrap(),
            file_data,
            boundary
        );

        upload_request.headers_mut().set_raw(
            "Content-Type",
            "multipart/form-data; boundary=".to_string() + &boundary,
        );

        upload_request
            .headers_mut()
            .set(ContentLength(body.len() as u64));

        upload_request.set_body(body);

        let upload_request = robot_controller.request(upload_request);
        core.run(upload_request)?;

        println!("done");
    }

    Ok(())
}

pub fn down(dest: &std::path::Path) -> Result<(), hyper::Error> {
    let host = conn_test()?;

    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    let tree_request = robot_controller
        .get((host.to_string() + "/java/file/tree").parse()?)
        .and_then(|res| {
            res.body()
                .concat2()
                .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
        });

    let tree = core.run(tree_request)?;

    for file in tree.split("\"").filter(|file| file.contains(".java")) {
        print!("Pulling {}...", file);
        stdout().flush()?;

        let filepath = dest.join(&file[1..]);
        fs::DirBuilder::new()
            .recursive(true)
            .create(filepath.parent().unwrap())?;

        let mut file_handle = fs::File::create(&filepath)?;

        let download_request = robot_controller
            .get((host.to_string() + "/java/file/download?f=/src" + file).parse()?)
            .and_then(|res| {
                res.body()
                    .for_each(|chunk| file_handle.write_all(&chunk).map_err(From::from))
            });

        core.run(download_request)?;

        println!("done");
    }

    Ok(())
}

pub fn build() -> Result<(), Error> {
    let host = conn_test()?;

    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    let tree_request = robot_controller.get((host.to_string() + "/java/file/tree").parse()?);

    let build_request = robot_controller.get((host.to_string() + "/java/build/start").parse()?);

    core.run(tree_request.join(build_request))?;

    print!("Building...");
    stdout().flush()?;

    let mut status;

    loop {
        let status_request = robot_controller
            .get((host.to_string() + "/java/build/status").parse()?)
            .and_then(|res| {
                res.body()
                    .concat2()
                    .map(|chunk| String::from_utf8(chunk.to_vec()).unwrap())
            });

        status = core.run(status_request)?;

        if status.contains("\"completed\": true") {
            break;
        } else {
            print!(".");
            stdout().flush()?;
        }
    }

    if status.contains("\"successful\": true") {
        println!("BUILD SUCCESSFUL");
    } else {
        println!("BUILD FAILED");

        let error_request = robot_controller
            .get((host.to_string() + "/java/build/wait").parse()?)
            .and_then(|res| {
                res.body()
                    .for_each(|chunk| std::io::stdout().write_all(&chunk).map_err(From::from))
            });

        core.run(error_request)?;
    }

    Ok(())
}

pub fn wipe() -> Result<(), Error> {
    let host = conn_test()?;

    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    print!("Wiping all remote files...");
    stdout().flush()?;

    let mut wipe_request = Request::new(
        Method::Post,
        (host.to_string() + "/java/file/delete").parse()?,
    );

    let body = "delete=[\"src\"]";

    wipe_request
        .headers_mut()
        .set(ContentLength(body.len() as u64));
    wipe_request
        .headers_mut()
        .set(ContentType::form_url_encoded());

    wipe_request.set_body(body);

    let wipe_request = robot_controller.request(wipe_request);
    core.run(wipe_request)?;

    println!("done");

    Ok(())
}

fn conn_test() -> Result<&'static str, Error> {
    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    for host in &HOSTS {
        let timeout = Timeout::new(Duration::from_secs(TIMEOUT), &core.handle())?;
        let mut fail = false;
        let conn_request = robot_controller
            .get(host.parse()?)
            .select2(timeout)
            .map(|res| match res {
                Either::B(_) => fail = true,
                _ => (),
            });
        
        core.run(conn_request).unwrap_or_default();
        
        if fail {
            continue;
        } else {
            return Ok(host)
        }
    }

    println!(
        "Failed to reach the robot controller. Please check that your robot controller\n\
         is in \"Program & Manage\" mode and that your computer is connected to the\n\
         robot controller via wifi-direct."
    );

    process::exit(0);
}
