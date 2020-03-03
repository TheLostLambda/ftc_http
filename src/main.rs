#[macro_use]
mod robot;

use crate::robot::config::RobotConfig;
use crate::robot::controller::RobotController;
use crate::robot::error::*;
use core::time::*;
use std::iter;
use std::path::Path;
use clap::Clap;

#[clap(author)]
#[derive(Clap)]
/// Provides an interface to FTC OnBotJava from outside the browser.
///
/// Flags can be combined to perform a series of actions following a single
/// invocation. This somewhat contrived example of this would be the following
/// command:
///
/// ftc_http -dwub foo/ bar/
///
/// This command downloads a copy of the code from the robot controller (saving
/// it in the foo/ directory), wipes the robot controller, uploads a fresh copy
/// of the code (from the bar/ directory), and builds it.
struct FTC {
    /// Download .java files from the robot controller.
    ///
    /// Source files are saved to the location specified in DIRS. This defaults to
    /// the current directory.
    ///
    /// Files on the local computer are never deleted by ftc_http, though old
    /// files with the same name are overwritten. Be sure to save to a fresh
    /// location if you don't want to risk overwriting old source files.
    #[clap(short, long)]
    download: bool,
    /// Uploads .java files to the robot controller.
    ///
    /// Uploads files from the location specified in DIRS. Defaults to the
    /// current directory. Source files are recursively located by their .java
    /// extension.
    #[clap(short, long)]
    upload: bool,
    /// Builds the code on the robot controller.
    ///
    /// Initiates a build on the robot controller and reports the build status
    /// and any errors back to the user.
    #[clap(short, long)]
    build: bool,
    /// Wipes all files from the robot controller.
    ///
    /// Using this option ensures that files deleted on the local machine are
    /// also deleted on the robot controller. Be cautious and make a backup with
    /// the -d option before wiping anything.
    #[clap(short, long)]
    wipe: bool,
    /// A list of directories used by the download and upload options.
    ///
    /// Between 0 and 2 directories can be specified. When -d and -u are used
    /// together, the first directory is where files are downloaded and the
    /// second is where they are uploaded from.
    #[clap(name = "DIRS")]
    directories: Vec<String>,
    /// Manually specify the address of the robot controller.
    ///
    /// Addresses are given in the form: "http://<IP>:<PORT>"
    #[clap(long, name = "ADDR")]
    host: Option<String>,
    /// Manually specify the connection timeout.
    ///
    /// Wait at least this long before declaring a robot controller offline
    /// (given in milliseconds).
    #[clap(long, name = "DELAY")]
    timeout_ms: Option<u64>,
    /// Reset the host and timeout values to their defaults.
    ///
    /// This deletes any custom values that have been automatically remembered.
    #[clap(long)]
    restore_defaults: bool,
}

fn main() {
    let opt = FTC::parse();
    if opt.restore_defaults {
        catch!(
            confy::store("ftc_http", RobotConfig::default()),
            1,
            "Failed {} to save configuration to file. \n\n{e}"
        );
    } else if opt.download || opt.wipe || opt.upload || opt.build {
        let mut dirs = opt
            .directories
            .iter()
            .map(|d| Path::new(d))
            .chain(iter::repeat(Path::new(".")));
        let mut conf: RobotConfig = catch!(
            confy::load("ftc_http"),
            2,
            "Failed to read configuration from file. \n\n{e}"
        );
        if let Some(host) = opt.host {
            if !conf.hosts.contains(&host) {
                conf.hosts.insert(0, host);
            }
        }
        if let Some(ms) = opt.timeout_ms {
            conf.timeout = Duration::from_millis(ms);
        }
        let r = catch!(
            RobotController::new(&mut conf),
            3,
            "Failed to establish a connection with the robot controller. \n\n{e}"
        );
        catch!(
            confy::store("ftc_http", conf),
            1,
            "Failed {} to save configuration to file. \n\n{e}"
        );
        if opt.download {
            catch!(
                r.download(dirs.next().unwrap()),
                4,
                "Failed to download source files from the robot controller. \n\n{e}"
            );
        }
        if opt.wipe {
            catch!(
                r.wipe(),
                5,
                "Failed to wipe source files from the robot controller. \n\n{e}"
            );
        }
        if opt.upload {
            catch!(
                r.upload(dirs.next().unwrap()),
                6,
                "Failed to upload source files to the robot controller. \n\n{e}"
            );
        }
        if opt.build {
            catch!(
                r.build(),
                7,
                "Failed to build the source file on the robot controller. \n\n{e}"
            );
        }
    } else {
        println!("Try running with -h for a usage summary or --help for a more complete manual.");
    }
}
