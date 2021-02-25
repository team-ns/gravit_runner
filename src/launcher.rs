use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;
use dirs::data_dir;
use crate::util::launcher_dir;
use std::{fs, io};
use std::io::Write;

#[cfg(windows)]
static JAVA_FILE: &'static str = "java.exe";

#[cfg(not(windows))]
static JAVA_FILE: &'static str = "java";

pub struct Launcher {
    pub(crate) url: String,
}

impl Launcher {
    pub fn run_launcher<P: AsRef<Path>>(&self, launcher_path: P, jre_path: P) -> Result<()> {
        let mut jre_path = jre_path.as_ref().join("bin").join(JAVA_FILE);
        Command::new(&jre_path)
            .args(&["-jar", launcher_path.as_ref().to_str().context("Can't convert path to string")?])
            .output()?;
        Ok(())
    }

    pub fn download_launcher<P: AsRef<Path>>(&self, launcher_path: P) -> Result<()> {
        let response = minreq::get(&self.url).send()?.as_bytes().to_vec();
        if let Some(parent) = launcher_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(launcher_path.as_ref())?;
        file.write(&response);
        Ok(())
    }
}