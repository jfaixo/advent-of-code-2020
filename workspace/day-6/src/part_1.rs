use crate::customs_declaration::CustomsDeclaration;
use std::collections::HashSet;

pub fn sum_of_unique_yes_counts(customs_declarations: &Vec<CustomsDeclaration>) -> u32 {
    customs_declarations
        .iter()
        .map(|customs_declaration| {
            unique_yes_count(customs_declaration)
        })
        .sum()
}

fn unique_yes_count(customs_declaration: &CustomsDeclaration) -> u32 {
    let mut yes_set : HashSet<char> = HashSet::new();

    for group_declaration in customs_declaration.declarations.iter() {
        for person_declaration in group_declaration.iter() {
            yes_set.insert(*person_declaration);
        }
    }

    yes_set.len() as u32
}