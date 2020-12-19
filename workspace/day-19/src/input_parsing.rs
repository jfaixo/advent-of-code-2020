use std::fs;
use crate::models::{Input, Rule};
use std::collections::HashMap;

pub fn parse_text_file(file_name: String) -> Input {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Input {
    let rules_and_messages : Vec<&str> = content.split("\n\n").collect();

    // Parse rules
    let mut rules = HashMap::new();
    rules_and_messages[0]
        .split("\n")
        .for_each(|line| {
            let id_and_rule :  Vec<&str> = line.split(": ").collect();
            let id = id_and_rule[0].parse::<usize>().unwrap();

            match id_and_rule[1] {
                rule if rule.starts_with("\"") => {
                    // Letter rule
                    rules.insert(id, Rule::Letter(rule.chars().nth(1).unwrap()));
                }
                rule  if rule.contains("|") => {
                    // Or rule
                    let ids : Vec<Vec<usize>> = rule.split("|")
                        .map(|rule_and_string| {
                            let and_ids : Vec<usize> = rule_and_string.split_ascii_whitespace().
                                map(|id| id.parse::<usize>().unwrap())
                                .collect();
                            and_ids
                        })
                        .collect();
                    rules.insert(id, Rule::Or(ids[0].clone(), ids[1].clone()));
                }
                rule => {
                    // And rule
                    let ids : Vec<usize> = rule.split_ascii_whitespace()
                        .map(|rule_id_string| rule_id_string.parse::<usize>().unwrap())
                        .collect();
                    rules.insert(id, Rule::And(ids));
                }
            }

        });


    // Parse messages
    let messages  = rules_and_messages[1]
        .split_ascii_whitespace()
        .map(|str| str.to_string())
        .collect();

    Input {
        rules,
        messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#.to_string();

        let input = parse_string(content);

        assert_eq!(input.rules.len(), 6);
        assert_eq!(input.rules[&0], Rule::And(vec![4, 1, 5]));
        assert_eq!(input.rules[&1], Rule::Or(vec![2, 3], vec![3, 2]));
        assert_eq!(input.rules[&2], Rule::Or(vec![4, 4], vec![5, 5]));
        assert_eq!(input.rules[&3], Rule::Or(vec![4, 5], vec![5, 4]));
        assert_eq!(input.rules[&4], Rule::Letter('a'));
        assert_eq!(input.rules[&5], Rule::Letter('b'));

        assert_eq!(input.messages.len(), 5);
        assert_eq!(input.messages[0], "ababbb");
        assert_eq!(input.messages[1], "bababa");
        assert_eq!(input.messages[2], "abbbab");
        assert_eq!(input.messages[3], "aaabbb");
        assert_eq!(input.messages[4], "aaaabbb");
    }
}