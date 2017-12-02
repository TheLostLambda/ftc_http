extern crate walkdir;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::io::{stdout, Write};
use hyper::error::Error;
use std::result::Result;
use hyper::Client;
use std::fs;

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

    let tree_request = robot_controller
        .get((HOST.to_string() + "/java/file/tree").parse()?)
        .and_then(|res| res.body()
            .concat2()
            .map(|chunk|
                String::from_utf8(chunk.to_vec()).unwrap()
            )
        );

    tree = core.run(tree_request)?;

    for file in tree.split("\"").filter(|file| file.contains(".java")) {
        print!("Pulling {}...", file);
        stdout().flush()?;
        let filepath = dest.join(&file[1..]);
        fs::DirBuilder::new()
            .recursive(true)
            .create(filepath.parent().unwrap())?;
        let mut file_handle = fs::File::create(&filepath)?;
        let download_request = robot_controller
            .get((HOST.to_string() + "/java/file/download?f=/src" + file).parse()?)
            .and_then(|res| res.body()
                .for_each(|chunk| file_handle.write_all(&chunk).map_err(From::from))
            );
        core.run(download_request)?;
        println!("done");
    }

    Ok(())
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
