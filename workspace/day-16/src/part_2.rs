use crate::models::{Input, TicketNumbers, Field};
use std::collections::HashMap;

// Mapping between a class and the columns that are valid regarding the constraints
type FieldsMapping = HashMap<String, Vec<usize>>;

pub fn get_multiplied_departure_fields(input: &Input) -> u64 {
    // Get valid tickets
    let valid_tickets = get_valid_tickets(input);

    let mapping = get_fields_mapping(&input.fields, &valid_tickets);

    input.my_ticket[mapping["departure location"][0]] as u64
        * input.my_ticket[mapping["departure station"][0]] as u64
        * input.my_ticket[mapping["departure platform"][0]] as u64
        * input.my_ticket[mapping["departure track"][0]] as u64
        * input.my_ticket[mapping["departure date"][0]] as u64
        * input.my_ticket[mapping["departure time"][0]] as u64
}

fn get_fields_mapping(fields: &Vec<Field>, tickets: &Vec<TicketNumbers>) -> FieldsMapping {
    let mut possible_mapping : FieldsMapping = Default::default();
    for field in fields {
        possible_mapping.insert(field.name.clone(), Vec::new());
    }

    // First pass: map the the fields to all columns they can possibly match
    for i in 0..tickets[0].len() {
        for field in fields {
            let mut is_valid = true;

            for ticket in tickets {
                if  (field.first_range[0] <= ticket[i] && ticket[i] <= field.first_range[1]) ||
                    (field.second_range[0] <= ticket[i] && ticket[i] <= field.second_range[1]) {
                    // Ok
                }
                else {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                let mut columns = possible_mapping[&field.name].clone();
                columns.push(i);
                possible_mapping.insert(field.name.clone(), columns);
            }
        }
    }

    // Second pass: find for each field its unique match, by removing fields that already have a unique match in other lists
    let mut completed = false;
    let mut found_fields: Vec<String> = Vec::new();
    while !completed {
        completed = true;
        for field in fields {
            if possible_mapping[&field.name].len() == 1 {
                if !found_fields.contains(&field.name) {
                    eprintln!("field {} has only possible column {}", field.name, possible_mapping[&field.name][0]);
                    found_fields.push(field.name.clone());
                    for inner_field in fields {
                        if inner_field.name != field.name {
                            let mut filtered_columns : Vec<usize> = possible_mapping[&inner_field.name].clone();
                            filtered_columns = filtered_columns.into_iter()
                                .filter(|column| {
                                    *column != possible_mapping[&field.name][0]
                                })
                                .collect();

                            possible_mapping.insert(inner_field.name.clone(), filtered_columns);
                        }
                    }
                }
            }
            else {
                completed = false;
            }
        }
    }

    possible_mapping
}


fn get_valid_tickets(input: &Input) -> Vec<TicketNumbers> {
    let mut valid_tickets: Vec<TicketNumbers> = Vec::new();

    // Hopefully my ticket is valid
    valid_tickets.push(input.my_ticket.clone());

    for ticket in &input.tickets {
        // Check if there is a match
        let mut is_ticket_valid = true;
        for number in ticket {
            let mut is_field_valid = false;
            for field in &input.fields {
                if  (field.first_range[0] <= *number && *number <= field.first_range[1]) ||
                    (field.second_range[0] <= *number && *number <= field.second_range[1]) {
                    // Ok
                    is_field_valid = true;
                    break;
                }
            }
            if !is_field_valid {
                is_ticket_valid = false;
                break;
            }
        }
        if is_ticket_valid {
            valid_tickets.push(ticket.clone());
        }
    }

    valid_tickets
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example_1_test_valid_tickets() {
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

        let valid_tickets = get_valid_tickets(&input);

        eprintln!("{:?}", valid_tickets);

        assert_eq!(valid_tickets[0], vec![7,1,14]);
        assert_eq!(valid_tickets[1], vec![7,3,47]);
    }

    #[test]
    fn example_2() {
        let content = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9".to_string();

        let input = parse_string(content);

        let valid_tickets = get_valid_tickets(&input);
        let mapping = get_fields_mapping(&input.fields, &valid_tickets);

        eprintln!("{:?}", mapping);

        assert_eq!(mapping["row"], vec![0]);
        assert_eq!(mapping["class"], vec![1]);
        assert_eq!(mapping["seat"], vec![2]);
    }
}