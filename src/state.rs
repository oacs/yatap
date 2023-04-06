use std::fs::read_dir;

use crossterm::event::KeyCode;

use crate::config::Config;

pub enum InputMode {
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
    pub fn add_input_char(&mut self, c: KeyCode) {
        match c {
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            _ => {}
        }
        self.paths = self.search_dirs();
    }

    pub fn select_next_item(&mut self) {
        if self.paths.len() > self.selection_index + 1 {
            self.selection_index += 1;
        }
    }
    pub fn select_prev_item(&mut self) {
        if self.selection_index > 0 {
            self.selection_index -= 1;
        }
    }

    pub fn search_dirs(&mut self) -> Vec<String> {
        self.selection_index = 0;
        if self.input.is_empty() {
            self.base_paths.clone()
        } else {
            self.base_paths
                .iter()
                .filter(|dir| dir.contains(&self.input))
                .cloned()
                .collect()
        }
    }

    pub(crate) fn from(config: Config) -> App {
        let base_paths: Vec<String> = config
            .paths
            .iter()
            .filter_map(|path| read_dir(path).ok())
            .flat_map(|dir| dir.filter_map(Result::ok))
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.path().to_string_lossy().into_owned())
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
            input_mode: InputMode::Insert,
            paths: Vec::new(),
            base_paths: Vec::new(),
            should_close: false,
        }
    }
}
