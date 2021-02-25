pub(crate) mod liberica;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::process::Command;

pub trait Jre {
    fn check_jre_archive<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    fn check_jre_folder<P: AsRef<Path>>(&self, folder: P, zip: P) -> Result<()>;

    fn extract_jre<P: AsRef<Path>>(&self, folder: P, zip: P) -> Result<()>;

    fn download_jre<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}

pub fn find_installed_jre() -> Option<PathBuf> {
    Command::new("java")
        .arg("-XshowSettings:properties")
        .arg("-version")
        .output()
        .ok()
        .and_then(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stdout.lines().chain(stderr.lines()) {
                if line.contains("java.home") {
                    let pos = match line.find('=') {
                        None => {
                            continue;
                        }
                        Some(pos) => pos + 1,
                    };
                    let path = line[pos..].trim();
                    let jre_path = PathBuf::from(path);
                    return if jre_path.join("lib").join("ext").join("jfxrt.jar").is_file() {
                        Some(jre_path)
                    } else {
                        None
                    };
                }
            }
            None
        })
}
