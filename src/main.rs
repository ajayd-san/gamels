use std::collections::HashSet;

use cli::parse;
use colored::*;

use games::{scan_for_games, GameInfo};
use paths::{start_traverse, GameDir};
use size::Size;

mod cli;
mod convert;
mod errors;
mod games;
mod paths;

fn print_info(mut games_info: Vec<GameInfo>) {
    let mut i = 0;
    let mut clients_printed = HashSet::new();

    games_info.sort_by_key(|game| -game.size().bytes());
    while i < games_info.len() {
        let client = games_info.get(i).unwrap().client();
        i += 1;
        if clients_printed.contains(&client) {
            continue;
        }

        let games_info_filtered_by_client = games_info
            .iter()
            .filter(|game| game.client() == client)
            .filter(|game| game.size() > Size::from_mb(10));

        let total_size: i64 = games_info_filtered_by_client
            .clone()
            .map(|game| game.size().bytes())
            .sum();
        let total_size = Size::from_bytes(total_size);

        let output = format!("{}: {}\n{:-<80}", client, total_size, "-");
        println!("{}", output.red());

        for game in games_info_filtered_by_client {
            println!("{}", game);
        }
        clients_printed.insert(client);
    }
}
fn main() {
    let cli = parse();
    let start_point = "/";
    let game_dirs = start_traverse(start_point);

    let mut games_info = Vec::new();

    for game_dir in game_dirs {
        let mut game_info = scan_for_games(game_dir);
        games_info.append(&mut game_info);
        // print_info(gameinfo);
    }
    print_info(games_info);
}
