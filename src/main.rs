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

fn print_info(mut game_info: Vec<GameInfo>) {
    if game_info.len() > 1 {
        let client = game_info.get(0).unwrap().client();

        game_info.sort_by_key(|game| -game.size().bytes());

        let game_info = game_info
            .iter()
            .filter(|game| game.size() > Size::from_mb(10));

        let total_size: i64 = game_info.clone().map(|game| game.size().bytes()).sum();
        let total_size = Size::from_bytes(total_size);

        let output = format!("{}: {}\n{:-<80}", client, total_size, "-");
        println!("{}", output.red());

        for game in game_info {
            println!("{}", game);
        }
    }
}
fn main() {
    let cli = parse();
    let start_point = "/";
    let game_dirs = start_traverse(start_point);

    for game_dir in game_dirs {
        let gameinfo = scan_for_games(game_dir);
        print_info(gameinfo);
    }
}
