mod virtual_machine;
mod input_parsing;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::execute_until_loop;
use crate::part_2::repair_boot;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let program = parse_text_file(args[1].clone());

    println!("part 1: {}", execute_until_loop(&program));
    println!("part 2: {}", repair_boot(&program));
}
