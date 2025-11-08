//! UI rendering for TUI interactive mode

use super::app::App;
use crate::config::schema::CommandSpec;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Main drawing function for the TUI
pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Search input
            Constraint::Min(10),    // Command list
            Constraint::Length(12), // Preview panel
            Constraint::Length(1),  // Help bar
        ])
        .split(f.area());

    // Draw search input
    draw_search_input(f, app, chunks[0]);

    // Draw command list
    draw_command_list(f, app, chunks[1]);

    // Draw preview panel
    draw_preview_panel(f, app, chunks[2]);

    // Draw help bar
    draw_help_bar(f, chunks[3]);
}

/// Draw the search input box
fn draw_search_input(f: &mut Frame, app: &App, area: Rect) {
    let input = Paragraph::new(app.search_input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Search ")
                .style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(input, area);
}

/// Draw the command list with fuzzy-matched results
fn draw_command_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_commands
        .iter()
        .enumerate()
        .map(|(idx, name)| {
            let description = app
                .config
                .commands
                .get(name)
                .map(|cmd| cmd.description.as_str())
                .unwrap_or("");

            let style = if idx == app.selected_index {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let content = format!("{:<20} {}", name, description);
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Commands ({}/{}) ",
                app.filtered_commands.len(),
                app.config.commands.len()
            ))
            .style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(list, area);
}

/// Draw the preview panel showing detailed information about selected command
fn draw_preview_panel(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    if let Some((name, cmd)) = app.selected_command_details() {
        // Command name
        lines.push(Line::from(vec![
            Span::styled(
                "Command: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(name, Style::default().fg(Color::Yellow)),
        ]));

        // Description
        if !cmd.description.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    "Description: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&cmd.description, Style::default().fg(Color::White)),
            ]));
        }

        // Command(s)
        let cmd_text = match &cmd.cmd {
            CommandSpec::Single(c) => c.clone(),
            CommandSpec::Multiple(cmds) => cmds.join(" && "),
            CommandSpec::Platform(_) => "[Platform-specific]".to_string(),
        };
        lines.push(Line::from(vec![
            Span::styled(
                "Cmd: ",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(cmd_text, Style::default().fg(Color::Green)),
        ]));

        // Environment variables
        if !cmd.env.is_empty() {
            let env_vars: Vec<String> = cmd
                .env
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            lines.push(Line::from(vec![
                Span::styled(
                    "Env: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(env_vars.join(", "), Style::default().fg(Color::Magenta)),
            ]));
        }

        // Dependencies
        if !cmd.deps.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    "Deps: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(cmd.deps.join(", "), Style::default().fg(Color::Blue)),
            ]));
        }

        // Tags
        if !cmd.tags.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    "Tags: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(cmd.tags.join(", "), Style::default().fg(Color::Yellow)),
            ]));
        }

        // Statistics from history
        if let Some((count, last_run)) = app.get_command_stats(name) {
            let last_run_str = last_run.unwrap_or("never");
            lines.push(Line::from(vec![
                Span::styled(
                    "Run count: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(count.to_string(), Style::default().fg(Color::Green)),
                Span::styled(
                    " | Last: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(last_run_str, Style::default().fg(Color::Green)),
            ]));
        }

        // Warning for dangerous commands
        if cmd.confirm {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("⚠ ", Style::default().fg(Color::Red)),
                Span::styled(
                    "This command requires confirmation",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled(
            "No command selected",
            Style::default().fg(Color::Gray),
        )));
    }

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Preview ")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

/// Draw the help bar at the bottom
fn draw_help_bar(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::styled(
            "[↑↓/jk] ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Navigate "),
        Span::styled(
            "[Enter] ",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Run "),
        Span::styled(
            "[Ctrl+U] ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("Clear "),
        Span::styled(
            "[Esc/q] ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::raw("Quit"),
    ];

    let paragraph =
        Paragraph::new(Line::from(help_text)).style(Style::default().bg(Color::DarkGray));

    f.render_widget(paragraph, area);
}
