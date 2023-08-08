#![allow(unused)]
use serde_json::Value;
use std::collections::HashSet;
use std::convert::AsRef;
use std::fs;
use std::io;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};
use walkdir::WalkDir;

const STEAM_PATH: &str = "steamapps/common";
const EPIC_PATH: &str = "Epic Games";

const LEGENDARY_CONFIG_PATH: &str = ".config/legendary/installed.json";
const HEROIC_CONFIG_PATH: &str = LEGENDARY_CONFIG_PATH;
// const ALL_CLIENT_PATHS: Vec<&str> = vec![STEAM_PATH, EPIC_PATH];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameClient {
    Steam,
    Epic,
    Heroic,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GameDir {
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
            client,
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
        _ => GameClient::Other,
    }
}

#[cfg(target_os = "linux")]
fn load_config(config_path: &str) -> io::Result<Value> {
    let mut pathbuf = dirs::home_dir().unwrap();
    pathbuf.push(config_path);
    let config = fs::read_to_string(&pathbuf)?;
    let config: Value = serde_json::from_str(&config)?;
    Ok(config)
}

#[cfg(target_os = "windows")]
fn load_config(config_path: &str) {
    fs::read_to_string(config_path);
}
fn load_client_configs() -> Vec<GameDir> {
    let mut gamedirs = Vec::new();

    if let Ok(config) = load_config(HEROIC_CONFIG_PATH) {
        const HEROIC_PATH_KEY: &str = "install_path";
        let mut path_dict = HashSet::new();

        for (_, value) in config.as_object().unwrap() {
            let path = value[HEROIC_PATH_KEY].as_str().unwrap();
            let path = Path::new(&path);
            let install_dir = path.parent().unwrap();
            path_dict.insert(install_dir.to_path_buf());
        }

        for path in path_dict {
            let gamedir = GameDir::new(path, GameClient::Heroic);
            gamedirs.push(gamedir);
        }
    }
    gamedirs
}

pub fn start_traverse<P>(start_point: P) -> Vec<GameDir>
where
    P: AsRef<Path>,
{
    let all_client_paths = vec![STEAM_PATH, EPIC_PATH];

    let mut game_paths = Vec::new();

    game_paths.append(&mut load_client_configs());
    // let start_point = dirs::home_dir().unwrap();

    for entry in WalkDir::new(start_point)
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
    use crate::paths::{GameClient, GameDir};

    use super::start_traverse;

    #[test]
    fn test_paths_exist() {
        let actual = vec![GameDir::new(
            "/mnt/linux_games/SteamLibrary/steamapps/common",
            GameClient::Steam,
        )];
        let res = start_traverse("/");
        println!("{:?}", res);
        // assert_eq!(res, actual);
    }
}
