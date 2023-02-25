#[macro_use]
extern crate lazy_static;

mod config;
pub mod tmux;
pub mod ui;
use crate::{config::get_config, ui::ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use std::{error::Error, io, path::PathBuf};

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,

    projects_paths: Vec<PathBuf>,

    // Arrow of the selected item
    selection_index: usize,

    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    repos: Vec<PathBuf>,
}

impl App {
    fn new(input: String, input_mode: InputMode, repos: Vec<PathBuf>) -> Self {
        //println!("{}", PROJ_PATHS.config_path.as_path().to_str().unwrap());
        Self {
            input,
            input_mode,
            selection_index: 0,
            repos,
            projects_paths: get_config()
                .unwrap()
                .projects_paths
                .iter()
                .map(|s| PathBuf::from(s))
                .collect(),
        }
    }
}

impl Default for App {
    fn default() -> App {
        App::new(String::new(), InputMode::Editing, Vec::new())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // on_exit hook
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if handle_input(&mut app, key) {
                return Ok(());
            }
        }
    }
}

fn handle_input(app: &mut App, key: KeyEvent) -> bool {
    // staless event's handler
    match key.code {
        KeyCode::Char('c') => {
            if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                return true;
            }
        }
        KeyCode::Char('z') => {
            return key.modifiers.contains(event::KeyModifiers::CONTROL);
        }
        // use arrow keys to navigate
        KeyCode::Up => {
            if app.selection_index > 0 {
                app.selection_index -= 1;
            }
        }
        KeyCode::Down => {
            if app.selection_index < app.repos.len() - 1 {
                app.selection_index += 1;
            }
        }

        _ => {}
    }
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                return true;
            }
            _ => {}
        },
        InputMode::Editing => match key.code {
            KeyCode::Enter => {
                println!("input: {}", app.repos.first().unwrap().display());
                let path = &app.repos[app.selection_index];
                tmux::attach_or_create_tmux_session(path.to_path_buf()).unwrap();
                return true;
            }
            KeyCode::Char(c) => {
                app.input.push(c);
                app.selection_index = 0;
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
    }

    false
}
