mod input_parsing;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::find_weakness;
use crate::part_2::find_weakness_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let serie = parse_text_file(args[1].clone());

    let weakness_1 = find_weakness(&serie, 25);
    println!("part 1: {}", weakness_1);
    println!("part 2: {}", find_weakness_2(&serie, weakness_1));
}
