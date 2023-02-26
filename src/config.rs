use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs, path::PathBuf};
/// Application data and config files path
pub struct ProjPaths {
    pub config_path: PathBuf,
}
#[derive(Serialize, Deserialize)]
pub struct Configs {
    pub projects_paths: Vec<String>,
    pub github_token: String,
}

lazy_static! {
    pub static ref PROJ_PATHS: ProjPaths = {
        let path = home::home_dir().unwrap();
        fs::create_dir_all(path.clone()).unwrap();

        let config_dir_path = path.join(".config").join("yatap");
        fs::create_dir_all(config_dir_path.clone()).unwrap();

        let config_path = path.join(".config").join("yatap/config.json");

        ProjPaths { config_path }
    };
}

pub fn get_config() -> Result<Configs> {
    let stringified_configs = fs::read_to_string(PROJ_PATHS.config_path.as_path())
        .unwrap_or_else(|_| create_base_config());
    let configs: Configs = serde_json::from_str(&stringified_configs)?;
    Ok(configs)
}

fn create_base_config() -> String {
    let base_config = Configs {
        projects_paths: vec![home::home_dir().unwrap().join("dev").display().to_string()],
        github_token: String::from(""),
    };

    let stringified_config = serde_json::to_string(&base_config).unwrap();
    fs::write(PROJ_PATHS.config_path.as_path(), stringified_config).unwrap();
    fs::read_to_string(PROJ_PATHS.config_path.as_path()).unwrap()
}
