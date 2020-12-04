use std::{env, fs};
use std::process::exit;

#[derive(Debug)]
struct PasswordEntry {
    first_value: u32,
    second_value: u32,
    letter: char,
    password: String
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Invalid arguments count");
        exit(1);
    }
    let entries = parse_text_file(args[1].clone());

    eprintln!("{:?}", entries);

    println!("part 1: {}", valid_password_count_part_1(&entries));
    println!("part 2: {}", valid_password_count_part_2(&entries));
}

fn parse_text_file(file_name: String) -> Vec<PasswordEntry> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    let raw_entries : Vec<&str> = file_content
        .split_ascii_whitespace()
        .collect();

    return raw_entries
        .chunks(3)
        .map(|tuple| {
            let frequencies : Vec<u32>= tuple[0]
                .split("-")
                .map(|x| {
                    String::from(x).parse::<u32>()
                        .expect("Invalid frequency parsing")
                })
                .collect();
            PasswordEntry {
                first_value: frequencies[0],
                second_value: frequencies[1],
                letter: tuple[1].chars().next().expect("Unable to get the password character"),
                password: String::from(tuple[2])
            }
        })
        .collect()
}

fn valid_password_count_part_1(password_entries: &Vec<PasswordEntry>) -> u32 {
    let mut valid_count : u32 = 0;

    for entry in password_entries {
        let letter_frequency : u32 = entry.password.chars()
            .map(|c| {
                if c == entry.letter {
                    1
                }
                else {
                    0
                }
            }).sum();

        if entry.first_value <= letter_frequency && letter_frequency <= entry.second_value {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn valid_password_count_part_2(password_entries: &Vec<PasswordEntry>) -> u32 {
    let mut valid_count : u32 = 0;

    for entry in password_entries {
        let mut letter_is_at_index_count = 0;
        if entry.password.chars().nth((entry.first_value - 1) as usize).unwrap() == entry.letter {
            letter_is_at_index_count += 1;
        }
        if entry.password.chars().nth((entry.second_value - 1) as usize).unwrap() == entry.letter {
            letter_is_at_index_count += 1;
        }

        if letter_is_at_index_count == 1 {
            valid_count += 1;
        }
    }

    return valid_count;
}