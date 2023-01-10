use std::{path::Path, fs::{DirEntry, self}, env, os::windows::prelude::*};

use tokio::task::JoinSet;

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

fn is_hidden(entry: &DirEntry) -> bool {
    if cfg!(target_os = "windows") {
        let metadata = match fs::metadata(entry.path()) {
            Ok(metadata) => metadata,
            Err(_) => return false,
        };

        let attributes = metadata.file_attributes();

        (attributes & ATTR_HIDDEN) > 0
    } else {
        false
    }
}

fn spawn_search_file(entry: DirEntry, set: &mut JoinSet<Vec<DirEntry>>) {
    set.spawn(async move {
        search_folder(entry.path().as_path()).await
    });
}

async fn search_folder(path: &Path) -> Vec<DirEntry> {
    let entries = match path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return vec![],
    };

    let child_dir: Vec<DirEntry> = entries.filter_map(|entry| entry.ok()).filter(|entry| entry.path().is_dir() && !is_hidden(entry)).collect();

    if let Some(dir_index) = child_dir.iter().position(|dir| is_match_name_dir(dir)) {
        let child = child_dir.into_iter().nth(dir_index).unwrap();
        return vec![child];
    }

    let mut set: JoinSet<Vec<DirEntry>> = JoinSet::new();
    let mut result: Vec<DirEntry> = vec![];

    for entry in child_dir.into_iter() {
        spawn_search_file(entry,&mut set);
    }

    while let Some(data) = set.join_next().await {
        let child_result = data.unwrap_or(Vec::new());
        result.extend(child_result);
    }

    return result;
}

pub async fn get_files_path() -> Vec<DirEntry> {
    let current_directory = env::current_dir().expect("Cannot get current directory");
    return search_folder(&current_directory).await
}