mod input_parsing;
mod regulation_rules;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::containing_bags_count;
use crate::part_2::contained_bags_count;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let rules = parse_text_file(args[1].clone());

    println!("part 1: {}", containing_bags_count(&rules, rules.nodes["shiny gold"]));
    println!("part 1: {}", contained_bags_count(&rules, rules.nodes["shiny gold"]));
}
