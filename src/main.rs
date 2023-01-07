mod file_helper;
mod search_file;

use std::time::Instant;

use byte_unit::Byte;
use search_file::get_files_path;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    for entry in get_files_path() {
        let node_module_size = Byte::from_bytes(file_helper::get_size(&entry.path()).await.into());
        println!(
            "{}: {}",
            entry.path().display().to_string(),
            node_module_size.get_appropriate_unit(true)
        )
    }

    println!("Time to execute: {:?}", start.elapsed())
}
