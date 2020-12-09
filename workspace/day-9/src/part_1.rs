
pub fn find_weakness(serie: &Vec<u64>, window_size: usize) -> u64 {

    // Search for each index after the first window
    for current_index in window_size..serie.len() {

        // Try all permutations in order to find two numbers that can sum to the target number
        let window_start = current_index - window_size;
        let window_end = current_index;

        let mut found = false;
        'search:
        for index_a in window_start..window_end - 1 {
            for index_b in window_start + 1..window_end {
                // Test for the sum
                if serie[index_a] + serie[index_b] == serie[current_index] {
                    found = true;
                    break 'search;
                }
            }
        }

        if !found {
            return serie[current_index];
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_1() {
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

        let weakness = find_weakness(&serie, 5);

        assert_eq!(weakness, 127);
    }
}