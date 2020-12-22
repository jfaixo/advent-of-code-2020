use crate::models::Input;
use crate::part_1::find_ingredients_without_allergen;
use std::collections::{HashMap, HashSet};

pub fn find_canonical_dangerous_list(input: &Input) -> String {
    let inert_ingredients = find_ingredients_without_allergen(input);

    let mut remaining_potential_ingredients : HashSet<String> = input.ingredients.iter().cloned()
        .filter(|ingredient| !inert_ingredients.contains(ingredient))
        .collect();
    let mut remaining_allergens_to_map = input.allergens.clone();

    let mut dangerous_stuff : HashMap<String, String> = Default::default();

    while remaining_allergens_to_map.len() > 0 {
        let mut mapped_allergens : Vec<String> = Default::default();
        for allergen in &remaining_allergens_to_map {
            let mut ingredients_intersection = remaining_potential_ingredients.clone();
            for (ingredients, allergens) in &input.foods {
                if allergens.contains(&allergen) {
                    let ingredients : HashSet<String> = ingredients.clone().into_iter().collect();
                    ingredients_intersection = ingredients_intersection.intersection(&ingredients).cloned().collect();
                }
            }

            if ingredients_intersection.len() == 1 {
                // Found a match, let's remove it from the remainings to map
                mapped_allergens.push(allergen.clone());
                remaining_potential_ingredients.remove(ingredients_intersection.iter().next().unwrap());
                dangerous_stuff.insert(allergen.clone(), ingredients_intersection.iter().next().unwrap().clone());
            }
        }

        for allergen in mapped_allergens {
            remaining_allergens_to_map.remove(&allergen);
        }
    }

    eprintln!("{:?}", dangerous_stuff);

    let mut allergens : Vec<String> = input.allergens.clone().into_iter().collect();
    allergens.sort();

    let result = allergens.iter().map(|allergen| {
        dangerous_stuff[allergen].clone()
    })
        .fold("".to_string(), |acc, s| format!("{},{}", acc, s));

    return String::from(&result[1..]);
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_2::find_canonical_dangerous_list;

    #[test]
    fn sample() {
        let content = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#.to_string();
        //endregion

        let input = parse_string(content);

        let result = find_canonical_dangerous_list(&input);
        eprintln!("{}", result);
    }
}