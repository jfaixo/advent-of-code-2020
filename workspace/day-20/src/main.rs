mod input_parsing;
mod models;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::multiplication_of_corners;
use crate::part_2::rough_water_indicator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }

    let capture = parse_text_file(args[1].clone());
    println!("part 1: {}", multiplication_of_corners(&capture));
    println!("part 2: {}", rough_water_indicator(&capture));
}
