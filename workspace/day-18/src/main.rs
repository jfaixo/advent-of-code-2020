mod input_parsing_part_1;
mod model;
mod input_parsing_part_2;

use std::env;
use std::process::exit;
use crate::model::Expression;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let expressions  = input_parsing_part_1::parse_text_file(args[1].clone());
    println!("part 1: {}", sum_all_expressions_results(&expressions));

    let expressions  = input_parsing_part_2::parse_text_file(args[1].clone());
    println!("part 2: {}", sum_all_expressions_results(&expressions));
}

fn sum_all_expressions_results(expressions: &Vec<Expression>) -> i64 {
    expressions.iter()
        .map(|expression| expression.evaluate())
        .sum()
}