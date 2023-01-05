use std::{fs::DirEntry, path::Path, env, time::Instant};

use byte_unit::Byte;

fn get_file_name(entry: &DirEntry) -> Option<String> {
    match entry.file_name().to_str() {
        Some(string) => Some(string.to_string()),
        None => None,
    }
}

fn get_size(path: &Path) -> u64 {
    let mut size = 0;
    let children = match path.read_dir() {
        Ok(children) => children,
        Err(_) => return 0,
    };

    for entry in children {
        let entry = match entry {
            Ok(data) => data,
            Err(_) => {
                continue;
            }
        };

        if entry.path().is_dir() {
            size += get_size(&entry.path());
        }

        let child_size = match entry.metadata() {
            Ok(data) => data,
            Err(_) => {
                continue;
            }
        };

        size += child_size.len()
    }

    return size;
}

fn main() {
    let start = Instant::now();
    let current_directory = env::current_dir().expect("Cannot get current directory");

    for entry in current_directory.as_path().read_dir().expect("Cannot read directory") {
        if let Ok(child_entry) = entry {
            let file_name = match get_file_name(&child_entry) {
                Some(name) => name,
                None => continue,
            };

            if file_name == "node_modules" {
                let node_module_size = Byte::from_bytes(get_size(&child_entry.path()).into());
                println!(
                    "{file_name}: {}",
                    node_module_size.get_appropriate_unit(true)
                )
            }
        }
    }

    println!("Time to execute: {:?}", start.elapsed())
}
