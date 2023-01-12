mod file_helper;
mod constants;
mod ui;

use std::{time::Instant, io};

use byte_unit::Byte;
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use file_helper::{get_files_path, get_size};
use tui::{backend::CrosstermBackend, Terminal};
use ui::{App, run_app};

#[tokio::main]
async fn main() {
    // let start = Instant::now();

    

    // for entry in node_module_paths {
    //     let node_module_size = Byte::from_bytes(get_size(&entry.path()).await.into());
    //     println!(
    //         "{}: {}",
    //         entry.path().display().to_string(),
    //         node_module_size.get_appropriate_unit(true)
    //     )
    // }

    // println!("Time to execute: {:?}", start.elapsed())

    // setup terminal
    enable_raw_mode().expect("Fail to enter raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Unable to execute");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Problem initialize tui");

    // create app and run it
    let node_module_paths = get_files_path().await;
    let app = App::new(node_module_paths);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode().expect("Unable to exit raw mode");
    if let Err(err) = res {
        println!("{:?}", err)
    }
    terminal.clear().expect("Unable to clear terminal");
    terminal.show_cursor().expect("Unable to show cursor");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).expect("Unable to execute");
}
