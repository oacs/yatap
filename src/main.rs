use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::{self, Stdout}, path::PathBuf};

use clap::Parser;
use config::load_config;

use config::Config;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{config::setup_config_file, state::App};

mod config;
mod github;
mod state;
mod ui;
mod tmux;

/// Project launcher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let config_path = args
        .config
        .map_or(setup_config_file(), Ok)
        .unwrap_or_else(|_| {
            println!("Warning: we couldn't load the config file from XDG_HOME_CONFIG");
            PathBuf::new()
        });
    let config = load_config(config_path).unwrap_or_else(|_| {
        println!("We failed to load the config, so we are going to use the default config");
        Config::default()
    });

    if let Some(path) = args.path {
                tmux::attach_or_create_tmux_session(path.into())?;
        return Ok(())
    }

    let mut app = App::from(config);
    app.paths = app.search_dirs();
    let mut terminal = setup_terminal()?;
    while !app.should_close {
        terminal.draw(|f| ui::ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            ui::handle_input(&mut app, key)?;
        }
    }
    
    close_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn close_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
