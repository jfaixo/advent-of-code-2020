mod input_parsing;
mod game;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }

    let game = parse_text_file(args[1].clone());

    println!("part 1: {}", part_1::get_winner_score(&game));
    println!("part 2: {}", part_2::get_winner_score(&game));
}
