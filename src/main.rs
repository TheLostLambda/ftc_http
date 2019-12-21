mod robot;

use crate::robot::config::RobotConfig;
use crate::robot::controller::RobotController;
use std::error::Error;
use std::iter;
use std::path::Path;
use structopt::StructOpt;

#[structopt(about = "Provides an interface to FTC OnBotJava from outside the browser")]
#[derive(StructOpt)]
struct FTC {
    #[structopt(short, long)]
    download: bool,
    #[structopt(short, long)]
    upload: bool,
    #[structopt(short, long)]
    build: bool,
    #[structopt(short, long)]
    wipe: bool,
    #[structopt(name = "DIRS")]
    directories: Vec<String>,
    #[structopt(long)]
    host: Option<String>,
}

// Don't return a result here...
fn main() -> Result<(), Box<dyn Error>> {
    let opt = FTC::from_args();
    if opt.download || opt.wipe || opt.upload || opt.build {
        let mut dirs = opt
            .directories
            .iter()
            .map(|d| Path::new(d))
            .chain(iter::repeat(Path::new(".")));
        let mut conf: RobotConfig = confy::load("ftc_http")?;
        if let Some(host) = opt.host {
            if !conf.hosts.contains(&host) {
                conf.hosts.insert(0, host);
            }
        }
        let r = RobotController::new(&mut conf)?;
        confy::store("ftc_http", conf)?;
        if opt.download {
            r.download(dirs.next().unwrap())?;
        }
        if opt.wipe {
            r.wipe()?;
        }
        if opt.upload {
            r.upload(dirs.next().unwrap())?;
        }
        if opt.build {
            r.build()?;
        }
    } else {
        FTC::clap().print_help()?;
        println!(); // Forgetting this is a clap bug...
    }
    Ok(())
}
