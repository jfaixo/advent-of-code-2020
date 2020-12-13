use crate::models::Input;

pub fn next_to_leave_part(input: &Input) -> u64 {

    // Find the next bus to leave
    let next_bus_to_leave = input.bus_schedule
        .iter()
        .map(|bus| {
            match bus {
                None => (0 as u64, u64::max_value()),
                Some(bus_id) => {
                    (*bus_id, bus_id - input.earliest_timestamp % bus_id)
                },
            }
        })
        .min_by_key(|x| x.1)
        .expect("Unknown error");

    next_bus_to_leave.0 * next_bus_to_leave.1
}

#[cfg(test)]
mod tests {
    use crate::input_parsing::parse_string;
    use crate::part_1::next_to_leave_part;

    #[test]
    fn test_example() {
        let content = "939
7,13,x,x,59,x,31,19
".to_string();
        let input = parse_string(content);
        let result = next_to_leave_part(&input);
        assert_eq!(result, 295);
    }
}