use crate::models::{Input, Rule, Rules};
use crate::part_1::MatchRule;

pub fn patched_rules_match_rule_0_count(input: &Input) -> u32 {

    let mut patched_input = input.clone();
    patched_input.rules.insert(8, Rule::Or(vec![42], vec![42, 8]));
    patched_input.rules.insert(11, Rule::Or(vec![31], vec![31, 11]));

    patched_input.messages.iter()
        .map(|message| {
            if message.match_custom_pattern(&patched_input.rules) {
                1
            }
            else {
                0
            }
        })
        .sum()
}

pub trait MatchCustomPattern {
    fn match_custom_pattern(&self, rules: &Rules) -> bool;
}

impl MatchCustomPattern for String {
    fn match_custom_pattern(&self, rules: &Rules) -> bool {
        let mut pattern_42_count = 0;
        let mut pattern_31_count = 0;
        let mut current_index = 0;

        // Count pattern 42
        loop {
            let result = self.apply_rule(rules, 42, current_index);

            if result.0 {
                pattern_42_count += 1;
                current_index = result.1;
            }
            else {
                break;
            }
            if result.1 == self.len() {
                break;
            }
        }

        // Count pattern 31
        loop {
            let result = self.apply_rule(rules, 31, current_index);

            if result.0 {
                pattern_31_count += 1;
                current_index = result.1;
            }
            else {
                break;
            }
            if result.1 == self.len() {
                break;
            }
        }

        if current_index != self.len() {
            return false
        }

        eprintln!("42: {}, 31: {}", pattern_42_count, pattern_31_count);

        if pattern_31_count >= 1 && pattern_42_count - pattern_31_count >= 1 {
            true
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_2::{patched_rules_match_rule_0_count, MatchCustomPattern};
    use crate::part_1::MatchRule;

    #[test]
    fn parse_example_case() {
        let content = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#.to_string();

        let input = parse_string(content);

        assert_eq!(patched_rules_match_rule_0_count(&input), 12);
    }
}