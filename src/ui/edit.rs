use anyhow::Result;
use tui::layout::{Layout, Rect};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::db;

use super::UserInterface;

pub fn render<B: Backend>(f: &mut Frame<B>, chunk: Rect, _app: &UserInterface) -> Result<()> {
    let setting_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunk);
    let edit = render_edit_table();
    f.render_widget(edit, setting_chunks[0]);

    Ok(())
}

fn render_edit_table<'a>() -> Table<'a> {
    let items: Vec<_> = db::get_settings()
        .unwrap()
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.key.to_string()),
                Cell::from(b.value.to_string()),
            ])
        })
        .collect();
    let t = Table::new(items)
        .style(Style::default().fg(Color::White))
        .header(Row::new(vec!["Key", "Value"]).style(Style::default().fg(Color::Yellow)))
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)])
        .column_spacing(0)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Edit ")
                .border_type(BorderType::Plain),
        );
    t
}
