use crate::adapter_input::AdapterInput;

pub fn longest_adapters_chain_1_3(input: &AdapterInput) -> u32 {
    // Hypothesis: the input is already good, we just have to count the delta between adapters two by two
    let mut current_joltage = 0;
    let mut gap_1_count = 0;
    let mut gap_3_count = 0;

    let mut serie = input.adapters.clone();
    serie.push(input.device_joltage);

    for i in 0..serie.len() {
        let joltage_delta = serie[i] - current_joltage;
        match joltage_delta {
            1 => gap_1_count += 1,
            3 => gap_3_count += 1,
            _ => {}
        }
        current_joltage += joltage_delta;
    }

    gap_1_count * gap_3_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_1_example_1() {
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
        let input = parse_string(content);
        let result = longest_adapters_chain_1_3(&input);
        assert_eq!(result, 7 * 5);
    }


    #[test]
    fn test_part_1_example_2() {
        let content = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
".to_string();
        let input = parse_string(content);
        let result = longest_adapters_chain_1_3(&input);
        assert_eq!(result, 22 * 10);
    }
}