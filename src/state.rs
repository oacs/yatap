use crossterm::event::KeyCode;
use rust_fuzzy_search::fuzzy_search_best_n;
use std::{collections::HashMap, fs::read_dir};

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
    base_paths: HashMap<String, Vec<String>>,
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

    pub fn all_paths(&self) -> Vec<String> {
        self.base_paths
            .values()
            .flat_map(|dirs| dirs.iter())
            .cloned()
            .collect()
    }

    /// This function searches through the directories and returns a vector of the directories that match the search input. If the search input is empty, it returns all the base paths.
    pub fn search_dirs(&mut self) -> Vec<String> {
        let all_paths = self.all_paths();
        if self.input.is_empty() {
            return all_paths;
        }

        let all_paths: Vec<&str> = all_paths.iter().map(AsRef::as_ref).to_owned().collect();

        let best_matches: Vec<(&str, f32)> = fuzzy_search_best_n(&self.input, &all_paths, 10);

        best_matches
            .into_iter()
            .map(|(matched_str, _score)| matched_str.to_string())
            .collect()
    }

    pub(crate) fn from(config: Config) -> App {
        let mut base_paths: HashMap<String, Vec<String>> = HashMap::new();

        for path in &config.paths {
            if let Ok(entries) = read_dir(path) {
                let dir_list: Vec<String> = entries
                    .filter_map(Result::ok)
                    .filter(|entry| entry.path().is_dir())
                    .map(|entry| entry.path().to_string_lossy().into_owned())
                    .collect();

                base_paths.insert(path.clone(), dir_list);
            }
        }

        let paths: Vec<String> = base_paths
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect();

        App {
            selection_index: 0,
            input: String::new(),
            input_mode: InputMode::Insert,
            paths,
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
            base_paths: HashMap::new(),
            should_close: false,
        }
    }
}
