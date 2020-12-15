use std::collections::HashMap;

/// Typed data input
pub type StartingNumbers = Vec<u32>;

/// Represents the game state at a specific point in time
#[derive(Debug, Eq, PartialEq)]
struct GameState {
    turn_number: u32,
    state_memory: HashMap<u32, NumberInfo>
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct NumberInfo {
    last_spoken_turn: u32
}

/// Default value of game state, with the turn number starting at 1
impl Default for GameState {
    fn default() -> Self {
        GameState {
            turn_number: 1,
            state_memory: Default::default()
        }
    }
}

impl GameState {
    fn say_number(&mut self, last_spoken_number: u32) -> u32 {
        if self.state_memory.contains_key(&last_spoken_number) == false {
            // First time the number has been spoken
            self.state_memory.insert(last_spoken_number, NumberInfo {
                last_spoken_turn: self.turn_number - 1
            });
            0
        }
        else {
            // Number has already been spoken
            let spoken_number = self.turn_number - 1 - self.state_memory[&last_spoken_number].last_spoken_turn;

            let mut new_info = self.state_memory[&last_spoken_number];
            new_info.last_spoken_turn = self.turn_number - 1;
            self.state_memory.insert(last_spoken_number, new_info);
            spoken_number
        }
    }
}

pub fn simulate_game(starting_numbers: &StartingNumbers, turn_count: u32) -> u32 {
    let mut state : GameState = Default::default();

    // First, start by saying the starting numbers
    for i in 0..starting_numbers.len() {
        state.state_memory.insert(starting_numbers[i], NumberInfo {
            last_spoken_turn: state.turn_number
        });
        state.turn_number += 1;
    }

    let mut last_spoken_number : u32 = starting_numbers[starting_numbers.len() - 1];
    for _i in starting_numbers.len()..turn_count as usize {
        last_spoken_number = state.say_number(last_spoken_number);
        state.turn_number += 1;
    }

    last_spoken_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example_1_1() {
        let content = "0,3,6".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 10);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_1_2() {
        let content = "0,3,6".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 436);
    }

    #[test]
    fn example_1_3() {
        let content = "0,3,6".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 30000000);
        assert_eq!(result, 175594);
    }

    #[test]
    fn example_2() {
        let content = "1,3,2".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let content = "2,1,3".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 10);
    }

    #[test]
    fn example_4() {
        let content = "1,2,3".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 27);
    }

    #[test]
    fn example_5() {
        let content = "2,3,1".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 78);
    }

    #[test]
    fn example_6() {
        let content = "3,2,1".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 438);
    }

    #[test]
    fn example_7() {
        let content = "3,1,2".to_string();
        let input  = parse_string(content);
        let result = simulate_game(&input, 2020);
        assert_eq!(result, 1836);
    }
}