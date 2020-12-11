mod input_parsing;
mod sit_layout;
mod seat_simulation;
mod seating_simulation_part_1;
mod seating_simulation_part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::seat_simulation::seating_simulation;
use crate::seating_simulation_part_1::compute_adjacency_map_part_1;
use crate::seating_simulation_part_2::compute_adjacency_map_part_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let input = parse_text_file(args[1].clone());

    println!("part 1: {}", seating_simulation(&input, &compute_adjacency_map_part_1(&input), 4));
    println!("part 2: {}", seating_simulation(&input, &compute_adjacency_map_part_2(&input), 5));
}
