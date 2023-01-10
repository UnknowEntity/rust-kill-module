use std::{path::Path, fs::{DirEntry, self}, env, os::windows::prelude::*};

const NODE_MODULE: &'static str = "node_modules";
const ATTR_HIDDEN: u32 = 0x2;

fn is_match_name_dir(dir: &DirEntry) -> bool {
    let path = dir.path();
    let file_name = match path.to_str() {
        Some(name) => name,
        None => return false,
    };

    if !(file_name.ends_with(NODE_MODULE)) {
        return false;
    }

    true
}

fn search_folder(path: &Path) -> Vec<DirEntry> {
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    let child_dir: Vec<DirEntry> = entries.filter_map(|entry| entry.ok()).filter(|entry| {
        let is_hidden = if cfg!(target_os = "windows") {
            let metadata = match fs::metadata(entry.path()) {
                Ok(metadata) => metadata,
                Err(_) => return false,
            };

            let attributes = metadata.file_attributes();

            (attributes & ATTR_HIDDEN) > 0
        } else {
            false
        };
        entry.path().is_dir() && !is_hidden
    }).collect();

    if let Some(dir_index) = child_dir.iter().position(|dir| is_match_name_dir(dir)) {
        let child = child_dir.into_iter().nth(dir_index).unwrap();
        return vec![child];
    }

    child_dir.iter().map(|dir| search_folder(&dir.path())).into_iter().flatten().collect()
}

pub fn get_files_path() -> Vec<DirEntry> {
    let current_directory = env::current_dir().expect("Cannot get current directory");
    return search_folder(&current_directory)
}