use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub hosts: Vec<String>,
    pub host_timeout: Duration,
    pub build_timeout: Duration,
}

impl Default for AppConfig {
    fn default() -> Self {
        // Should I really be using .into() here?
        // Maybe prefer something a little more explicit?
        let hosts = vec![
            "http://192.168.43.1:8080".into(),
            "http://192.168.49.1:8080".into(),
        ];
        let host_timeout = Duration::from_millis(500);
        let build_timeout = Duration::from_secs(15);
        Self {
            hosts,
            host_timeout,
            build_timeout,
        }
    }
}
