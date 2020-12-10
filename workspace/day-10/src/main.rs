mod input_parsing;
mod adapter_input;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::longest_adapters_chain_1_3;
use crate::part_2::count_valid_adapters_chain_possibilities;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input = parse_text_file(args[1].clone());
    println!("part 1: {}", longest_adapters_chain_1_3(&input));
    println!("part 2: {}", count_valid_adapters_chain_possibilities(&input));
}
