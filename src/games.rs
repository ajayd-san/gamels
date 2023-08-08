use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf}, fmt::Display
};

use size::Size;
use walkdir::WalkDir;
use colored::*;

use crate::paths::{self, GameClient};

#[derive(Debug)]
pub struct GameInfo {
    name: String,
    path: PathBuf,
    client: GameClient,
    size: Size,
}

impl Display for GameInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}\n", "Name".magenta(), self.name);
        write!(f, "{}: {}\n", "Path".magenta(), self.path.as_path().to_string_lossy());
        write!(f, "{}: {}\n", "Size".magenta(), self.size)
    }
}

impl GameInfo {
    pub fn new_from_pathbuf(path: PathBuf, client: GameClient) -> Self {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let size: u64 = WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|entry| entry.metadata().unwrap().len())
            .sum();

        let size = Size::from_bytes(size);

        GameInfo {
            name,
            path,
            client,
            size,
        }
    }

    pub fn client(&self) -> GameClient {
        self.client
    }

    pub fn size(&self) -> Size {
        self.size
    }

}

pub fn scan_for_games(gamedir: paths::GameDir) -> Vec<GameInfo> {
    let game_dirs: Vec<GameInfo> = WalkDir::new(gamedir.path())
        .max_depth(1)
        .sort_by_key(|entry| entry.metadata().unwrap().len())
        .into_iter()
        .filter(|entry| entry.is_ok())
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            if metadata.is_file() || metadata.is_symlink() {
                return None;
            }

            if entry.path() == gamedir.path() {
                return None;
            }

            Some(GameInfo::new_from_pathbuf(
                entry.into_path(),
                *gamedir.client(),
            ))
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
