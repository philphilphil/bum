mod budget;
mod planning;
mod settings;
use anyhow::Result;
use std::io;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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

#[derive(Default)]
enum UIMode {
    #[default]
    Normal,
    Command,
}

struct UserInterface<'a> {
    pub tabs: Vec<&'a str>,
    pub index: usize,
    pub mode: UIMode,
    pub error_message: String,
    command: String,
}

impl<'a> UserInterface<'a> {
    fn new() -> UserInterface<'a> {
        UserInterface {
            tabs: vec!["Planning", "Budget", "Settings"],
            index: 0,
            mode: UIMode::default(),
            command: String::new(),
            error_message: String::new(),
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

fn run_ui<B: Backend>(terminal: &mut Terminal<B>, mut app: UserInterface) -> io::Result<()> {
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
                    KeyCode::Char(':') => app.mode = UIMode::Command,
                    KeyCode::Char('c') => app.mode = UIMode::Command,
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
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &UserInterface) {
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

    // Bottom - Overview/Command
    match app.mode {
        UIMode::Normal => {
            let bottom: Paragraph = get_bottom(app);
            f.render_widget(bottom, chunks[2]);
        }
        UIMode::Command => {
            let input = Paragraph::new(app.command.as_ref())
                .style(match app.mode {
                    UIMode::Normal => Style::default(),
                    UIMode::Command => Style::default().fg(Color::Yellow),
                })
                .block(Block::default().borders(Borders::ALL).title(" Command "));
            f.set_cursor(chunks[2].x + app.command.len() as u16 + 1, chunks[2].y + 1);
            f.render_widget(input, chunks[2]);
        }
    }

    // Tabs
    let tabs: Tabs = get_tab_menu(app);
    f.render_widget(tabs, chunks[0]);

    // Content
    match app.index {
        0 => planning::render(f, chunks[1]),
        1 => budget::render(f, chunks[1]),
        2 => settings::render(f, chunks[1]),
        _ => {}
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

fn get_bottom<'a>(app: &UserInterface) -> Paragraph<'a> {
    // FIXME: fix bad code
    let mut bottom = Paragraph::new("Budget left: 321,32 ‚Ç¨")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(" Overview / Command ")
                .border_type(BorderType::Plain),
        );

    if !app.error_message.is_empty() {
        bottom = Paragraph::new(app.error_message.clone())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title(" Overview / Command ")
                    .border_type(BorderType::Plain),
            );
    }
    bottom
}
