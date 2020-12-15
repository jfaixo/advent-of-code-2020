use std::fs;
use crate::the_game::StartingNumbers;

pub fn parse_text_file(file_name: String) -> StartingNumbers {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> StartingNumbers {
    content
        .split(",")
        .map(|x| x.parse::<u32>().expect("Unable to parse starting number"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "0,3,6".to_string();

        let input = parse_string(content);

        assert_eq!(input, vec![0,3,6]);
    }
}