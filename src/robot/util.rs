use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

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

// FIXME: There are a few other errors that should use this system!
// The BuiltTimeout and NoPackage come to mind
#[derive(Debug)]
pub enum RobotError {
    NotConnected,
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "No known hosts were online. Please check that your robot controller\n\
             is in \"Program & Manage\" mode and that your computer is connected to the\n\
             robot controller via wifi-direct.\n\n\
             Alternatively, you can try manually specifying a host address with the\n\
             --host option or extending the timeout period with the --timeout-ms option."
        )
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

    Err("Couldn't resolve Java package!\n\nDoes your file contain a `package ...;` line?".into())
}
