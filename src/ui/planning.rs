use std::collections::HashSet;

use serde_json::error::Category;
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

// TODO: Move all the code that gathers and handles data into its own file

pub fn render<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunk);

    // Col 1
    let col1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let rec_i = db::get_recurring().unwrap();
    let rec_i: Vec<&RecurringEntry> = rec_i
        .iter()
        .filter(|c| c.kind == EntryType::Income)
        .collect();

    // rec_i.retain(|r| r.kind == EntryType::Income);

    f.render_widget(
        render_expense_table(&rec_i, " Calculation ".to_string()),
        col1[0],
    );
    f.render_widget(
        render_expense_table(&rec_i, " Income ".to_string()),
        col1[1],
    );

    // group by categorie, render one box for each category
    //  Col 2
    let col2 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let col3 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let mut rec_m = db::get_recurring().unwrap();
    rec_m.retain(|r| r.rate_type == RecurringType::Monthly);

    let mut rec_y = db::get_recurring().unwrap();
    rec_y.retain(|r| r.rate_type == RecurringType::Yearly);

    let recurring: Vec<RecurringEntry> = db::get_recurring().unwrap();

    let mut categories: Vec<String> = recurring
        .iter()
        .map(|c| c.category_token.to_string())
        .collect();
    categories.sort();
    categories.dedup();

    let mut widget_col = 0;
    let mut widget_row = 0;
    for cat in categories {
        let rec: Vec<&RecurringEntry> = recurring
            .iter()
            .filter(|c| c.category_token == cat)
            .collect();

        match widget_col {
            0 => {
                f.render_widget(render_expense_table(&rec, cat.clone()), col2[widget_row]);
            }
            1 => {
                f.render_widget(render_expense_table(&rec, cat.clone()), col3[widget_row]);
            }
            _ => panic!("Invalid col"),
        }

        if widget_col == 0 {
            widget_col = 1;
        } else {
            widget_col = 0;
            widget_row += 1;
        }
    }
}

fn render_expense_table<'a>(items: &Vec<&RecurringEntry>, title: String) -> Table<'a> {
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

fn render_calc_table<'a>(items: Vec<RecurringEntry>) -> Table<'a> {
    let sum: f32 = items.iter().map(|r| r.amount).sum();
    let mut items: Vec<_> = items
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.name.to_string()),
                Cell::from(format!("{} ‚Ç¨", b.amount)),
            ])
        })
        .collect();

    items.push(Row::new(vec![Cell::default()]));
    items.push(Row::new(vec![
        Cell::from(" Budget Left ").style(Style::default().fg(Color::Cyan)),
        Cell::default(),
        Cell::from(format!("{} €", sum)).style(Style::default().fg(Color::Cyan)),
    ]));

    let t = Table::new(items)
        .style(Style::default().fg(Color::White))
        .header(Row::new(vec!["Name", "Amount"]).style(Style::default().fg(Color::Yellow)))
        .widths(&[Constraint::Length(10), Constraint::Length(10)])
        .column_spacing(5)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Calculation ")
                .border_type(BorderType::Plain),
        );
    t
}
