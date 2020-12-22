use std::fs;
use crate::models::Input;

pub fn parse_text_file(file_name: String) -> Input {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Input {
    let mut input : Input = Default::default();

    content.split("\n")
        .for_each(|line| {
            let ingredients_and_allergens : Vec<&str> = line.split(" (contains ").collect();
            let ingredients : Vec<String> = ingredients_and_allergens[0].split_ascii_whitespace()
                .map(|str| {
                    let ingredient = str.to_string();
                    input.ingredients.insert(ingredient.clone());
                    ingredient
                })
                .collect();
            let allergens : Vec<String> = ingredients_and_allergens[1][..ingredients_and_allergens[1].len() - 1]
                .split(", ")
                .map(|str| {
                    let allergen = str.to_string();
                    input.allergens.insert(allergen.clone());
                    allergen
                })
                .collect();

            input.foods.insert(ingredients.clone(), allergens.clone());
            input.foods_by_allergens.insert(allergens, ingredients);
        });

    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn parse_example_case() {
        let content = r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"#.to_string();
        //endregion

        let input = parse_string(content);

        let allergens : HashSet<String> = ["dairy", "fish", "soy"]
            .iter().cloned().map(|str| str.to_string()).collect();
        assert_eq!(input.allergens, allergens);

        let ingredients : HashSet<String> = ["mxmxvkd", "kfcds", "sqjhc", "nhms", "trh", "fvjkl", "sbzzf"]
            .iter().cloned().map(|str| str.to_string()).collect();
        assert_eq!(input.ingredients, ingredients);

        assert_eq!(input.foods.len(), 4);

        let ingredients : Vec<String> = vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"]
            .iter().cloned().map(|str| str.to_string()).collect();
        let allergens : Vec<String> = vec!["dairy", "fish"]
            .iter().cloned().map(|str| str.to_string()).collect();
        assert_eq!(input.foods[&ingredients], allergens);

        let ingredients : Vec<String> = vec!["sqjhc", "mxmxvkd", "sbzzf"]
            .iter().cloned().map(|str| str.to_string()).collect();
        let allergens : Vec<String> = vec!["fish"]
            .iter().cloned().map(|str| str.to_string()).collect();
        assert_eq!(input.foods[&ingredients], allergens);
    }
}