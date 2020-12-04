use std::{env, fs};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let entries = parse_text_file(args[1].clone());

    println!("part 1: {}", find_result_part_1(&entries));
    println!("part 2: {}", find_result_part_2(&entries));
}

fn parse_text_file(file_name: String) -> Vec<u32> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    return file_content
        .split_ascii_whitespace()
        .map(|s| {
            s.parse::<u32>()
                .expect("An entry cannot be parsed")
        })
        .collect()
}

fn find_result_part_1(entries: &Vec<u32>) -> u32 {
    for index_a in 0..entries.len() {
        for index_b in index_a + 1 .. entries.len() {
            if entries[index_a] + entries[index_b] == 2020 {
                return entries[index_a] * entries[index_b]
            }
        }
    }
    return 0;
}

fn find_result_part_2(entries: &Vec<u32>) -> u32 {
    for index_a in 0..entries.len() {
        for index_b in index_a + 1 .. entries.len() {
            for index_c in index_b + 1 .. entries.len() {
                if entries[index_a] + entries[index_b] + entries[index_c] == 2020 {
                    return entries[index_a] * entries[index_b] * entries[index_c]
                }
            }
        }
    }
    return 0;
}