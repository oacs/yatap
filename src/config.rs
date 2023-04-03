use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::File, path::PathBuf};

const APP_NAME: &str = "yatap";
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Deserialize, Serialize, Debug, PartialEq, Default)]
pub struct Config {
    pub paths: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum ConfigLoadErrors {
    ParseFailed,
    OpenConfigFailed,
}

pub fn load_config(path: PathBuf) -> Result<Config, ConfigLoadErrors> {
    let config = std::fs::read_to_string(path).map_err(|_| ConfigLoadErrors::OpenConfigFailed)?;
    let config = toml::from_str::<Config>(&config).map_err(|_| ConfigLoadErrors::ParseFailed)?;
    Ok(config)
}

pub fn setup_config_file() -> Result<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::new()?;
    xdg_dirs.create_config_directory(APP_NAME)?;
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME)?;
    let config_file_path = xdg_dirs.get_config_file(CONFIG_FILENAME);
    let config_file = File::open(config_file_path.clone());
    if config_file.is_err() {
        let mut config_file = File::create(config_file_path.clone())?;
        write!(&mut config_file, "{}", toml::to_string(&Config::default())?)?;
    }
    Ok(config_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    fn create_temp_config_file(content: &str) -> Result<(PathBuf, TempDir)> {
        let dir = tempdir()?;
        let file_path = dir.path().join("temp_config.toml");
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", content)?;
        Ok((file_path, dir))
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.paths, Vec::<String>::new());
    }

    #[test]
    fn test_load_config_success() -> Result<(), ConfigLoadErrors> {
        let content = r#"
            paths = ["path1", "path2", "path3"]
        "#;

        let (file_path, _dir) =
            create_temp_config_file(content).map_err(|_| ConfigLoadErrors::OpenConfigFailed)?;
        let config = load_config(file_path)?;

        assert_eq!(config.paths, vec!["path1", "path2", "path3"]);
        Ok(())
    }

    #[test]
    fn test_load_config_open_config_failed() {
        let non_existent_path = PathBuf::from("non_existent_file.toml");
        let result = load_config(non_existent_path);
        assert_eq!(result, Err(ConfigLoadErrors::OpenConfigFailed));
    }

    #[test]
    fn test_load_config_parse_failed() -> Result<()> {
        let content = r#"
            invalid_key = "value"
        "#;

        let (file_path, _dir) = create_temp_config_file(content)?;
        let result = load_config(file_path);
        assert_eq!(result, Err(ConfigLoadErrors::ParseFailed));
        Ok(())
    }
    // Helper function to set up test environment
    fn setup_test_env() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(APP_NAME);
        create_dir_all(&config_dir).unwrap();
        temp_dir
    }

    // Test when a configuration file already exists
    #[test]
    fn test_setup_config_file_existing() {
        let temp_dir = setup_test_env();

        std::env::set_var("XDG_CONFIG_HOME", temp_dir.path());
        let config_file_path = temp_dir.path().join(APP_NAME).join(CONFIG_FILENAME);
        println!("Config pat{}", config_file_path.to_str().unwrap());

        // Create and write sample content to the existing config file
        let mut config_file = File::create(&config_file_path).unwrap();
        config_file
            .write_all(b"paths = ['path1', 'path2', 'path3']")
            .unwrap();

        let result = setup_config_file();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), config_file_path);

        // Check if the content of the file remains unchanged
        let mut content = String::new();
        let mut file = File::open(config_file_path).unwrap();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "paths = ['path1', 'path2', 'path3']");
        temp_dir.close().unwrap();
    }

    // Test when a configuration file does not exist
    #[test]
    fn test_setup_config_file_non_existing() {
        let dir = setup_test_env();
        std::env::set_var("XDG_CONFIG_HOME", dir.path());
        let result = setup_config_file();
        assert!(result.is_ok());

        let config_file_path = result.unwrap();
        assert!(config_file_path.exists());

        // Check if the content of the new file matches the default config
        let mut content = String::new();
        let mut file = File::open(config_file_path).unwrap();
        file.read_to_string(&mut content).unwrap();
        let parsed_config: Config = toml::from_str(&content).unwrap();
        assert_eq!(parsed_config, Config::default());
        dir.close().unwrap();
    }
}
