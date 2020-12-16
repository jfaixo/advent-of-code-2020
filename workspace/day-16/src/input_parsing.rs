use std::fs;
use crate::models::{Input, Field};

pub fn parse_text_file(file_name: String) -> Input {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Input {
    let mut input: Input = Default::default();

    let parts : Vec<&str> = content.split("\n\n").collect();

    // Parse the classes
    parts[0].split("\n")
        .for_each(|class_string| {
            let class_and_range_string : Vec<&str> = class_string.split(": ").collect();
            let ranges : Vec<u32> = class_and_range_string[1]
                .split(" or ")
                .flat_map(|range_string| range_string.split("-"))
                .map(|number_string| number_string.parse::<u32>().expect("Unable to parse number"))
                .collect();

            input.fields.push(Field {
                name: class_and_range_string[0].to_string(),
                first_range: [ranges[0], ranges[1]],
                second_range: [ranges[2], ranges[3]]
            });
        });

    // Parse my ticket
    input.my_ticket = parts[1]
        .split("\n")
        .nth(1).unwrap()
        .split(",")
        .map(|number_string| number_string.parse::<u32>().expect("Unable to parse number"))
        .collect();

    // Parse all other tickets
    let mut tickets_lines = parts[2].split("\n");
    tickets_lines.next();
    input.tickets = tickets_lines
        .map(|line| {
            line
                .split(",")
                .map(|number_string| number_string.parse::<u32>().expect("Unable to parse number"))
                .collect::<Vec<u32>>()
        })
        .collect();

    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12".to_string();

        let input = parse_string(content);

        assert_eq!(input.fields.len(), 3);
        assert_eq!(input.fields[0], Field { name: "class".to_string(), first_range: [1, 3], second_range: [5, 7] });
        assert_eq!(input.fields[1], Field { name: "row".to_string(), first_range: [6, 11], second_range: [33, 44] });
        assert_eq!(input.fields[2], Field { name: "seat".to_string(), first_range: [13, 40], second_range: [45, 50] });

        assert_eq!(input.my_ticket, vec![7, 1, 14]);

        assert_eq!(input.tickets.len(), 4);
        assert_eq!(input.tickets[0], vec![7, 3, 47]);
        assert_eq!(input.tickets[1], vec![40, 4, 50]);
        assert_eq!(input.tickets[2], vec![55, 2, 20]);
        assert_eq!(input.tickets[3], vec![38, 6, 12]);
    }
}