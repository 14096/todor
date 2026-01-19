use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::modules::app::{App, InputMode, PopupField};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, chunks[0]);
    draw_main_content(f, chunks[1], app);
    draw_footer(f, chunks[2], app);

    if app.input_mode == InputMode::Add || app.input_mode == InputMode::Edit {
        draw_add_todo_popup(f, app);
    }
}

fn draw_header(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("TODOR")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(app.split_percentage),
            Constraint::Percentage(100 - app.split_percentage),
        ])
        .split(area);

    let items: Vec<ListItem> = app
        .todo_list
        .todos
        .iter()
        .enumerate()
        .map(|(i, todo)| {
            let status = if todo.completed { "✓" } else { "○" };
            let style = if todo.completed {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::CROSSED_OUT)
            } else {
                Style::default()
            };

            let category_text = if let Some(category) = &todo.category {
                format!("[{}] ", category)
            } else {
                String::new()
            };

            let line = Line::from(vec![
                Span::styled(format!("{} ", status), style),
                Span::styled(category_text, Style::default().fg(Color::Cyan)),
                Span::styled(&todo.title, style),
            ]);

            ListItem::new(line).style(if Some(i) == app.todo_list.selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            })
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut list_state = ListState::default();
    list_state.select(app.todo_list.selected);
    f.render_stateful_widget(list, chunks[0], &mut list_state);

    draw_details(f, chunks[1], app);
}

fn draw_details(f: &mut Frame, area: Rect, app: &App) {
    let details = if let Some(todo) = app.todo_list.get_selected() {
        let status = if todo.completed {
            "Completed ✓"
        } else {
            "Pending ○"
        };

        let mut text = vec![
            Line::from(vec![
                Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&todo.title),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    status,
                    if todo.completed {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Yellow)
                    },
                ),
            ]),
            Line::from(vec![
                Span::styled("Created: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(todo.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            ]),
        ];

        if let Some(description) = &todo.description {
            text.push(Line::from(""));
            text.push(Line::from(vec![Span::styled(
                "Description:",
                Style::default().add_modifier(Modifier::BOLD),
            )]));
            text.push(Line::from(description.as_str()));
        }

        if let Some(category) = &todo.category {
            text.push(Line::from(""));
            text.push(Line::from(vec![
                Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(category, Style::default().fg(Color::Cyan)),
            ]));
        }

        Text::from(text)
    } else {
        Text::from("No todo selected")
    };

    let paragraph = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.input_mode {
        InputMode::Normal => {
            "Controls: ↑/↓/k/j Navigate | Space Toggle | a Add | e Edit | d Delete | [/] Resize | q Quit"
        }
        InputMode::Add | InputMode::Edit => {
            "Tab/Shift+Tab/↑/↓: Navigate fields | Enter: Save | Esc: Cancel"
        }
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}

fn draw_add_todo_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 40, f.area());

    f.render_widget(Clear, area);

    let popup_block = Block::default()
        .title(match app.input_mode {
            InputMode::Add => "Add new",
            InputMode::Edit => "Edit todo",
            _ => "Add new",
        })
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    f.render_widget(popup_block, area);

    let inner_area = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 2,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(2),
        ])
        .split(inner_area);

    let title_style = if matches!(app.todo_form.current_field, PopupField::Title) {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default()
    };

    let title_paragraph = Paragraph::new(app.todo_form.title.as_str())
        .block(Block::default().borders(Borders::ALL).title("Title"))
        .style(title_style);
    f.render_widget(title_paragraph, chunks[0]);

    let category_style = if matches!(app.todo_form.current_field, PopupField::Category) {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default()
    };

    let category_paragraph = Paragraph::new(app.todo_form.category.as_str())
        .block(Block::default().borders(Borders::ALL).title("Category"))
        .style(category_style);
    f.render_widget(category_paragraph, chunks[1]);

    let description_style = if matches!(app.todo_form.current_field, PopupField::Description) {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default()
    };

    let description_paragraph = Paragraph::new(app.todo_form.description.as_str())
        .block(Block::default().borders(Borders::ALL).title("Description"))
        .style(description_style)
        .wrap(Wrap { trim: true });
    f.render_widget(description_paragraph, chunks[2]);

    let instructions =
        Paragraph::new("* Required field | Tab: Next field | Shift+Tab: Previous field")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
    f.render_widget(instructions, chunks[3]);

    let (cursor_x, cursor_y) = match app.todo_form.current_field {
        PopupField::Title => (
            chunks[0].x + app.todo_form.title.len() as u16 + 1,
            chunks[0].y + 1,
        ),
        PopupField::Category => (
            chunks[1].x + app.todo_form.category.len() as u16 + 1,
            chunks[1].y + 1,
        ),
        PopupField::Description => (
            chunks[2].x
                + (app.todo_form.description.len() % (chunks[2].width.saturating_sub(2) as usize))
                    as u16
                + 1,
            chunks[2].y
                + 1
                + (app.todo_form.description.len() / (chunks[2].width.saturating_sub(2) as usize))
                    as u16,
        ),
    };

    f.set_cursor_position((cursor_x, cursor_y));
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
