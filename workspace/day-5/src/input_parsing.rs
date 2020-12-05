use std::fs;
use crate::boarding_pass::BoardingPass;

pub fn parse_text_file(file_name: String) -> Vec<BoardingPass> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    return file_content
        // Split between passports
        .split_ascii_whitespace()
        .map(|encoded_boarding_pass| {
            // For each boarding pass
            BoardingPass::from_encoded_boarding_pass(encoded_boarding_pass)
        })
        .collect()
}