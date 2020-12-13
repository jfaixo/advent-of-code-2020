use crate::models::Input;

pub fn shuttle_company_contest(input: &Input) -> u64 {

    let mut buses: Vec<(u64, u64)> = Vec::new();
    for i in 0..input.bus_schedule.len() {
        match input.bus_schedule[i] {
            None => {}
            Some(bus_id) => buses.push((i as u64, bus_id)),
        }
    }

    // Iterate with the biggest bus id
    let mut t : u64 = buses[0].1;
    let mut increment : u64 = buses[0].1;
    let mut bus_index : usize = 1;
    loop {
        if bus_index == buses.len() {
            break;
        }

        if (t + buses[bus_index].0) % buses[bus_index].1 == 0 {
            increment *= buses[bus_index].1;
            bus_index += 1;
        }

        t += increment;
    }

    t - increment
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example_1() {
        let content = "42
17,x,13,19
".to_string();

        let input = parse_string(content);
        let result = shuttle_company_contest(&input);
        assert_eq!(result, 3417);
    }

    #[test]
    fn example_2() {
        let content = "42
67,7,59,61
".to_string();

        let input = parse_string(content);
        let result = shuttle_company_contest(&input);
        assert_eq!(result, 754018);
    }

    #[test]
    fn example_3() {
        let content = "42
67,x,7,59,61
".to_string();

        let input = parse_string(content);
        let result = shuttle_company_contest(&input);
        assert_eq!(result, 779210);
    }

    #[test]
    fn example_4() {
        let content = "42
67,7,x,59,61
".to_string();

        let input = parse_string(content);
        let result = shuttle_company_contest(&input);
        assert_eq!(result, 1261476);
    }

    #[test]
    fn example_5() {
        let content = "42
1789,37,47,1889
".to_string();

        let input = parse_string(content);
        let result = shuttle_company_contest(&input);
        assert_eq!(result, 1202161486);
    }
}