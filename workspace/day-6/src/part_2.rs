use crate::customs_declaration::CustomsDeclaration;
use std::collections::HashSet;

pub fn sum_of_common_yes_counts(customs_declarations: &Vec<CustomsDeclaration>) -> u32 {
    customs_declarations
        .iter()
        .map(|customs_declaration| {
            common_yes_count(customs_declaration)
        })
        .sum()
}

fn common_yes_count(customs_declaration: &CustomsDeclaration) -> u32 {
    let yes_sets : Vec<HashSet<char>> = customs_declaration.declarations
        .iter()
        .map(|person_declaration| {
            let mut yes_set : HashSet<char> = HashSet::with_capacity(person_declaration.len());
            for x in person_declaration {
                yes_set.insert(*x);
            }
            yes_set
        }).collect();

    let mut intersected_yes_set = yes_sets[0].clone();

    yes_sets.iter().for_each(|yes_set| {
        intersected_yes_set = intersected_yes_set.intersection(yes_set).cloned().collect()
    });

    intersected_yes_set.len() as u32
}
