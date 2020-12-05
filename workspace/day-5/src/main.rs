mod input_parsing;
mod boarding_pass;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::highest_seat_id;
use crate::part_2::my_seat_id;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let mut entries = parse_text_file(args[1].clone());

    eprintln!("{:?}", entries);

    println!("part 1: {}", highest_seat_id(&entries));
    println!("part 2: {}", my_seat_id(&mut entries));
}
