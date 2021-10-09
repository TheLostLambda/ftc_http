use crate::robot::{
    config::RobotConfig,
    error::{Result, RobotError},
};
use reqwest::blocking::{multipart, Client};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    thread,
    time::Duration,
};
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

    pub fn download(&self, dest: &Path) -> Result<()> {
        let url = self.host.clone() + "/java/file/tree";
        let tree = self.client.get(&url).send()?.text()?;
        // Maybe actually parse this JSON?
        for file in tree.split('\"').filter(|s| s.contains(".java")) {
            let path = dest.join(&file[1..]);
            fs::create_dir_all(path.parent().unwrap())?;

            print!("Pulling {}...", path.to_string_lossy());
            io::stdout().flush()?;

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

            let send_file = |file: &Path| -> Result<u16> {
                let form = multipart::Form::new().file("file", file)?;
                let resp = self.client.post(&url).multipart(form).send()?;
                Ok(resp.status().as_u16())
            };

            // If a file fails to upload due to a "Bad Request" this probably
            // means that the file already exists on the target. In this case,
            // the old version is deleted and upload is reattempted
            if send_file(&file)? == 400 {
                self.delete(&file)?;
                send_file(&file)?;
            }

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
            }

            print!(".");
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(500));
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

        let url = self.host.clone() + "/java/file/tree";
        let tree = self.client.get(&url).send()?.text()?;
        for file in tree.split('\"').filter(|s| s.contains(".java")) {
            self.delete(Path::new(file))?;
        }

        println!("done");
        Ok(())
    }

    // Maybe this should be exposed to the user at some point?
    fn delete(&self, target: &Path) -> Result<()> {
        let url = self.host.clone() + "/java/file/delete";
        let path = Path::new("src").join(target);
        let params = [("delete", format!("[\"{}\"]", path.display()))];
        self.client.post(&url).form(&params).send()?;

        Ok(())
    }
}
