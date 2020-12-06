mod input_parsing;
mod customs_declaration;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::sum_of_unique_yes_counts;
use crate::part_2::sum_of_common_yes_counts;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let entries = parse_text_file(args[1].clone());

    eprintln!("{:?}", entries);

    println!("part 1: {}", sum_of_unique_yes_counts(&entries));
    println!("part 2: {}", sum_of_common_yes_counts(&entries));
}
