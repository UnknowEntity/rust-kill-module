mod file_helper;
mod ui;
use ui::start_ui;

#[tokio::main]
async fn main() {
    if let Ok(size) = start_ui().await {
        println!("Free: {}", size);
    } else {
        println!("Error occurs");
    }
}
