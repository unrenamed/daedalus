use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Table, Cell, Row, BorderType, Paragraph},
    Frame,
};

use crate::app::{widgets::maze_container::MazeContainer, App};
use tui_logger::TuiLoggerWidget;

use super::actions::Actions;

pub fn draw<B: Backend>(rect: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(rect.size());

    // Title block
    let title = draw_title(app);
    rect.render_widget(title, chunks[0]);

    // Body block
    draw_body(rect, app, chunks[1]);
}

fn draw_body<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // Draw tab content
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    if let Some(snapshot) = app.state.get_curr_snapshot() {
        if let Some(title) = app.state.get_running_algorithm_title() {
            let maze_container = MazeContainer::new(snapshot.get_grid(), snapshot.get_highlights())
                .block(Block::default().title(title).borders(Borders::ALL));

            f.render_widget(maze_container, chunks[0]);
        }
    }

    let dashboard_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(20), Constraint::Percentage(100)])
        .split(chunks[1]);

    // Iterate through all elements in the `items` app and append some debug text to it.
    let items: Vec<ListItem> = app
        .state
        .algorithms
        .items
        .iter()
        .map(|i| {
            let lines = vec![Spans::from(i.0)];
            ListItem::new(lines).style(Style::default().fg(Color::White))
        })
        .collect();

    let control_panel_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(30), Constraint::Length(70)].as_ref())
        .split(dashboard_chunks[0]);

    // Create a List from all list items and highlight the currently selected one
    let mut text_color = Color::LightGreen;
    if app.state.is_generator_running {
        text_color = Color::Yellow;
    }
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Algorithm"))
        .highlight_style(Style::default().fg(text_color).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, control_panel_chunks[0], &mut app.state.algorithms.state);

    // Draw the help block
    let help = draw_help(&app.actions);
    f.render_widget(help, control_panel_chunks[1]);

    // Render logs
    let logs = draw_logs();
    f.render_widget(logs, dashboard_chunks[1]);
}

fn draw_title<'a>(app: &'a App) -> Paragraph<'a> {
    Paragraph::new(app.title)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White))
}

fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(20), Constraint::Min(40)])
        .column_spacing(1)
}
