use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config { paths: vec![] }
    }
}

pub fn load_config(path: PathBuf) -> Config {
    let config = std::fs::read_to_string(path);
    match config {
        Ok(conf) => {
            let config = toml::from_str::<Config>(&conf);

            match config {
                Ok(conf) => return conf,
                Err(_) => println!("Failed to parse config file"),
            }
        }
        Err(_) => println!("Failed to open config file"),
    }
    return Config::default();
}
