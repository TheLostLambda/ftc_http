mod robot;

use reqwest::blocking::*;
use std::error::Error;
use structopt::StructOpt;
use crate::robot::controller::RobotController;
use std::path::Path;

#[structopt(about = "Provides an interface to FTC OnBotJava from outside the browser")]
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    download: bool,
    #[structopt(short, long)]
    upload: bool,
    #[structopt(short, long)]
    build: bool,
    #[structopt(short, long)]
    wipe: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opts::from_args();
    let r = RobotController::new(None)?;
    r.download(Path::new("/home/tll/Downloads"));
    Ok(())
}
