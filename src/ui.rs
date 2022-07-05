use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::layout::Layout;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, Tabs},
    Frame, Terminal,
};

use crate::{commands, db};

#[derive(Default)]
enum UIMode {
    #[default]
    Normal,
    Command,
}

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub mode: UIMode,
    input: String,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Planning", "Budget", "Settings"],
            index: 0,
            mode: UIMode::default(),
            input: String::new(),
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub fn draw() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                UIMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    KeyCode::Char('p') => app.index = 0,
                    KeyCode::Char('b') => app.index = 1,
                    KeyCode::Char('s') => app.index = 2,
                    KeyCode::Esc => app.mode = UIMode::Normal,
                    KeyCode::Char(':') => app.mode = UIMode::Command,
                    _ => {}
                },
                UIMode::Command => match key.code {
                    KeyCode::Esc => app.mode = UIMode::Normal,
                    KeyCode::Enter => {
                        app.mode = UIMode::Normal;
                        commands::handle_command(&app.input).unwrap();
                        app.input = String::new();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    // bottom
    let budget_overview = Paragraph::new("Budget left: 321,32 €")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(" Overview / Command ")
                .border_type(BorderType::Plain),
        );

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.mode {
            UIMode::Normal => Style::default(),
            UIMode::Command => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title(" Command "));

    match app.mode {
        UIMode::Normal => {
            f.render_widget(budget_overview, chunks[2]);
        }
        UIMode::Command => {
            f.set_cursor(chunks[2].x + app.input.len() as u16 + 1, chunks[2].y + 1);
            f.render_widget(input, chunks[2]);
        }
    }

    // Tabs
    let menu = app
        .titles
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

    f.render_widget(tabs, chunks[0]);

    // content
    match app.index {
        0 => f.render_widget(render_home(), chunks[1]),
        1 => {
            let budget_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            let table = render_budget();
            let table2 = render_budget();

            f.render_widget(table, budget_chunks[1]);
            f.render_widget(table2, budget_chunks[0]);
        }
        _ => {}
    }
}

fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "bum",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("some instructions")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title(" Home ")
            .border_type(BorderType::Plain),
    );
    home
}

fn render_budget<'a>() -> Table<'a> {
    // active
    let items: Vec<_> = db::get_bookings()
        .iter()
        .map(|b| {
            Row::new(vec![
                Cell::from(b.name.to_string()),
                Cell::from(format!("{} €", b.amount)),
                Cell::from("cattbd"),
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
                .title("Entries")
                .border_type(BorderType::Plain),
        );
    t
}
