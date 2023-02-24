use std::{fs::read_dir, path::PathBuf};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{App, InputMode};

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

    app.repos = fetch_paths(app);
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
        InputMode::Editing => (
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
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunk);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
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

fn fetch_paths(app: &mut App) -> Vec<PathBuf> {
    read_dir("/home/oacs/dev")
        .unwrap()
        .filter_map(|m| {
            let path = m.unwrap().path();
            // TODO change this to import string
            if path.to_str().unwrap().contains(&app.input) {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn render_list_paths<B: Backend>(f: &mut Frame<B>, app: &mut App, chunk: Rect) {
    let repos = app
        .repos
        .iter()
        .map(|p| ListItem::new(vec![Spans::from(Span::raw(format!("{}", p.display())))]))
        .collect::<Vec<ListItem>>();

    let messages = List::new(repos).block(Block::default().borders(Borders::ALL).title("Messages"));
    f.render_widget(messages, chunk);
}
