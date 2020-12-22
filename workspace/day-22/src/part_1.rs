use crate::game::{Game, winner_score};

pub fn get_winner_score(game: &Game) -> u32 {
    let mut current_game = game.clone();

    while !current_game.completed() {
        simulate_round(&mut current_game);
    }

    if current_game.player_1_cards.len() != 0 {
        winner_score(&current_game.player_1_cards)
    }
    else {
        winner_score(&current_game.player_2_cards)
    }
}

fn simulate_round(game: &mut Game) {
    let player_1_card = game.player_1_cards.pop_front().unwrap();
    let player_2_card = game.player_2_cards.pop_front().unwrap();

    if player_1_card > player_2_card {
        game.player_1_cards.push_back(player_1_card);
        game.player_1_cards.push_back(player_2_card);
    }
    else {
        game.player_2_cards.push_back(player_2_card);
        game.player_2_cards.push_back(player_1_card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

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

        let game = parse_string(content);

        assert_eq!(get_winner_score(&game), 306);
    }
}