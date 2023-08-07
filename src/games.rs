use std::{
    fs::{DirEntry, self},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::paths::{self, GameClient};

#[derive(Debug)]
pub struct GameInfo {
    name: String,
    path: PathBuf,
    client: GameClient,
    size: Option<usize>,
}

impl GameInfo {
    pub fn new<P>(name: String, path: P, client: paths::GameClient) -> Self
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        GameInfo {
            name,
            path,
            client,
            size: None,
        }
    }

    pub fn new_from_pathbuf(path: PathBuf, client: GameClient) -> Self {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let mut size = None;
        if let Ok(metadata) = path.metadata() {
            size = Some(metadata.len() as usize);
        }
        GameInfo {
            name,
            path,
            client,
            size,
        }
    }
}

pub fn scan_for_games(gamedir: paths::GameDir) -> Vec<GameInfo> {
    let game_dirs: Vec<GameInfo> = WalkDir::new(gamedir.path())
        .max_depth(1)
        .sort_by_key(|entry| entry.metadata().unwrap().len())
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| {
            let entry = entry.unwrap();
            GameInfo::new_from_pathbuf(entry.into_path(), *gamedir.client())
        })
        .collect();

    game_dirs
}

#[cfg(test)]
mod tests {
    use crate::paths::GameDir;

    use super::*;

    #[test]
    fn test_steam_scan_for_games() {
        let gamedir = GameDir::new(
            "/mnt/linux_games/SteamLibrary/steamapps/common",
            GameClient::Steam,
        );
        let res = scan_for_games(gamedir);
        dbg!(res);
    }
}
