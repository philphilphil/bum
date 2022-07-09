use crate::ui::CURRENCY_SYMBOL;
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
    let budget_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);
    // let table = render_budget();
    let table2 = render_budget();

    // f.render_widget(table, budget_chunks[1]);
    f.render_widget(table2, budget_chunks[0]);
}

fn render_budget<'a>() -> Table<'a> {
    // active
    let items: Vec<_> = db::get_bookings()
        .unwrap()
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.name.to_string()),
                Cell::from(format!("{} {}", b.amount, *CURRENCY_SYMBOL)),
                Cell::from(b.category_token.to_string()),
                Cell::from(b.date.to_string()),
            ])
        })
        .collect();
    let t = Table::new(items)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Name", "Amount", "Category", "Date"])
                .style(Style::default().fg(Color::Yellow)),
        )
        .widths(&[
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .column_spacing(5)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Expenses ")
                .border_type(BorderType::Plain),
        );
    t
}
