use std::fs;
use crate::navigation_instructions::{NavigationAction, NavigationInstructions, NavigationInstruction};

pub fn parse_text_file(file_name: String) -> NavigationInstructions {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> NavigationInstructions {
    content
        .split_ascii_whitespace()
        .map(|instruction_string| {
            let action = match instruction_string.chars().next() {
                Some('N') => NavigationAction::North,
                Some('S') => NavigationAction::South,
                Some('E') => NavigationAction::East,
                Some('W') => NavigationAction::West,
                Some('L') => NavigationAction::Left,
                Some('R') => NavigationAction::Right,
                Some('F') => NavigationAction::Forward,
                _ => panic!("Unknown instruction action")
            };

            let value : i32 = instruction_string[1..].parse().expect("Unable to parse value");

            NavigationInstruction {
                action,
                value
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "F10
N3
F7
R90
F11
".to_string();

        let instructions = parse_string(content);

        assert_eq!(instructions.len(), 5);
        assert_eq!(instructions[0], NavigationInstruction { action: NavigationAction::Forward, value: 10 });
        assert_eq!(instructions[1], NavigationInstruction { action: NavigationAction::North, value: 3 });
        assert_eq!(instructions[2], NavigationInstruction { action: NavigationAction::Forward, value: 7 });
        assert_eq!(instructions[3], NavigationInstruction { action: NavigationAction::Right, value: 90 });
        assert_eq!(instructions[4], NavigationInstruction { action: NavigationAction::Forward, value: 11 });

    }
}