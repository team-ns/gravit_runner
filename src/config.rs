use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub(crate) project_name: String,
    pub(crate) jre_version: String,
    pub(crate) launcher_url: String,
    pub(crate) check_jre: bool,
}

impl Default for Config {
    fn default() -> Self {
        serde_json::from_str(include_str!("../config.json")).unwrap()
    }
}