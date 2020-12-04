use std::env;
use std::process::exit;
use day_4::{parse_text_file, valid_number_count_part_1, valid_number_count_part_2};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let entries = parse_text_file(args[1].clone());

    eprintln!("{:?}", entries);

    println!("part 1: {}", valid_number_count_part_1(&entries));
    println!("part 2: {}", valid_number_count_part_2(&entries));
}

