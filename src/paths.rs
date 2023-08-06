use std::path::{Path, PathBuf};

use walkdir::WalkDir;

const STEAM_PATH: &str = "steamapps/common";
const EPIC_PATH: &str = "Epic Games";

// const ALL_CLIENT_PATHS: Vec<&str> = vec![STEAM_PATH, EPIC_PATH];

enum GameClient {
    Steam,
    Epic,
    Other,
}

struct GameDir {
    path: PathBuf,
    client: GameClient,
}

impl GameDir {
    fn new(path: PathBuf, client: GameClient) -> Self {
        GameDir {
            path,
            client
        }
    }
}

pub fn start_traverse<P>(start_point: P)
where
    P: AsRef<Path>,
{
    let all_client_paths: Vec<&str> = vec![STEAM_PATH, EPIC_PATH];

    let mut game_paths = Vec::new();

    for entry in WalkDir::new(start_point)
        .max_depth(5)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        for client in &all_client_paths {
            if entry.path().ends_with(client) {
                // game_paths.push(GameDir::new(entry.path().to_str(), ))
            }

        }
    }
}
