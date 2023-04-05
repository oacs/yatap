use anyhow::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use unicode_width::UnicodeWidthStr;

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    state::{App, InputMode},
    tmux,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    render_help(f, app, chunks[0]);
    render_list_paths(f, app, chunks[1]);
    render_input(f, app, chunks[2]);
}

fn render_help<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Insert => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunk);
}

fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Insert => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunk);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Insert => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunk.x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunk.y + 1,
            )
        }
    }
}

fn render_list_paths<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let paths: Vec<ListItem> = app
        .paths
        .iter()
        .enumerate()
        .map(|(i, p)| {
            ListItem::new(vec![Spans::from(Span::raw(format!(
                "{} {}",
                if i == app.selection_index { ">>" } else { "  " },
                p.clone()
            )))])
        })
        .collect();

    let messages = List::new(paths).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunk);
}

pub fn handle_input(app: &mut App, key: KeyEvent) -> Result<()> {
    // staless event's handler
    match key.code {
        KeyCode::Char('c') => {
            if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                app.should_close = true;
            }
        }
        KeyCode::Char('z') => {
            if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                app.should_close = true;
            }
        }
        // use arrow keys to navigate
        KeyCode::Up => {
            if app.selection_index > 0 {
                app.selection_index -= 1;
            }
        }
        KeyCode::Down => {
            if app.paths.len().gt(&app.selection_index) {
                app.selection_index += 1;
            }
        }

        _ => {}
    }
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => {
                app.input_mode = InputMode::Insert;
            }
            KeyCode::Char('a') => {
                app.input_mode = InputMode::Insert;
            }
            KeyCode::Char('q') => {
                app.should_close = true;
            }
            _ => {}
        },
        InputMode::Insert => match key.code {
            KeyCode::Enter => {
                let path = &app.paths[app.selection_index];
                tmux::attach_or_create_tmux_session(path.into())?;
                app.should_close = true;
            }
            KeyCode::Char('n') => {
                if key.modifiers.contains(event::KeyModifiers::CONTROL)
                    && app.paths.len() > app.selection_index + 1
                {
                    app.selection_index += 1;
                }
            }
            KeyCode::Char('p') => {
                if key.modifiers.contains(event::KeyModifiers::CONTROL) && app.selection_index > 0 {
                    app.selection_index -= 1;
                }
            }
            KeyCode::Char(c) => {
                app.input.push(c);
                app.selection_index = 0;
                app.paths = app.search_dirs();
            }
            KeyCode::Backspace => {
                app.input.pop();
                app.selection_index = 0;
                app.paths = app.search_dirs();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
    }
    Ok(())
}
