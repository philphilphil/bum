mod budget;
mod planning;
mod settings;
use crate::dataservice::DataService;
use crate::{dataservice, db};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lazy_static::lazy_static;
use std::{collections::HashMap, io};
use tui::layout::Layout;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::commands;

lazy_static! {
    pub static ref CURRENCY_SYMBOL: String = db::get_setting_currency_symbol().unwrap();
    pub static ref CATEGORY_TOKEN_MAP: HashMap<String, String> =
        DataService::new().get_categorie_map().unwrap();
}

#[derive(Default, PartialEq)]
pub enum UIMode {
    #[default]
    Normal,
    Command,
}

pub struct UserInterface<'a> {
    pub tabs: Vec<&'a str>,
    pub index: usize,
    pub mode: UIMode,
    pub error_message: String,
    command: String,
    pub dataservice: DataService,
}

impl<'a> UserInterface<'a> {
    fn new() -> UserInterface<'a> {
        UserInterface {
            tabs: vec!["Planning", "Budget", "Settings"],
            index: 0,
            mode: UIMode::default(),
            command: String::new(),
            error_message: String::new(),
            dataservice: DataService::new(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tabs.len() - 1;
        }
    }
}

pub fn draw() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create ui and run it
    let ui = UserInterface::new();
    let res = run_ui(&mut terminal, ui);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_ui<B: Backend>(terminal: &mut Terminal<B>, mut app: UserInterface) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app).expect("Error drawing UI"))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                UIMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    KeyCode::Char('p') => app.index = 0,
                    KeyCode::Char('b') => app.index = 1,
                    KeyCode::Char('s') => app.index = 2,
                    KeyCode::Char(':') | KeyCode::Char('c') => {
                        app.mode = UIMode::Command;
                        app.error_message = String::new();
                    }
                    _ => {}
                },
                UIMode::Command => match key.code {
                    KeyCode::Esc => {
                        app.mode = UIMode::Normal;
                        app.command = String::new();
                    }

                    KeyCode::Enter => {
                        app.mode = UIMode::Normal;
                        match commands::handle_command(&app.command) {
                            Ok(_) => {}
                            Err(_) => app.error_message = "Invalid Command".to_string(),
                        };
                        app.command = String::new();
                    }
                    KeyCode::Char(c) => {
                        app.command.push(c);
                    }
                    KeyCode::Backspace => {
                        app.command.pop();
                    }
                    _ => {}
                },
            }
        }
        app.dataservice.load_data()?;
        app.dataservice.calc_overview()?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &UserInterface) -> Result<()> {
    let size = f.size();

    let mut cmd_box_size = 3;
    if app.mode == UIMode::Command {
        cmd_box_size = 4;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(cmd_box_size),
            ]
            .as_ref(),
        )
        .split(size);

    // Bottom - Overview/Command
    match app.mode {
        UIMode::Normal => {
            let bottom: Paragraph = get_overview(app);
            f.render_widget(bottom, chunks[2]);
        }
        UIMode::Command => {
            let input = get_command(app);
            f.set_cursor(chunks[2].x + app.command.len() as u16 + 1, chunks[2].y + 1);
            f.render_widget(input, chunks[2]);
        }
    }

    // Tabs
    let tabs: Tabs = get_tab_menu(app);
    f.render_widget(tabs, chunks[0]);

    // Content
    match app.index {
        0 => planning::render(f, chunks[1], app)?,
        1 => budget::render(f, chunks[1], app)?,
        2 => settings::render(f, chunks[1], app)?,
        _ => {}
    }
    Ok(())
}

fn get_command_help_text(app: &str) -> String {
    // TODO: add help commands and long commands

    if app.starts_with("are") {
        "Add-Recurring-Expense Syntax: <Name> <Category-Token> <Amount> (<Yearly>)".to_string()
    } else if app.starts_with("ari") {
        "Add-Recurring-Income Syntax: <Name> <Category-Token> <Amount>".to_string()
    } else if app.starts_with("ae") {
        "Add-Expense Syntax: <Name> <Category-Token> <Amount>".to_string()
    } else if app.starts_with("ac") {
        "Add-Category Syntax: <Name> <Category-Token>".to_string()
    } else {
        "Commands: add-expense | add-recurring-expense | add-recurring-income | add-categorie"
            .to_string()
    }
}

fn get_tab_menu<'a>(app: &UserInterface<'a>) -> Tabs<'a> {
    let menu = app
        .tabs
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(app.index)
        .block(Block::default().title(" Menu ").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    tabs
}

fn get_overview<'a>(app: &'a UserInterface) -> Paragraph<'a> {
    let mut text = Spans::from(vec![
        Span::styled(
            format!(
                "  Income: {:.2} {}",
                app.dataservice.total_income, *CURRENCY_SYMBOL
            ),
            Style::default().fg(Color::LightGreen),
        ),
        Span::styled(
            format!(
                "  Expenses: {:.2} {}",
                app.dataservice.total_expenses, *CURRENCY_SYMBOL
            ),
            Style::default().fg(Color::LightRed),
        ),
        Span::styled(
            format!(
                "  Budget Spent: {:.2} {}",
                app.dataservice.total_budget_spent, *CURRENCY_SYMBOL
            ),
            Style::default().fg(Color::LightMagenta),
        ),
        Span::styled(
            format!(
                "  Budget left: {:.2} {}",
                app.dataservice.total_budget_left, *CURRENCY_SYMBOL
            ),
            Style::default().fg(Color::Green),
        ),
    ]);

    if !app.error_message.is_empty() {
        text = Spans::from(Span::styled(
            &app.error_message,
            Style::default().fg(Color::Red),
        ));
    }

    let bottom = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(" Overview / Command ")
            .border_type(BorderType::Thick),
    );
    bottom
}

fn get_command<'a>(app: &'a UserInterface) -> Paragraph<'a> {
    let text = vec![
        Spans::from(Span::styled(
            &app.command,
            Style::default().fg(Color::White),
        )),
        Spans::from(Span::styled(
            get_command_help_text(&app.command),
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::LightBlue),
        )),
    ];
    let input = Paragraph::new(text)
        .style(match app.mode {
            UIMode::Normal => Style::default(),
            UIMode::Command => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .title(" Command "),
        );
    input
}
