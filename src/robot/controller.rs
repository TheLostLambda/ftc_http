use crate::robot::config::*;
use crate::robot::error::*;
use reqwest::blocking::*;
use std::fs;
use std::io;
use std::path::*;
// Gross
use std::io::Write;
use walkdir::WalkDir;

pub struct RobotController {
    client: Client,
    host: String, // Maybe make this a Url struct?
}

impl RobotController {
    pub fn new(conf: &mut RobotConfig) -> Result<Self> {
        // Make a HTTP client with a custom connection timeout
        let client = Client::builder()
            .connect_timeout(conf.timeout)
            .cookie_store(true)
            .build()?;
        // Start pinging hosts to see which one the robot controller is on
        // Is this clone needed? (Prolly) This clones as it goes?
        for (i, host) in conf.hosts.iter().cloned().enumerate() {
            print!("Trying host {}...", host);
            io::stdout().flush()?;
            match client.get(&host).send() {
                Ok(resp) if resp.status().is_success() => {
                    println!("online");
                    conf.hosts.swap(0, i);
                    return Ok(Self { client, host });
                }
                _ => {
                    println!("offline");
                    continue;
                }
            }
        }
        // If no hosts are online, conclude that we aren't connected
        Err(RobotError::NotConnected.into())
    }
    // The handling of no path is done in main
    pub fn download(&self, dest: &Path) -> Result<()> {
        let url = self.host.clone() + "/java/file/tree";
        let tree = self.client.get(&url).send()?.text()?;
        // Maybe actually parse this JSON?
        for file in tree.split('\"').filter(|s| s.contains(".java")) {
            print!("Pulling {}...", file);
            io::stdout().flush()?;

            let path = dest.join(&file[1..]);
            fs::create_dir_all(path.parent().unwrap())?;

            let url = self.host.clone() + "/java/file/download?f=/src" + file;
            let data = self.client.get(&url).send()?.text()?;

            fs::write(&path, &data)?;

            println!("done");
        }
        Ok(())
    }
    pub fn upload(&self, src: &Path) -> Result<()> {
        let src_files = WalkDir::new(src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |t| t == "java"))
            .map(|e| e.into_path());

        let url = self.host.clone() + "/java/file/upload";
        for file in src_files {
            // Ew, not a normal string here...
            print!("Pushing {}...", &file.display());
            io::stdout().flush()?;

            let form = multipart::Form::new().file("file", &file)?;
            self.client.post(&url).multipart(form).send()?;

            println!("done");
        }

        Ok(())
    }
    // Add a self.get function that makes url unneeded?
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

    pub fn wipe(&self) -> Result<()> {
        print!("Wiping all remote files...");
        io::stdout().flush()?;

        let url = self.host.clone() + "/java/file/delete";
        let params = [("delete", "[\"src\"]")];
        self.client.post(&url).form(&params).send()?;

        println!("done");
        Ok(())
    }
}
