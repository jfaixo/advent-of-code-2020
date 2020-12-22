use std::fs;
use crate::game::Game;

pub fn parse_text_file(file_name: String) -> Game {
    let file_content = fs::read_to_string(file_name)
        .expect("Error while reading the data file");

    parse_string(file_content)
}

pub fn parse_string(content: String) -> Game {
    let mut game : Game = Default::default();

    let players : Vec<&str> = content.split("\n\n").collect();

    let player_1_input : Vec<&str>= players[0].split_ascii_whitespace().collect();
    for i in 2..player_1_input.len() {
        game.player_1_cards.push_back(player_1_input[i].parse::<u32>().unwrap());
    }

    let player_2_input : Vec<&str>= players[1].split_ascii_whitespace().collect();
    for i in 2..player_2_input.len() {
        game.player_2_cards.push_back(player_2_input[i].parse::<u32>().unwrap());
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_case() {
        let content = r#"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"#.to_string();

        let input = parse_string(content);

        assert_eq!(input.player_1_cards, vec![9, 2, 6, 3, 1]);
        assert_eq!(input.player_2_cards, vec![5, 8, 4, 7, 10]);
    }
}