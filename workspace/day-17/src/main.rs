mod input_parsing_part_1;
mod part_1;
mod part_2;
mod input_parsing_part_2;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let mut context = input_parsing_part_1::parse_text_file(args[1].clone());
    println!("part 1: {}", part_1::active_count_after_6_steps(&mut context));

    let mut context = input_parsing_part_2::parse_text_file(args[1].clone());
    println!("part 2: {}", part_2::active_count_after_6_steps(&mut context));
}
