use std::collections::{HashSet, HashMap};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Input {
    pub foods: HashMap<Vec<String>, Vec<String>>,
    pub foods_by_allergens: HashMap<Vec<String>, Vec<String>>,

    pub ingredients: HashSet<String>,
    pub allergens: HashSet<String>
}