use std::fs;
use crate::models::Input;

pub fn parse_text_file(file_name: String) -> Input {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Input {
    let lines: Vec<&str> = content.split_ascii_whitespace().collect();

    let earliest_timestamp : u64 = lines[0].parse().expect("Unable to parse timestamp");
    let bus_schedule: Vec<Option<u64>> = lines[1]
        .split(',')
        .map(|id_string| {
            match id_string {
                "x" => None,
                _ => Some(id_string.parse::<u64>().expect("Unable to parse bus id")),
            }
        })
        .collect();

    Input {
        earliest_timestamp,
        bus_schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "939
7,13,x,x,59,x,31,19
".to_string();

        let input = parse_string(content);

        assert_eq!(input.earliest_timestamp, 939);
        assert_eq!(input.bus_schedule, vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)])
    }
}