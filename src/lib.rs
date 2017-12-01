extern crate walkdir;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::error::Error;
use std::result::Result;

static HOST: &'static str = "http://192.168.49.1:8080";
const TIMEOUT: u64 = 3;

pub fn up(src: &std::path::Path) -> Result<(),Error> {
    unimplemented!();
}

pub fn down(dest: &std::path::Path) -> Result<(), hyper::Error> {
    //conn_test();
    let mut core = Core::new()?;
    let robot_controller = Client::new(&core.handle());

    let mut tree = String::new();

    let request = robot_controller
        .get((HOST.to_string() + "/java/file/tree").parse()?)
        .and_then(|response| {
            println!("Response: {}", response.status());

            response.body().for_each(|chunk| {
                io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
            })
        });

    core.run(request)?;
    Ok(())
        /*.expect("HTTP request failed")
        .read_to_string(&mut tree)
        .expect("Couldn't read HTTP response");

    for file in tree.split("\"").filter(|file| file.contains(".java")) {
        print!("Pulling {}...", file);
        stdout().flush().unwrap();
        let filepath = dest.join(&file[1..]);
        fs::DirBuilder::new()
            .recursive(true)
            .create(filepath.parent().unwrap())
            .expect("Creating a new directory failed");
        let mut file_handle = fs::File::create(&filepath).expect("Creating a new file failed");
        let mut file_data = String::new();
        phone
            .get(&(HOST.to_string() + "/java/file/download?f=/src" + file))
            .send()
            .expect("HTTP request failed")
            .read_to_string(&mut file_data)
            .expect("Couldn't read HTTP response");
        file_handle
            .write_all(file_data.as_bytes())
            .expect("Writing to file failed");
        println!("done");*/
}

pub fn build() -> Result<(),Error> {
    unimplemented!();
}

pub fn wipe() -> Result<(),Error> {
    /*conn_test();
    let mut core = Core::new()?;
    let phone = Client::new(&core.handle());
    let uri =
        &(HOST.to_string() + "/java/file/delete")
        .parse()
        .expect("Parsing URI failed");

    print!("Wiping all remote files...");
    stdout().flush().unwrap();
    let post = Request::new(Method.Post, uri)
        .set_body("delete=[\"src\"]")
    println!("done");*/
    unimplemented!();
}

fn conn_test() {
    unimplemented!();
}
