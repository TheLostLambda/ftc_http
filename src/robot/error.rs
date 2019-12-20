use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum RobotError {
    NotConnected,
}

impl fmt::Display for RobotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to reach the robot controller. Please check that your robot controller\n\
       is in \"Program & Manage\" mode and that your computer is connected to the\n\
       robot controller via wifi-direct."
        )
    }
}

impl Error for RobotError {}
