#![allow(unused)]
use std::{path::{Path, PathBuf}, str::FromStr, fmt::Display};
use std::convert::AsRef;
use walkdir::WalkDir;

const STEAM_PATH: &str = "steamapps/common";
const EPIC_PATH: &str = "Epic Games";

// const ALL_CLIENT_PATHS: Vec<&str> = vec![STEAM_PATH, EPIC_PATH];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameClient {
    Steam,
    Epic,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GameDir  {
    path: PathBuf,
    client: GameClient,
}

impl Display for GameClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GameDir {
    pub fn new<P: AsRef<Path>>(path: P, client: GameClient) -> Self {
        GameDir {
            path: path.as_ref().to_path_buf(),
            client
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn client(&self) -> &GameClient {
        &self.client
    }

}

fn get_client_with_loc_str(loc_str: &str) -> GameClient {
    match loc_str {
        STEAM_PATH => GameClient::Steam,
        EPIC_PATH => GameClient::Epic,
        _ => GameClient::Other
    }
}

impl GameClient {
    pub fn get_client_str(&self) -> &'static str {
        match self {
            GameClient::Steam => STEAM_PATH,
            GameClient::Epic => EPIC_PATH,
            GameClient::Other => unreachable!()
        }
    }
}

pub fn start_traverse<P>(start_point: P) -> Vec<GameDir>
where
    P: AsRef<Path>,
{
    let all_client_paths = vec![STEAM_PATH, EPIC_PATH];

    let mut game_paths = Vec::new();

    for entry in WalkDir::new(start_point)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        for client in &all_client_paths {
            if entry.path().ends_with(client) {
                let client = get_client_with_loc_str(client);
                game_paths.push(GameDir::new(entry.path().to_path_buf(), client));
            }

        }
    }

    game_paths
}

#[cfg(test)]
mod tests {
    use crate::paths::{GameDir, GameClient};

    use super::start_traverse;

    #[test]
    fn test_paths_exist() {
        let actual = vec![GameDir::new("/mnt/linux_games/SteamLibrary/steamapps/common", GameClient::Steam)];
        let res = start_traverse("/");
        println!("{:?}", res);
        assert_eq!(res, actual);
    }
}
