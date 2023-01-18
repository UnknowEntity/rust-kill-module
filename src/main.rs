mod file_helper;
mod ui;
use ui::start_ui;

#[tokio::main]
async fn main() {
    start_ui().await.expect("Start UI Error")
}
