mod input_parsing;
mod the_game;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::the_game::simulate_game;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_text_file(args[1].clone());

    println!("part 1: {}", simulate_game(&input, 2020));
    println!("part 2: {}", simulate_game(&input, 30000000));
}
