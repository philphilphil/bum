use crate::calculation::{self, CalcResult};
use crate::ui::CURRENCY_SYMBOL;
use crate::{
    db,
    model::{EntryType, RecurringEntry, RecurringType},
};
use crossterm::style::style;
use tui::layout::{Layout, Rect};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
    Frame,
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
    // FIXME: fix bad code
    let calc_items = calculation::calculate_total(&db::get_recurring().unwrap());
    f.render_widget(render_calc_table(calc_items), col1[0]);
    f.render_widget(render_income_table(&rec_i), col1[1]);

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

    // FIXME: bad
    let mut recurring: Vec<RecurringEntry> = db::get_recurring().unwrap();
    recurring.retain(|r| r.kind != EntryType::Income);

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
    let mut expenses = vec![];

    for b in items {
        let mut cells = vec![Cell::from(b.name.to_string())];
        if b.rate_type == RecurringType::Yearly {
            cells.push(Cell::from(format!(
                "{:.2} {}",
                b.amount / 12.0,
                *CURRENCY_SYMBOL
            )));
            cells.push(Cell::from(format!("{:.2} {}", b.amount, *CURRENCY_SYMBOL)));
            cells.push(Cell::from(format!("{}", b.rate_type)));
        } else {
            cells.push(Cell::from(format!("{:.2} {}", b.amount, *CURRENCY_SYMBOL)));
            cells.push(Cell::default());
            cells.push(Cell::from("-".to_string()));
        }

        expenses.push(Row::new(cells));
    }

    expenses.push(Row::new(vec![Cell::default()]));
    expenses.push(Row::new(vec![
        Cell::from(" Sum ").style(Style::default().fg(Color::Cyan)),
        Cell::from(format!("{:.2} {}", sum, *CURRENCY_SYMBOL))
            .style(Style::default().fg(Color::Cyan)),
        Cell::default(),
    ]));

    let t = Table::new(expenses)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Name", "Monthly", "Yearly", "Due"])
                .style(Style::default().fg(Color::Yellow)),
        )
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .column_spacing(0)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", title))
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::LightRed)),
        );
    t
}

fn render_income_table<'a>(items: &Vec<&RecurringEntry>) -> Table<'a> {
    let sum: f32 = items.iter().map(|r| r.amount).sum();
    let mut expenses = vec![];

    for b in items {
        let mut cells = vec![Cell::from(b.name.to_string())];
        cells.push(Cell::from(format!("{:.2} {}", b.amount, *CURRENCY_SYMBOL)));
        expenses.push(Row::new(cells));
    }

    expenses.push(Row::new(vec![Cell::default()]));
    expenses.push(Row::new(vec![
        Cell::from(" Sum ").style(Style::default().fg(Color::Cyan)),
        Cell::from(format!("{:.2} {}", sum, *CURRENCY_SYMBOL))
            .style(Style::default().fg(Color::Cyan)),
    ]));

    let t = Table::new(expenses)
        .style(Style::default().fg(Color::White))
        .header(Row::new(vec!["Name", "Monthly"]).style(Style::default().fg(Color::Yellow)))
        .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)])
        .column_spacing(0)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Income ",))
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::LightGreen)),
        );
    t
}

fn render_calc_table<'a>(items: Vec<CalcResult>) -> Table<'a> {
    let sum: f32 = items.iter().map(|r| r.amount).sum();
    let mut items: Vec<_> = items
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.name.to_string()),
                Cell::from(format!("{:.2} {}", b.amount, *CURRENCY_SYMBOL)),
            ])
        })
        .collect();

    items.push(Row::new(vec![Cell::default()]));
    items.push(Row::new(vec![
        Cell::from(" Budget Left ").style(Style::default().fg(Color::Cyan)),
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
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::LightCyan)),
        );
    t
}
