use tui::layout::{Layout, Rect};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
};

use crate::{
    db,
    model::{EntryType, RecurringEntry, RecurringType},
};

pub fn render<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let chunks = Layout::default()
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

    let mut rec_i = db::get_recurring().unwrap();
    rec_i.retain(|r| r.kind == EntryType::Income);

    let mut rec_m = db::get_recurring().unwrap();
    rec_m.retain(|r| r.rate_type == RecurringType::Monthly);

    let mut rec_y = db::get_recurring().unwrap();
    rec_y.retain(|r| r.rate_type == RecurringType::Yearly);

    f.render_widget(render_expense_table(rec_i, "Income".to_string()), chunks[0]);
    f.render_widget(
        render_expense_table(rec_m, "Montlhy".to_string()),
        chunks[1],
    );
    f.render_widget(render_expense_table(rec_y, "Yearly".to_string()), chunks[2]);
}

fn render_expense_table<'a>(items: Vec<RecurringEntry>, title: String) -> Table<'a> {
    let sum: f32 = items.iter().map(|r| r.amount).sum();
    let mut items: Vec<_> = items
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.name.to_string()),
                Cell::from(b.category_token.to_string()),
                Cell::from(format!("{} €", b.amount)),
            ])
        })
        .collect();

    items.push(Row::new(vec![Cell::default()]));
    items.push(Row::new(vec![
        Cell::from(" Sum ").style(Style::default().fg(Color::Cyan)),
        Cell::default(),
        Cell::from(format!("{} €", sum)).style(Style::default().fg(Color::Cyan)),
    ]));

    let t = Table::new(items)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Name", "Category", "Amount"]).style(Style::default().fg(Color::Yellow)),
        )
        .widths(&[
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
                .title(format!(" {} ", title))
                .border_type(BorderType::Plain),
        );
    t
}
