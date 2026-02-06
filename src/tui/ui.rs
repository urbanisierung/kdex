use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::{App, AppMode, StatusLevel};
use super::views;

const MIN_WIDTH: u16 = 60;
const MIN_HEIGHT: u16 = 15;

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Check minimum size
    if size.width < MIN_WIDTH || size.height < MIN_HEIGHT {
        render_size_warning(frame, size);
        return;
    }

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
            Constraint::Length(1),  // Status bar
        ])
        .split(size);

    // Render header
    render_header(frame, app, chunks[0]);

    // Render content based on mode
    match app.mode {
        AppMode::Search => views::search::render(frame, app, chunks[1]),
        AppMode::Repos => views::repos::render(frame, app, chunks[1]),
        AppMode::Help => {
            views::search::render(frame, app, chunks[1]);
            views::help::render(frame, chunks[1]);
        }
    }

    // Render status bar
    render_status_bar(frame, app, chunks[2]);
}

fn render_size_warning(frame: &mut Frame, size: Rect) {
    let warning = Paragraph::new(vec![
        Line::from("Terminal too small!"),
        Line::from(""),
        Line::from(format!("Current: {}x{}", size.width, size.height)),
        Line::from(format!("Required: {MIN_WIDTH}x{MIN_HEIGHT}")),
        Line::from(""),
        Line::from("Please resize your terminal."),
    ])
    .style(Style::default().fg(Color::Red))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(warning, size);
}

fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let tabs: Vec<Span> = vec![
        Span::styled(
            " Search ",
            if app.mode == AppMode::Search {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
        Span::raw(" "),
        Span::styled(
            " Repos ",
            if app.mode == AppMode::Repos {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ];

    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("knowledge-index", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
        ]),
        Line::from(tabs),
    ])
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, area);
}

fn render_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let (message, style) = if let Some((ref msg, level)) = app.status_message {
        let color = match level {
            StatusLevel::Info => Color::Blue,
            StatusLevel::Success => Color::Green,
            StatusLevel::Warning => Color::Yellow,
            StatusLevel::Error => Color::Red,
        };
        (msg.clone(), Style::default().fg(color))
    } else {
        let hints = match app.mode {
            AppMode::Search => "Type to search │ ↑↓ navigate │ Enter open │ Tab repos │ ? help │ q quit",
            AppMode::Repos => "↑↓ navigate │ d delete │ r refresh │ Tab search │ ? help │ q quit",
            AppMode::Help => "Press ? or Esc to close",
        };
        (hints.to_string(), Style::default().fg(Color::DarkGray))
    };

    let status = Paragraph::new(message).style(style);
    frame.render_widget(status, area);
}
