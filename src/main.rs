use crate::jre::Jre;
use crate::launcher::Launcher;
use crate::util::launcher_dir;
use std::path::Path;

use crate::config::Config;
use std::fs;

mod config;
mod jre;
mod launcher;
mod util;

enum Stage {
    Launcher,
    DownloadJre,
    CheckDownload,
    CheckExtract,
}

pub fn main() {
    let config = Config::default();
    let project_name = &config.project_name;
    let check_installed = config.check_jre;
    let launcher_url = &config.launcher_url;
    let project_path = launcher_dir(project_name).expect("Can't get launcher path");
    let launcher = Launcher {
        url: launcher_url.to_string(),
        user_agent: config.user_agent.clone(),
    };
    let launcher_path = project_path.join("Launcher.jar");
    let jre = crate::jre::liberica::LibericaJre::new(util::get_os_type(), &config);
    let zip_path = project_path.join("launcher-jre.zip");
    let folder_path = if check_installed {
        if let Some(installed_jre) = jre::find_installed_jre() {
            installed_jre
        } else {
            project_path.join("launcher-jre")
        }
    } else {
        project_path.join("launcher-jre")
    };
    let mut try_counter = 0;
    loop {
        if try_counter > 5 {
            panic!("Can't start launcher")
        }
        try_counter += 1;
        match get_stage(&zip_path, &folder_path) {
            Stage::Launcher => {
                if !launcher_path.is_file() {
                    println!("Download Launcher");
                    launcher
                        .download_launcher(&launcher_path)
                        .expect("Can't download launcher");
                }
                println!("Run Launcher");
                launcher
                    .run_launcher(&launcher_path, &folder_path)
                    .expect("Can't run launcher");
                std::process::exit(0);
            }
            Stage::DownloadJre => {
                println!("Download JRE");
                jre.download_jre(&zip_path).expect("Can't download jre");
                continue;
            }
            Stage::CheckDownload => {
                println!("Check Download Archive");
                match jre.check_jre_archive(&zip_path) {
                    Err(_) => {
                        fs::remove_file(&zip_path).expect("Can't delete file");
                        continue;
                    }
                    _ => {}
                };
                println!("Extract JRE");
                jre.extract_jre(&folder_path, &zip_path)
                    .expect("Can't extract jre");
                continue;
            }
            Stage::CheckExtract => {
                println!("Check Extracted JRE");
                match jre.check_jre_folder(&folder_path, &zip_path) {
                    Err(_) => {
                        fs::remove_dir_all(&folder_path).expect("Can't delete file");
                        continue;
                    }
                    _ => {}
                };
            }
        }
    }
    loop {

    }
}

fn get_stage<P: AsRef<Path>>(zip_path: P, folder_path: P) -> Stage {
    if !zip_path.as_ref().exists() {
        if folder_path.as_ref().is_dir() {
            Stage::Launcher
        } else {
            Stage::DownloadJre
        }
    } else {
        if !folder_path.as_ref().exists() {
            Stage::CheckDownload
        } else {
            Stage::CheckExtract
        }
    }
}
