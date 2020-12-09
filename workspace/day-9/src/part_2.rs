
pub fn find_weakness_2(serie: &Vec<u64>, weakness_1: u64) -> u64 {

    // Search for each index after the first window
    for current_index in 0..serie.len() {

        let mut i : usize = 0;
        let mut sum = 0;
        let mut smallest = serie[current_index];
        let mut largest = serie[current_index];

        while sum < weakness_1 {
            sum += serie[current_index + i];

            if serie[current_index + i] < smallest {
                smallest = serie[current_index + i];
            }

            if serie[current_index + i] > largest {
                largest = serie[current_index + i];
            }

            if sum == weakness_1 {
                return smallest + largest;
            }

            i += 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn test_part_2() {
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

        let weakness = find_weakness_2(&serie, 5, 127);

        assert_eq!(weakness, 62);
    }
}