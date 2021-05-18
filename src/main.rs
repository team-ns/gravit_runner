#![windows_subsystem = "windows"]

use crate::jre::Jre;
use crate::launcher::Launcher;
use crate::util::launcher_dir;
use anyhow::{Context, Result};
use std::path::Path;

use crate::config::{Config, ProjectConfig};
use crate::ui::{build_root_widget, AppState, Delegate, Status, UPDATE_STAGE};
use druid::{theme, AppLauncher, Color, PlatformError, Target, WindowDesc};
use std::{fs, thread};

mod config;
mod jre;
mod launcher;
mod ui;
mod util;

#[derive(PartialEq, Eq, Clone)]
pub enum Stage {
    Launcher,
    DownloadJre,
    CheckDownload,
    CheckExtract,
    None,
}

fn main() -> Result<(), PlatformError> {
    let (project_config, window_config) = {
        let config = Config::default();
        (config.project, config.window)
    };

    let main_window = WindowDesc::new(build_root_widget)
        .with_min_size((0., 0.))
        .window_size((300., 120.))
        .title(window_config.window_title.as_str())
        .resizable(false);

    let initial_state = AppState {
        stage: 0.0,
        stage_text: "Download Launcher".to_string(),
    };
    let launcher = AppLauncher::with_window(main_window);

    let event_sink = launcher.get_external_handle();

    thread::spawn(move || run(project_config, event_sink));

    launcher
        .use_simple_logger()
        .delegate(Delegate)
        .configure_env(move |env, _| {
            env.set(
                theme::LABEL_COLOR,
                Color::from_hex_str(&window_config.text_color).unwrap(),
            );
            // Progress bar background
            env.set(
                theme::BACKGROUND_DARK,
                Color::from_hex_str(&window_config.progress_bar_background).unwrap(),
            );
            env.set(
                theme::BACKGROUND_LIGHT,
                Color::from_hex_str(&window_config.progress_bar_background).unwrap(),
            );
            // Progress bar
            env.set(
                theme::PRIMARY_LIGHT,
                Color::from_hex_str(&window_config.progress_bar_color).unwrap(),
            );
            env.set(
                theme::PRIMARY_DARK,
                Color::from_hex_str(&window_config.progress_bar_color).unwrap(),
            );
        })
        .launch(initial_state)?;

    Ok(())
}

fn run(config: ProjectConfig, event_sink: druid::ExtEventSink) -> Result<()> {
    let project_name = &config.project_name;
    let check_installed = config.check_jre;
    let launcher_url = &config.launcher_url;
    let project_path = launcher_dir(project_name).context("Can't get launcher path")?;
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
    let current_stage = Stage::None;
    loop {
        let stage = get_stage(&zip_path, &folder_path);
        if current_stage.eq(&stage) {
            break;
        }
        match stage {
            Stage::Launcher => {
                if !launcher_path.is_file() {
                    println!("Download Launcher");
                    event_sink.submit_command(UPDATE_STAGE, Status::DownloadLauncher, Target::Auto)?;
                    launcher
                        .download_launcher(&launcher_path)
                        .context("Can't download launcher")?;
                } else {
                    println!("Verify Launcher");
                    event_sink.submit_command(UPDATE_STAGE, Status::VerifyLauncher, Target::Auto)?;
                    if let Err(_) = launcher.check_launcher(&launcher_path) {
                        fs::remove_file(&launcher_path).context("Can't remove invalid launcher")?;
                        continue;
                    }
                }
                println!("Run Launcher");
                event_sink.submit_command(UPDATE_STAGE, Status::RunLauncher, Target::Auto)?;
                launcher
                    .run_launcher(&launcher_path, &folder_path)
                    .context("Can't run launcher")?;
                std::process::exit(0);
            }
            Stage::DownloadJre => {
                println!("Download JRE");
                event_sink.submit_command(UPDATE_STAGE, Status::DownloadJre, Target::Auto)?;
                jre.download_jre(&zip_path).context("Can't download jre")?;
                continue;
            }
            Stage::CheckDownload => {
                println!("Check Download Archive");
                event_sink.submit_command(UPDATE_STAGE, Status::CheckJreArchive, Target::Auto)?;
                match jre.check_jre_archive(&zip_path) {
                    Err(_) => {
                        fs::remove_file(&zip_path).context("Can't delete file")?;
                        continue;
                    }
                    _ => {}
                };
                println!("Extract JRE");
                event_sink.submit_command(UPDATE_STAGE, Status::ExtractJre, Target::Auto)?;
                jre.extract_jre(&folder_path, &zip_path)
                    .context("Can't extract jre")?;
                continue;
            }
            Stage::CheckExtract => {
                println!("Check Extracted JRE");
                event_sink.submit_command(UPDATE_STAGE, Status::CheckJreFolder, Target::Auto)?;
                match jre.check_jre_folder(&folder_path, &zip_path) {
                    Err(_) => {
                        fs::remove_dir_all(&folder_path).context("Can't delete file")?;
                        continue;
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
    Ok(())
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
