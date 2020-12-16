mod input_parsing;
mod models;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::ticket_scanning_error_rate;
use crate::part_2::get_multiplied_departure_fields;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_text_file(args[1].clone());

    println!("part 1: {}", ticket_scanning_error_rate(&input));
    println!("part 2: {}", get_multiplied_departure_fields(&input));
}
