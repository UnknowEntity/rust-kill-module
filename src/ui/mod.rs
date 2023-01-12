use std::{fs::DirEntry, time::Duration, sync::mpsc::{channel, Receiver, Sender, RecvError}, thread};
use crossterm::event::{self, KeyCode};
use tui::{widgets::{TableState, Cell, Row, Table, Block, Borders}, backend::Backend, Terminal, Frame, layout::{Layout, Constraint}, style::{Modifier, Style, Color}};

const DEFAULT_TICK_RATE: u64 = 200;

enum DirStatus {
    Found,
    Deleting,
    Deleted,
}

struct Dir {
    path: DirEntry,
    size: Option<String>,
    status: DirStatus,
}

impl Dir {
    pub fn new(path: DirEntry) -> Dir {
        Dir { path, size: None, status: DirStatus::Found }
    }
}

pub struct App {
    row_data: Vec<Dir>,
    state: TableState 
}

impl App {
    pub fn new(paths: Vec<DirEntry>) -> App {
        let row_data = paths.into_iter().map(|path| Dir::new(path)).collect();
        App { row_data, state: TableState::default() }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.row_data.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.row_data.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub enum InputEventType {
    Quit,
    MoveUp,
    MoveDown,
    Delete,
    Null,
}

fn map_event(key: KeyCode) -> InputEventType {
    match key {
        KeyCode::Up => InputEventType::MoveUp,
        KeyCode::Down => InputEventType::MoveDown,
        KeyCode::Char(' ') => InputEventType::Delete,
        KeyCode::Char('q') => InputEventType::Quit,
        _ => InputEventType::Null,
    }
}

pub struct InputEvents {
    rx: Receiver<InputEventType>,
    tx: Sender<InputEventType>,
}

impl InputEvents {
    pub fn new(tick_rate: Duration) -> InputEvents {
        let (tx, rx) = channel();
        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        match map_event(key.code) {
                            InputEventType::Null => {},
                            event_type @ _ => event_tx.send(event_type).unwrap(),
                        };
                    }
                }
            }
        });

        InputEvents { rx, tx }
    }

    pub fn next(&self) -> Result<InputEventType, RecvError> {
        self.rx.recv()
    }
}

pub enum AsyncEventType {
    FinishDeleting(usize),
    FinishRead(usize),
}

pub struct AsyncEvents {
    rx: Receiver<AsyncEventType>,
    tx: Sender<AsyncEventType>,
}

impl AsyncEvents {
    pub fn new() -> AsyncEvents {
        let (tx, rx) = channel();
        AsyncEvents {rx, tx}
    }

    pub fn next(&self) -> Result<AsyncEventType, RecvError> {
        self.rx.recv()
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), &str> {
    let tick_rate = Duration::from_millis(DEFAULT_TICK_RATE);
    let events = InputEvents::new(tick_rate);

    loop {
        if let Err(_) = terminal.draw(|f| ui(f, &mut app)) {
            return Err("Cannot render terminal");
        }

        let event = match events.next() {
            Ok(event) => event,
            Err(_) => return Err("Channel close unexpectedly"),
        };

        match event {
            InputEventType::MoveUp => app.previous(),
            InputEventType::MoveDown => app.next(),
            InputEventType::Quit => return Ok(()),
            _ => {},
        };
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Path", "Size", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    
    let rows = app.row_data.iter().map(|item| {
        let path = item.path.path().display().to_string();
        let (size, is_ready) = match &item.size {
            Some(size) => (size.clone(), true),
            None => ("..".to_string(), false),
        };
        let status = match item.status {
            DirStatus::Found => {
                if is_ready {
                    "Ready".to_string()  
                } else {
                    "Loading".to_string()
                }
            },
            DirStatus::Deleting => "Deleting".to_string(),
            DirStatus::Deleted => "Deleted".to_string(),
        };
        Row::new(vec![
            Cell::from(path),
            Cell::from(size),
            Cell::from(status)
        ])
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(70),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);
}