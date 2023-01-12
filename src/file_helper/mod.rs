use std::{fs::DirEntry, io::Error, path::Path};

use tokio::task::JoinSet;

fn cal_size_spawn(entry: Result<DirEntry, Error>, set: &mut JoinSet<u64>) {
    set.spawn(async move {
        let entry = match entry {
            Ok(data) => data,
            Err(_) => {
                return 0;
            }
        };

        if entry.path().is_dir() {
            return get_size(entry.path().as_path()).await;
        }

        let child_size = match entry.metadata() {
            Ok(data) => data,
            Err(_) => {
                return 0;
            }
        };

        child_size.len()
    });
}

pub async fn get_size(path: &Path) -> u64 {
    let mut size = 0;
    let children = match path.read_dir() {
        Ok(children) => children,
        Err(_) => return 0,
    };

    let mut set = JoinSet::new();

    for entry in children .into_iter(){
        cal_size_spawn(entry, &mut set);
    }

    while let Some(result) = set.join_next().await {
        size += result.unwrap_or(0);
    }

    return size;
}