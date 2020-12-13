mod input_parsing;
mod models;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::next_to_leave_part;
use crate::part_2::shuttle_company_contest;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input  = parse_text_file(args[1].clone());

    println!("part 1: {}", next_to_leave_part(&input));
    println!("part 2: {}", shuttle_company_contest(&input));
}
