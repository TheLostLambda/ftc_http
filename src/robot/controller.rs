use crate::robot::{
    config::AppConfig,
    util::{java_package_to_path, BuildStatus, Files, Result, RobotError},
};
use reqwest::blocking::{multipart, Client};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    thread,
    time::{Duration, Instant},
};
use url::Url;
use walkdir::WalkDir;

pub struct RobotController {
    client: Client,
    host: Url,
    build_timeout: Duration,
}

impl RobotController {
    pub fn new(conf: &mut AppConfig) -> Result<Self> {
        // Make a HTTP client with a custom connection timeout and cookie support
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
                    return Ok(Self {
                        client,
                        host,
                        build_timeout: conf.build_timeout,
                    });
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
        // Get a list of the remote files on the device
        let files = self.get_files()?;
        // Loop over all files with a `.java` extension, downloading them one at a time
        for file in files.iter().filter(|s| s.contains(".java")) {
            let path = dest.join(&file[1..]);
            fs::create_dir_all(path.parent().unwrap())?;

            print!("Pulling {}...", path.display());
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
        // Get a listing of all `.java` files in the local target directory
        let local_files = WalkDir::new(src)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |t| t == "java"))
            .map(|e| e.into_path());

        // Get a listing of remote files, to resolve upload conflicts if a file already exists
        let remote_files = self.get_files()?;

        for file in local_files {
            print!("Pushing {}...", &file.display());
            io::stdout().flush()?;

            // Predict the remote path using Java `package` information
            let java_path = java_package_to_path(&file)?;
            // Does the local file already exist on the robot controller?
            if remote_files.contains(&java_path) {
                // If so, delete it before attempting an upload
                self.delete(&java_path)?;
            }

            // Finally, upload the local file
            let url = self.host.join("/java/file/upload")?;
            let form = multipart::Form::new().file("file", file)?;
            self.client.post(url).multipart(form).send()?;

            println!("done");
        }

        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        // Start a build on the robot controller
        let url = self.host.join("/java/build/start")?;
        self.client.get(url).send()?;

        print!("Building...");
        io::stdout().flush()?;

        // Record the current time and poll the build until completion. If the build takes too long
        // to complete, bail out with an informative error
        let build_start = Instant::now();
        while build_start.elapsed() < self.build_timeout {
            // Check the build status
            let url = self.host.join("/java/build/status")?;
            let status: BuildStatus = self.client.get(url).send()?.json()?;

            // If the build has completed
            if status.completed {
                // Check if it was successful
                if status.successful {
                    // If it was, inform the user
                    println!("BUILD SUCCESSFUL");
                } else {
                    // Otherwise inform the user of failure
                    println!("BUILD FAILED");

                    // And print the encountered build errors
                    let url = self.host.join("/java/build/wait")?;
                    println!("{}", self.client.get(url).send()?.text()?);
                }
                // Return from the function since the build is complete
                return Ok(());
            }

            print!(".");
            io::stdout().flush()?;

            // Wait half a second between polling requests
            thread::sleep(Duration::from_millis(500));
        }

        // If we made it all of the way here, the build has timed-out and we should inform the user
        println!("BUILD TIMEOUT");
        Err(RobotError::BuildTimeout(self.build_timeout).into())
    }

    pub fn wipe(&self) -> Result<()> {
        print!("Wiping all remote files...");
        io::stdout().flush()?;

        // Recursively delete all files on the robot controller
        self.delete("/")?;

        println!("done");
        Ok(())
    }

    fn delete(&self, file: &str) -> Result<()> {
        let url = self.host.join("/java/file/delete")?;
        // Build the form params to be POSTed to delete the file
        let params = [("delete", format!(r#"["src/{}"]"#, &file[1..]))];
        self.client.post(url).form(&params).send()?;

        Ok(())
    }

    fn get_files(&self) -> Result<Vec<String>> {
        // Read the remote file-tree into a vector of paths
        let url = self.host.join("/java/file/tree")?;
        let tree: Files = self.client.get(url).send()?.json()?;
        Ok(tree.src)
    }
}
