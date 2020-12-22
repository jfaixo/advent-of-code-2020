use crate::game::{Game, winner_score};

pub fn get_winner_score(game: &Game) -> u32 {

    let (winner, game) = simulate_game(game, 0);

    if winner == 0 {
        winner_score(&game.player_1_cards)
    }
    else {
        winner_score(&game.player_2_cards)
    }
}

fn simulate_game(game: &Game, depth: u32) -> (u32, Game) {
    let mut current_game = game.clone();
    let mut previous_configurations : Vec<Game> = Default::default();

    while !current_game.completed() {
        // If we are in a previously found configuration, stop
        if previous_configurations.contains(&current_game) {
            return (0, current_game);
        }
        previous_configurations.push(current_game.clone());

        // Draw cards
        let player_1_card = current_game.player_1_cards.pop_front().unwrap();
        let player_2_card = current_game.player_2_cards.pop_front().unwrap();

        // Test if we are in a sub game configuration
        let winner = if player_1_card <= current_game.player_1_cards.len() as u32 && player_2_card <= current_game.player_2_cards.len() as u32 {
            // Create the decks for the sub game
            let mut sub_game = current_game.clone();
            sub_game.player_1_cards.drain(player_1_card as usize..);
            sub_game.player_2_cards.drain(player_2_card as usize..);

            let (winner, _) = simulate_game(&sub_game, depth + 1);
            winner
        }
        else {
            if player_1_card > player_2_card {
                0
            }
            else {
                1
            }
        };

        if winner == 0 {
            current_game.player_1_cards.push_back(player_1_card);
            current_game.player_1_cards.push_back(player_2_card);
        }
        else {
            current_game.player_2_cards.push_back(player_2_card);
            current_game.player_2_cards.push_back(player_1_card);
        }
    }

    if current_game.player_1_cards.len() != 0 {
        (0, current_game)
    }
    else {
        (1, current_game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_parsing::parse_string;

    #[test]
    fn example() {
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

        assert_eq!(get_winner_score(&game), 291);
    }

    #[test]
    fn example_infinite() {
        let content = r#"Player 1:
43
19

Player 2:
2
29
14"#.to_string();

        let game = parse_string(content);
        get_winner_score(&game);
    }
}
