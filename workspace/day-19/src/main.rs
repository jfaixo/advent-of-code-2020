mod input_parsing;
mod models;
mod part_1;
mod part_2;

use std::env;
use std::process::exit;
use crate::input_parsing::parse_text_file;
use crate::part_1::match_rule_0_count;
use crate::part_2::patched_rules_match_rule_0_count;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }

    let input = parse_text_file(args[1].clone());
    println!("part 1: {}", match_rule_0_count(&input));
    println!("part 2: {}", patched_rules_match_rule_0_count(&input));
}
