use std::path::PathBuf;

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
        InputMode::Insert => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to attach or create tmux session"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    f.render_widget(Paragraph::new(text), chunk);
}

fn render_input<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Insert => Style::default().fg(Color::Magenta),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunk);
    match app.input_mode {
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
    match key.code {
        KeyCode::Up => {
            app.select_prev_item();
        }
        KeyCode::Down => {
            app.select_next_item();
        }
        _ => match app.input_mode {
            InputMode::Insert => match key.code {
                KeyCode::Enter => {
                    let path = &app.paths[app.selection_index];
                    let path: PathBuf = path.into();
                    tmux::attach_or_create_tmux_session(path)?;
                    app.should_close = true;
                }
                KeyCode::Backspace => app.add_input_char(crossterm::event::KeyCode::Backspace),
                KeyCode::Char(c) => {
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                        match c {
                            'c' | 'z' => app.should_close = true,
                            'p' => app.select_prev_item(),
                            'n' => app.select_next_item(),
                            _ => (),
                        }
                    }
                    app.add_input_char(crossterm::event::KeyCode::Char(c));
                }
                KeyCode::Esc => {
                    app.should_close = true;
                }
                _ => {}
            },
        },
    }
    Ok(())
}
