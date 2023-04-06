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
    /// Current selection on the list
    pub selection_index: usize,

    /// First loaded paths value
    base_paths: Vec<String>,
    pub paths: Vec<String>,

    /// toggle to close the app
    pub should_close: bool,
}

impl App {
    /// Adds a character to the input buffer or removes the previous character if the backspace key is pressed.
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

    /// Increments the selection index to move to the next item in the list of paths, if there is one.
    pub fn select_next_item(&mut self) {
        if self.paths.len() > self.selection_index + 1 {
            self.selection_index += 1;
        }
    }

    /// Decrements the current selection index by one to select the previous item in the list.
    pub fn select_prev_item(&mut self) {
        if self.selection_index > 0 {
            self.selection_index -= 1;
        }
    }

    /// This function searches through the directories and returns a vector of the directories that match the search input. If the search input is empty, it returns all the base paths.
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
