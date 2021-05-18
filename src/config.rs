use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub project: ProjectConfig,
    pub window: WindowConfig,
}

#[derive(Deserialize, Clone)]
pub struct ProjectConfig {
    pub project_name: String,
    pub jre_version: String,
    pub launcher_url: String,
    pub check_jre: bool,
    pub user_agent: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    pub window_title: String,
    pub progress_bar_background: String,
    pub progress_bar_color: String,
    pub text_color: String,
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../config.json")).unwrap()
    }
}
