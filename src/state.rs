use std::fs::read_dir;

use crate::config::Config;

pub enum InputMode {
    Normal,
    Insert,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    pub selection_index: usize,

    base_paths: Vec<String>,
    pub paths: Vec<String>,

    pub should_close: bool,
}

impl App {
    pub fn search_dirs(&self) -> Vec<String> {
        if self.input.is_empty() {
            return self.base_paths.clone();
        }
        self.base_paths
            .iter()
            .filter(|dir| dir.contains(&self.input))
            .cloned()
            .collect()
    }

    pub(crate) fn from(config: Config) -> App {
        let base_paths: Vec<String> = config
            .paths
            .iter()
            .filter_map(|path| read_dir(path).ok())
            .flat_map(|dir| dir.filter_map(Result::ok))
            .filter(|entry| entry.path().is_dir())
            .map(|entry| {
                let path = entry.path();
                path.to_str().unwrap().to_string()
            })
            .collect();
        App {
            selection_index: 0,
            input: String::new(),
            input_mode: InputMode::Insert,
            paths: base_paths.clone(),
            base_paths,
            should_close: false,
        }
    }
}

impl Default for App {
    fn default() -> App {
        App {
            selection_index: 0,
            input: String::new(),
            input_mode: InputMode::Normal,
            paths: Vec::new(),
            base_paths: Vec::new(),
            should_close: false,
        }
    }
}
