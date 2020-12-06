use std::fs;
use crate::customs_declaration::CustomsDeclaration;

pub fn parse_text_file(file_name: String) -> Vec<CustomsDeclaration> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    return file_content
        // Split between groups
        .split("\n\n")
        .map(|group_declarations| {
            let mut customs_declaration : CustomsDeclaration = Default::default();

            // For each group declarations
            group_declarations
                .split_ascii_whitespace()
                .for_each(|person_declaration| {
                    let declaration : Vec<char> = person_declaration.chars().map(|c| c).collect();
                    customs_declaration.declarations.push(declaration);
                });
            customs_declaration
        })
        .collect()
}