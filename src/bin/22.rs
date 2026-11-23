use std::collections::VecDeque;
use std::hash::{DefaultHasher, Hash, Hasher};

use hashbrown::HashSet;

advent_of_code::solution!(22);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deck {
    pub player1: VecDeque<u8>,
    pub player2: VecDeque<u8>,
}

impl Deck {
    pub fn new(input: &str) -> Self {
        let (player1, player2) = input
            .split_once("\n\n")
            .expect("invalid input, no double newline");
        let (_, player1) = player1.split_once(":\n").expect("Missing player title");
        let (_, player2) = player2.split_once(":\n").expect("missing player title");

        let player1 = player1
            .lines()
            .map(|l| str::parse::<u8>(l.trim()).expect("invalid input"))
            .collect();
        let player2 = player2
            .lines()
            .map(|l| str::parse::<u8>(l.trim()).expect("invalid input"))
            .collect();

        Self { player1, player2 }
    }

    pub fn process_round(&mut self) -> bool {
        if self.player1.is_empty() || self.player2.is_empty() {
            return false;
        }
        let p1 = self.player1.pop_front().unwrap();
        let p2 = self.player2.pop_front().unwrap();

        if p1 > p2 {
            self.player1.push_back(p1);
            self.player1.push_back(p2);
        } else {
            self.player2.push_back(p2);
            self.player2.push_back(p1);
        }

        true
    }

    pub fn process_game(&mut self) {
        while self.process_round() {}
    }

    pub fn score(&self) -> Option<usize> {
        let mut winning_hand = self.player1.clone();
        if winning_hand.is_empty() {
            winning_hand = self.player2.clone();
        }

        Some(
            winning_hand
                .iter()
                .rev()
                .enumerate()
                .map(|(i, v)| *v as usize * (i + 1))
                .sum(),
        )
    }

    pub fn draw(&mut self) -> Option<(u8, u8)> {
        if self.player1.is_empty() || self.player2.is_empty() {
            return None;
        }
        Some((
            self.player1.pop_front().unwrap(),
            self.player2.pop_front().unwrap(),
        ))
    }
}

/// Game struct for round 2, since we need to track separate game states recursively
pub struct Game {
    pub tracked_states: HashSet<u64>,
    pub deck: Deck,
}

pub enum Player {
    One,
    Two,
}

fn calculate_hash<T: Hash>(item: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    item.hash(&mut hasher);
    hasher.finish()
}

use Player::*;

impl Game {
    pub fn new(deck: Deck) -> Self {
        Self {
            tracked_states: HashSet::new(),
            deck,
        }
    }

    pub fn round(&mut self) -> Option<Player> {
        if !self.tracked_states.insert(calculate_hash(&self.deck)) {
            return Some(One);
        }
        let cards = self.deck.draw();
        if cards.is_none() {
            match (self.deck.player1.is_empty(), self.deck.player2.is_empty()) {
                (true, false) => {
                    return Some(Two);
                }
                (false, true) => {
                    return Some(One);
                }
                _ => {}
            }
        }
        let (p1, p2) = cards.unwrap();

        let result =
            if p1 as usize <= self.deck.player1.len() && p2 as usize <= self.deck.player2.len() {
                let player1 = VecDeque::from_iter(self.deck.player1.clone().drain(..(p1 as usize)));
                let player2 = VecDeque::from_iter(self.deck.player2.clone().drain(..(p2 as usize)));
                let deck = Deck { player1, player2 };
                Game::new(deck).play()
            } else {
                if p1 > p2 {
                    Some(One)
                } else {
                    Some(Two)
                }
            };

        match result {
            Some(One) => {
                self.deck.player1.push_back(p1);
                self.deck.player1.push_back(p2);
            }
            Some(Two) => {
                self.deck.player2.push_back(p2);
                self.deck.player2.push_back(p1);
            }
            None => unreachable!("Should have a winner here"),
        }

        None
    }

    pub fn play(&mut self) -> Option<Player> {
        loop {
            if self.deck.player1.is_empty() {
                return Some(Two);
            } else if self.deck.player2.is_empty() {
                return Some(One);
            }
            if let Some(result) = self.round() {
                return Some(result);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut deck = Deck::new(input);

    deck.process_game();

    deck.score()
}

pub fn part_two(input: &str) -> Option<usize> {
    let deck = Deck::new(input);
    let mut game = Game::new(deck);

    let winner = game.play().unwrap();
    let winning_hand = match winner {
        One => game.deck.player1,
        Two => game.deck.player2,
    };

    Some(
        winning_hand
            .iter()
            .rev()
            .enumerate()
            .map(|(i, v)| *v as usize * (i + 1))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(306));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(291));
    }
}
