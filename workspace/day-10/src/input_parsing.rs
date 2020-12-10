use std::fs;
use crate::adapter_input::AdapterInput;

pub fn parse_text_file(file_name: String) -> AdapterInput {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> AdapterInput {
    let mut adapters : Vec<u32> = content
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("Unable to parse number"))
        .collect();

    adapters.sort();
    let device_joltage = adapters.last().unwrap() + 3;

    AdapterInput {
        adapters,
        device_joltage
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "16
10
15
5
1
11
7
19
6
12
4
".to_string();
        let problem = parse_string(content);

        assert_eq!(problem.adapters, vec![1 , 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]);
        assert_eq!(problem.device_joltage, 22);
    }
}