mod file_helper;
mod search_file;

use std::time::Instant;

use byte_unit::Byte;
use search_file::get_files_path;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let node_module_paths = get_files_path().await;

    for entry in node_module_paths {
        // let node_module_size = Byte::from_bytes(file_helper::get_size(&entry.path()).await.into());
        // println!(
        //     "{}: {}",
        //     entry.path().display().to_string(),
        //     node_module_size.get_appropriate_unit(true)
        // )

        println!(
                "{}",
                entry.path().display().to_string(),
            )
    }

    println!("Time to execute: {:?}", start.elapsed())
}
