use crate::models::Input;
use std::collections::{HashSet, HashMap};

pub fn find_no_allergen_ingredients_count(input: &Input) -> u32 {
    let remaining_ingredients = find_ingredients_without_allergen(input);

    // Count the number of time each remaining ingredient appears
    let mut count = 0;

    for ingredient in remaining_ingredients {
        for (ingredients, _) in &input.foods {
            if ingredients.contains(&ingredient) {
                count += 1;
            }
        }
    }

    count
}

pub fn find_ingredients_without_allergen(input: &Input) -> HashSet<String> {
    // Initially we suspect all ingredients for all allergens
    let mut potential_ingredients_per_allergen: HashMap<String, HashSet<String>> = Default::default();
    for allergen in &input.allergens {
        potential_ingredients_per_allergen.insert(allergen.clone(), input.ingredients.clone());
    }

    // For each allergen, intersect the list of ingredients that mayt contain it with the whole list
    for allergen in &input.allergens {
        for (ingredients, allergens) in &input.foods {
            if allergens.contains(&allergen) {
                let potentiel_ingredients = potential_ingredients_per_allergen.get_mut(allergen).unwrap();
                let ingredients : HashSet<String> = ingredients.iter().cloned().collect();
                *potentiel_ingredients = potentiel_ingredients.intersection(&ingredients).cloned().collect();
            }
        }
    }

    let mut remaining_ingredients: HashSet<String> = input.ingredients.clone();
    for (_, potential_ingredients) in potential_ingredients_per_allergen {
        remaining_ingredients = remaining_ingredients.difference(&potential_ingredients).cloned().collect();
    }

    remaining_ingredients
}

#[cfg(test)]
mod tests {
    use crate::part_1::find_no_allergen_ingredients_count;
    use crate::input_parsing::parse_string;

    #[test]
    fn sample() {
        let content = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#.to_string();
        //endregion

        let input = parse_string(content);

        let result = find_no_allergen_ingredients_count(&input);

        assert_eq!(result, 5);
    }
}