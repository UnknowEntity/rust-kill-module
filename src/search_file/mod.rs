use std::{path::Path, fs::DirEntry, env};

const NODE_MODULE: &'static str = "node_modules";

fn is_match_name_dir(dir: &DirEntry) -> bool {
    let file = dir.file_name();
    let file_name = match file.to_str() {
        Some(name) => name,
        None => return false,
    };

    if !(file_name == NODE_MODULE) {
        return false;
    }

    true
}

fn search_folder(path: &Path) -> Vec<DirEntry> {
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    let child_dir: Vec<DirEntry> = entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir()).collect();
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