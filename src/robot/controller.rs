use reqwest::blocking::*;
use std::path::*;
use crate::robot::error::*;
use crate::robot::config::*;
use std::io;
use std::fs;
// Gross
use std::io::{Write};

pub struct RobotController {
    client: Client,
    host: String,
}

impl RobotController {
    // Get rid of this option!
    pub fn new(conf: Option<RobotConfig>) -> Result<Self> {
        // Test for a config or fallback to the default
        let RobotConfig { hosts, timeout } = conf.unwrap_or_default();
        // Make a HTTP client with a custom connection timeout
        let client = Client::builder()
            .connect_timeout(timeout)
            .build()?;
        // Start pinging hosts to see which one the robot controller is on
        for host in hosts {
            println!("Trying host: {}", host);
            match client.get(&host).send() {
                Ok(resp) if resp.status().is_success() =>
                    return Ok(Self { client, host }),
                Ok(resp) => {println!("{:?}", resp); todo!() },
                _ => continue,
            }
        }
        // If no hosts are online, conclude that we aren't connected
        Err(RobotError::NotConnected.into())
    }
    // The handling of no path is done in main
    pub fn download(&self, dest: &Path) -> Result<()> {
        let url = self.host.clone() + "/java/file/tree";
        let tree = self.client.get(&url).send()?.text()?;
        for file in tree.split("\"").filter(|s| s.contains(".java")) {
            print!("Pulling {}...", file);
            io::stdout().flush()?;

            let path = dest.join(&file[1..]);
            fs::create_dir_all(path.parent().unwrap())?;

            let url = self.host.clone() + "/java/file/download?f=/src" + file;
            let data = self.client.get(&url).send()?.text()?;

            fs::write(&path, &data);
            
            println!("done");
        }
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        let url = self.host.clone() + "/java/file/tree";
        self.client.get(&url).send()?;
        let url = self.host.clone() + "/java/build/start";
        self.client.get(&url).send()?;

        print!("Building...");
        io::stdout().flush()?;

        let url = self.host.clone() + "/java/build/status";
        let status = loop {
            let status = self.client.get(&url).send()?.text()?;

            if status.contains("\"completed\": true") {
                break status;
            } else {
                print!(".");
                io::stdout().flush()?;
            }
        };

        if status.contains("\"successful\": true") {
            println!("BUILD SUCCESSFUL");
        } else {
            println!("BUILD FAILED");

            let url = self.host.clone() + "/java/build/wait";
            println!("{}", self.client.get(&url).send()?.text()?);
        }
        
        Ok(())
    }
}
