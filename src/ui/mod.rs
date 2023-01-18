use std::{io, time::{Duration, Instant}, sync::Arc, fs::DirEntry};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tokio::sync::Mutex;
use tui::{widgets::TableState, Frame, backend::{Backend, CrosstermBackend}, layout::{Layout, Direction, Constraint, Rect}, Terminal};

mod title;
mod status;
mod version;
mod input_event;
mod io_event;
mod table;
mod table_placeholder;

use title::title;

use crate::file_helper::{get_files_path, get_size};

use self::{status::status_block, version::version_block, input_event::{InputEvent, InputEventType}, io_event::IoEventType, table::table, table_placeholder::table_placeholder};

const CHANNEL_BUFFER: usize = 100;

#[derive(PartialEq)]
pub enum DirStatus {
    Loading,
    Ready,
    Deleting,
    Deleted,
    Error,
}

pub struct DirData {
    path: String,
    size: Option<u128>,
    status: DirStatus,
}

impl DirData {
    fn update_size(&mut self, size: u128) {
        self.size = Some(size);
        self.status = DirStatus::Ready;
    }

    fn deleting(&mut self) {
        self.status = DirStatus::Deleting;
    }

    fn deleted(&mut self) {
        self.status = DirStatus::Deleted;
    }

    fn error(&mut self) {
        self.status = DirStatus::Error;
    }
}

struct App {
    data: Option<Vec<DirData>>,
    state: TableState,
    total_size: Option<u128>,
    time_init: Option<Duration>,
    free_space: u128,
    io_tx: tokio::sync::mpsc::Sender<IoEventType>,
}

fn cal_size(io_tx: tokio::sync::mpsc::Sender<IoEventType>, index: usize, path: DirEntry) {
    tokio::spawn(async move {
        let size = get_size(path.path().as_path()).await;
        if let Err(_) = io_tx.send(IoEventType::Loaded(index, size.into())).await {
            return;
        }
    });
}

fn delete_file(io_tx: tokio::sync::mpsc::Sender<IoEventType>, index: usize, path: String) {
    tokio::spawn(async move {
        let result = remove_dir_all::remove_dir_all(path);
        let event = match result {
            Err(_) => IoEventType::DeleteError(index),
            Ok(_) => IoEventType::Deleted(index),
        };
        if let Err(_) = io_tx.send(event).await {
            return;
        }
    });
}

impl App {
    fn new(io_tx: tokio::sync::mpsc::Sender<IoEventType>) -> App {
        App { data: None, state: TableState::default(), total_size: None, time_init: None, free_space: 0, io_tx }
    }

    fn update_data(&mut self, paths: Vec<DirEntry>) {
        if let None = self.data {
            let data: Vec<DirData> = paths.into_iter().enumerate().map(|(index, path)| {
                let file_path = path.path().display().to_string();
                let result = DirData{path: file_path, size: None, status: DirStatus::Loading};
                let io_tx = self.io_tx.clone();
                cal_size(io_tx, index, path);
                return result;
            }).collect();
            self.data = Some(data);
        }
    }

    fn delete_file(&mut self) {
        let index = match self.state.selected() {
            None => return,
            Some(index) => index,
        };

        if let Some(data) = &mut self.data {
            if data[index].status != DirStatus::Ready {
                return;
            }

            data[index].deleting();
            let io_tx = self.io_tx.clone();
            delete_file(io_tx, index, data[index].path.clone());
        }
    }

    fn deleted_file(&mut self, index: usize) {
        if let Some(data) = &mut self.data {
            data[index].deleted();
            if let Some(size) = data[index].size {
                self.free_space += size;
            }
        }
    }

    fn deleted_error(&mut self, index: usize) {
        if let Some(data) = &mut self.data {
            data[index].error();
        }
    }

    fn update_size(&mut self, index: usize, size: u128, instant: Instant) {
        if let Some(data) = &mut self.data {
            data[index].update_size(size);
            let mut added_size = size;
            if let Some(current_size) = self.total_size {
                added_size += current_size;
            }
            self.total_size = Some(added_size);
            match data.iter().find(|item| item.size == None) {
                None => {
                    self.time_init = Some(instant.elapsed());
                },
                Some(_) => {}
            }
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if let Some(data) = &self.data {
                    if i >= data.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if let Some(data) = &self.data {
                    if i == 0 {
                        data.len() - 1
                    } else {
                        i - 1
                    }    
                } else {
                    0
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

struct IoAsyncHandler {
    app: Arc<Mutex<App>>
}

impl IoAsyncHandler {
    pub fn new(app: Arc<Mutex<App>>) -> IoAsyncHandler {
        IoAsyncHandler { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEventType, instant: Instant) {
        let mut app = self.app.lock().await;

        match io_event {
            IoEventType::Initialize => {
                let paths = self.initialize().await;
                app.update_data(paths);
            },
            IoEventType::Loaded(index, size) => {
                app.update_size(index, size, instant);
            },
            IoEventType::Deleted(index) => {
                app.deleted_file(index);
            }
            IoEventType::DeleteError(index) => {
                app.deleted_error(index);
            }
        };
    }
    
    async fn initialize(&self) -> Vec<DirEntry> {
        get_files_path().await
    }
}

pub async fn start_ui() -> Result<(), io::Error> {
    enable_raw_mode().expect("Error");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Error");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let tick_rate = Duration::from_millis(200);
    let events = InputEvent::new(tick_rate);

    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEventType>(CHANNEL_BUFFER);

    // ② Create app

    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);
    let mut is_initialize = false;

    let now = Instant::now();

    // ④ Handle I/O
    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event, now).await;
        }
    });

    loop {
        let mut app = app_ui.lock().await;

        // Render
        match terminal.draw(|rect| drawn(rect, &mut app)) {
            Err(_) => break,
            _ => {}
        }

        if !is_initialize {
           if let Err(_) = sync_io_tx.send(IoEventType::Initialize).await {
               break;
           }
           is_initialize = true;
        }

        let event = match events.next() {
            Err(_) => break,
            Ok(event) => event,
        };

        // ② Handle inputs
        match event {
            InputEventType::Quit => break,
            InputEventType::Up => app.previous(),
            InputEventType::Down => app.next(),
            InputEventType::Select => app.delete_file(),
            InputEventType::Tick => continue,
        }
    }

    // restore terminal
    disable_raw_mode().expect("Error");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("Error");
    terminal.show_cursor()?;

    Ok(())
}

fn drawn<B: Backend>(rect: &mut Frame<B>, app: &mut App) {
    let size = rect.size();

    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(7)].as_ref())
        .split(size);

    let title = title();
    rect.render_widget(title, chunks[0]);

    let version_chunk = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(2), Constraint::Min(2)].as_ref())
    .split(chunks[1]);

    let version = version_block();
    rect.render_widget(version, version_chunk[0]);

    let mid_chunk = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
    .split(version_chunk[1]);

    let status_block = status_block(app.total_size, app.time_init, app.free_space);
    rect.render_widget(status_block, mid_chunk[0]);

    match &app.data {
        Some(data) => {
            let table = table(data);
            rect.render_stateful_widget(table, mid_chunk[1], &mut app.state);
        },
        None => {
            let placeholder = table_placeholder();
            rect.render_widget(placeholder, mid_chunk[1]);
        }
    }
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}