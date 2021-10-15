use crate::robot::{
    config::AppConfig,
    data::Files,
    error::{Result, RobotError},
};
use reqwest::blocking::{multipart, Client, Response};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    thread,
    time::Duration,
};
use url::Url;
use walkdir::WalkDir;

pub struct RobotController {
    client: Client,
    host: Url,
}

impl RobotController {
    pub fn new(conf: &mut AppConfig) -> Result<Self> {
        // Make a HTTP client with a custom connection timeout
        let client = Client::builder()
            .connect_timeout(conf.host_timeout)
            .cookie_store(true)
            .build()?;
        // Start pinging hosts to see which one the robot controller is on
        for (i, host) in conf.hosts.iter().cloned().enumerate() {
            let host = Url::parse(&host)?;
            print!("Trying host {}...", host);
            io::stdout().flush()?;
            match client.get(host.clone()).send() {
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
        let files = self.get_files()?;
        for file in files.iter().filter(|s| s.contains(".java")) {
            let path = dest.join(&file[1..]);
            fs::create_dir_all(path.parent().unwrap())?;

            print!("Pulling {}...", path.to_string_lossy());
            io::stdout().flush()?;

            let mut url = self.host.join("/java/file/download")?;
            url.set_query(Some(&["f=/src", file].concat()));
            let data = self.client.get(url).send()?.text()?;

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

        let remote_files = self.get_files()?;

        for file in src_files {
            print!("Pushing {}...", &file.display());
            io::stdout().flush()?;

            let send_file = |file: &Path| -> Result<Response> {
                let url = self.host.join("/java/file/upload")?;
                let form = multipart::Form::new().file("file", file)?;
                Ok(self.client.post(url).multipart(form).send()?)
            };

            // If a file fails to upload due to a "Bad Request" this probably
            // means that the file already exists on the target. In this case,
            // the old version is deleted and upload is reattempted
            if send_file(&file)?.status() == 400 {
                let min_path = file.strip_prefix(src)?.to_string_lossy();
                let del_file = remote_files
                    .iter()
                    .find(|f| f.contains(min_path.as_ref()))
                    .unwrap();
                self.delete(Path::new(del_file))?;
                assert!(send_file(&file)?.status().is_success());
            }

            println!("done");
        }

        Ok(())
    }

    // Add a self.get function that makes url unneeded?
    pub fn build(&self) -> Result<()> {
        let url = self.host.join("/java/file/tree")?;
        self.client.get(url).send()?;

        let url = self.host.join("/java/build/start")?;
        self.client.get(url).send()?;

        print!("Building...");
        io::stdout().flush()?;

        let mut tries = 30;
        let status = loop {
            let url = self.host.join("/java/build/status")?;
            let status = self.client.get(url).send()?.text()?;

            if status.contains("\"completed\": true") {
                break status;
            }

            print!(".");
            io::stdout().flush()?;
            if tries == 0 {
                break "timeout".to_string();
            }
            tries -= 1;
            thread::sleep(Duration::from_millis(500));
        };

        if status.contains("\"successful\": true") {
            println!("BUILD SUCCESSFUL");
        } else if status.contains("timeout") {
            println!("BUILD TIMEOUT");
            println!("The build system appears unresponsive. Please restart the robot controller.");
        } else {
            println!("BUILD FAILED");

            let url = self.host.join("/java/build/wait")?;
            println!("{}", self.client.get(url).send()?.text()?);
        }

        Ok(())
    }

    pub fn wipe(&self) -> Result<()> {
        print!("Wiping all remote files...");
        io::stdout().flush()?;

        self.delete(Path::new(""))?;

        println!("done");
        Ok(())
    }

    // Maybe this should be exposed to the user at some point?
    fn delete(&self, target: &Path) -> Result<()> {
        let url = self.host.join("/java/file/delete")?;
        let target = format!("[\"src{}\"]", target.display());
        let params = [("delete", target)];
        self.client.post(url).form(&params).send()?;

        Ok(())
    }

    fn get_files(&self) -> Result<Vec<String>> {
        let url = self.host.join("/java/file/tree")?;
        let tree: Files = self.client.get(url).send()?.json()?;
        Ok(tree.src)
    }
}
