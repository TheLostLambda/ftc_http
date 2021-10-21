use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize)]
pub struct Files {
    pub src: Vec<String>,
}

#[derive(Deserialize)]
pub struct BuildStatus {
    pub completed: bool,
    pub successful: bool,
}

#[derive(Debug)]
pub enum RobotError {
    BuildTimeout(Duration),
    NoJavaPackage(PathBuf),
    NotConnected,
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotConnected => write!(
                f,
                "No known hosts were online.\n\n
                Please check that your robot controller is in \"Program & Manage\" mode and\n\
                that your computer is connected to the robot controller via wifi-direct.\n\
                Alternatively, you can try manually specifying a host address with the --host\n\
                option or extending the timeout period with the --host-timeout-ms option."
            ),
            Self::NoJavaPackage(path) => write!(
                f,
                "Could not resolve the Java package for {}\n\n\
                 Does the file contain a correctly formatted `package ...;` line? A package\n\
                 declaration is required by OnBotJava to assign paths to uploaded files.",
                path.display()
            ),
            Self::BuildTimeout(duration) => write!(
                f,
                "The build has taken more than {} seconds to complete and appears unresponsive.\n\n\
                 Please restart the robot controller or, if this problem persists, increase the\n\
                 build timeout using the --build-timeout-sec option.",
                duration.as_secs()
            ),
        }
    }
}

impl Error for RobotError {}

macro_rules! catch {
    ($result:expr, $code:expr, $fmt:expr $(, $arg:tt)*) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                eprintln!($fmt, $($arg, )* e = err);
                std::process::exit($code);
            }
        }
    }
}

pub fn java_package_to_path(java_file: &Path) -> Result<String> {
    lazy_static! {
        static ref PACKAGE_LINE: Regex = Regex::new(r"package ([\S]+);").unwrap();
        static ref PACKAGE_PATH: Regex = Regex::new(r"[^.]+").unwrap();
    }
    let reader = BufReader::new(File::open(java_file)?);

    for line in reader.lines() {
        if let Some(package) = PACKAGE_LINE.captures(&line?) {
            let path: Vec<String> = PACKAGE_PATH
                .captures_iter(&package[1])
                .map(|c| c[0].to_string())
                .collect();
            let file_name = java_file.file_name().unwrap().to_string_lossy();
            return Ok(format!("/{}/{}", path.join("/"), file_name));
        }
    }

    Err(RobotError::NoJavaPackage(java_file.to_owned()).into())
}
