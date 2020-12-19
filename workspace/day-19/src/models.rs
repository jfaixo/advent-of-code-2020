use std::collections::HashMap;

pub type Rules = HashMap<usize, Rule>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rule {
    Letter(char),
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub rules: Rules,
    pub messages: Vec<String>,
}