use tui::layout::{Layout, Rect};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::db;

pub fn render<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let setting_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(chunk);
    let table = render_settings();
    f.render_widget(table, setting_chunks[0]);
}

fn render_settings<'a>() -> Table<'a> {
    // active
    let items: Vec<_> = db::get_categories()
        .unwrap()
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.token.to_string()),
                Cell::from(b.name.to_string()),
            ])
        })
        .collect();
    let t = Table::new(items)
        .style(Style::default().fg(Color::White))
        .header(Row::new(vec!["Id", "Name"]).style(Style::default().fg(Color::Yellow)))
        .widths(&[Constraint::Length(3), Constraint::Length(10)])
        .column_spacing(5)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Categories ")
                .border_type(BorderType::Plain),
        );
    t
}
