use crate::models::Input;

pub fn ticket_scanning_error_rate(input: &Input) -> u32 {
    let mut error_rate : u32 = 0;

    for ticket in &input.tickets {
        for number in ticket {
            // Check if there is a match
            let mut is_valid = false;
            for field in &input.fields {
                if  (field.first_range[0] <= *number && *number <= field.first_range[1]) ||
                    (field.second_range[0] <= *number && *number <= field.second_range[1]) {
                    // Ok
                    is_valid = true;
                    break;
                }
            }

            if !is_valid {
                error_rate += number;
            }
        }
    }

    error_rate
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example() {
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
        let error_rate = ticket_scanning_error_rate(&input);
        assert_eq!(error_rate, 71);
    }
}