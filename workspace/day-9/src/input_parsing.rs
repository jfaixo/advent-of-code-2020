use std::fs;

pub fn parse_text_file(file_name: String) -> Vec<u64> {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Vec<u64> {
    let parts : Vec<u64> = content
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().expect("Unable to parse number"))
        .collect();
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
".to_string();
        let serie = parse_string(content);

        assert_eq!(serie, vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]);
    }
}