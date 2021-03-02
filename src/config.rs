use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) project_name: String,
    pub(crate) jre_version: String,
    pub(crate) launcher_url: String,
    pub(crate) check_jre: bool,
    pub(crate) user_agent: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../config.json")).unwrap()
    }
}
