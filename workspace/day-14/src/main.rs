mod input_parsing;
mod virtual_machine;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::sum_of_all_mem_values_part_1;
use crate::part_2::sum_of_all_mem_values_part_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let program  = parse_text_file(args[1].clone());

    println!("part 1: {}", sum_of_all_mem_values_part_1(&program));
    println!("part 2: {}", sum_of_all_mem_values_part_2(&program));
}
