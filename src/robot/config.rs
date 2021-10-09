use core::time::Duration;
use serde::{Deserialize, Serialize};

// TODO: I need a better name
#[derive(Serialize, Deserialize)]
pub struct RobotConfig {
    pub hosts: Vec<String>,
    pub timeout: Duration,
}

impl Default for RobotConfig {
    fn default() -> Self {
        // Should I really be using .into() here?
        // Maybe prefer something a little more explicit?
        let hosts = vec![
            "http://192.168.43.1:8080".into(),
            "http://192.168.49.1:8080".into(),
        ];
        let timeout = Duration::from_millis(500);
        Self { hosts, timeout }
    }
}
