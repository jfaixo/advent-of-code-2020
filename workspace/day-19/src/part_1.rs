use crate::models::{Input, Rules, Rule};

pub fn match_rule_0_count(input: &Input) -> u32 {
    input.messages.iter()
        .map(|message| {
            if message.match_rule(&input.rules, 0) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub trait MatchRule {
    fn match_rule(&self, rules: &Rules, rule_id: usize) -> bool;
    fn apply_rule(&self, rules: &Rules, rule_id: usize, index: usize) -> (bool, usize);
    fn apply_and(&self, rules: &Rules, rules_ids: &Vec<usize>, index: usize) -> (bool, usize);
}

impl MatchRule for String {
    fn match_rule(&self, rules: &Rules, rule_id: usize) -> bool {
        let result = self.apply_rule(rules, rule_id, 0);
        if result.0 && result.1 == self.len() {
            true
        } else {
            false
        }
    }

    fn apply_rule(&self, rules: &Rules, rule_id: usize, index: usize) -> (bool, usize) {
        match &rules[&rule_id] {
            Rule::Letter(letter) => {
                if index >= self.len() {
                    return (false, index + 1);
                }

                if self.chars().nth(index).unwrap() == *letter {
                    (true, index + 1)
                } else {
                    (false, index + 1)
                }
            }
            Rule::And(rules_ids) => {
                return self.apply_and(rules, rules_ids, index);
            }
            Rule::Or(rules_ids_1, rules_ids_2) => {
                for rules_ids in vec![rules_ids_1, rules_ids_2] {
                    let result = self.apply_and(rules, rules_ids, index);
                    if result.0 {
                        return result;
                    }
                }
                return (false, index + 1);
            }
        }
    }

    fn apply_and(&self, rules: &Rules, rules_ids: &Vec<usize>, index: usize) -> (bool, usize) {
        let mut current_index = index;
        for rule in rules_ids {
            let result = self.apply_rule(rules, *rule, current_index);
            if result.0 {
                current_index = result.1;
            }
            else {
                return (false, current_index + 1);
            }
        }
        (true, current_index)
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_1::{MatchRule, match_rule_0_count};

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

        assert_eq!("ababbb".to_string().match_rule(&input.rules, 0), true);
        assert_eq!("bababa".to_string().match_rule(&input.rules, 0), false);
        assert_eq!("abbbab".to_string().match_rule(&input.rules, 0), true);
        assert_eq!("aaabbb".to_string().match_rule(&input.rules, 0), false);
        assert_eq!("aaaabbb".to_string().match_rule(&input.rules, 0), false);

        assert_eq!(match_rule_0_count(&input), 2);
    }
}