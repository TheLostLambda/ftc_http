use std::error::Error;
use std::fmt;

pub use std::process;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
                process::exit($code);
            }
        }
    }
}
