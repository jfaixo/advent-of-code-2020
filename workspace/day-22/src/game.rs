use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Game {
    pub player_1_cards: VecDeque<u32>,
    pub player_2_cards: VecDeque<u32>,
}

impl Game {
    pub fn completed(&self) -> bool {
        self.player_1_cards.len() == 0 || self.player_2_cards.len() == 0
    }
}

pub fn winner_score(cards: &VecDeque<u32>) -> u32 {
    let mut score : u32 = 0;

    for i in 0..cards.len() {
        score += (cards.len() - i) as u32 * cards[i];
    }

    score
}