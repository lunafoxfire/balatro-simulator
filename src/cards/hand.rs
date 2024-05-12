#[cfg(test)]
#[path = "hand.test.rs"]
mod tests;

use std::{collections::HashMap, fmt};
use itertools::Itertools;
use super::{Card, HandType, Rank, Suit};

#[derive(Default)]
pub struct HandEval {
    is_pair: bool,
    is_two_pair: bool,
    is_three_of_a_kind: bool,
    is_straight: bool,
    is_flush: bool,
    is_full_house: bool,
    is_four_of_a_kind: bool,
    is_straight_flush: bool,
}

impl HandEval {
    pub fn new() -> Self {
        HandEval {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        self.is_pair = false;
        self.is_two_pair = false;
        self.is_three_of_a_kind = false;
        self.is_straight = false;
        self.is_flush = false;
        self.is_full_house = false;
        self.is_four_of_a_kind = false;
        self.is_straight_flush = false;
    }
}

pub struct Hand {
    cards: Vec<Card>,
    eval: HandEval,
    needs_evaluated: bool,
    suit_counts: HashMap<Suit, u32>,
    rank_counts: HashMap<i32, u32>,
}

impl Hand {
    pub fn cards(&self) -> &Vec<Card> { &self.cards }
    pub fn size(&self) -> u32 { self.cards().len().try_into().expect("Hand size should fit in a u32") }
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::new(),
            eval: HandEval::new(),
            needs_evaluated: false,
            suit_counts: HashMap::new(),
            rank_counts: HashMap::new(),
        }
    }

    pub fn as_str(&self) -> String {
        self.cards.iter()
            .map(|card| { card.as_str() })
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn reset(&mut self) {
        self.cards.clear();
        self.reset_eval();
        self.needs_evaluated = false;
    }

    fn reset_eval(&mut self) {
        self.eval.reset();
        self.suit_counts.clear();
        self.rank_counts.clear();
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
        self.needs_evaluated = true;
    }

    pub fn discard(&mut self, count: u32) {
        for _ in 0..count {
            self.cards.pop();
        }
    }

    fn evaluate(&mut self) {
        if !self.needs_evaluated { return };
        self.reset_eval();

        for card in self.cards.iter() {
            *self.suit_counts.entry(*card.suit()).or_insert(0) += 1;
            *self.rank_counts.entry(card.rank().as_val()).or_insert(0) += 1;
            if *card.rank() == Rank::Ace {
                *self.rank_counts.entry(1).or_insert(0) += 1;
            }
        }

        for (_, count) in self.suit_counts.iter() {
            if *count >= 5 {
                self.eval.is_flush = true;
            }
        }

        let mut prev_rank = 0;
        let mut straight_count = 1;
        for (rank, count) in self.rank_counts.iter().sorted() {
            if prev_rank != 0 && *rank == prev_rank + 1 {
                straight_count += 1;
                if straight_count >= 5 {
                    self.eval.is_straight = true;
                }
            } else {
                straight_count = 1;
            }
            prev_rank = *rank;

            if *rank == 1 { continue; }
            let prev_pair_found = self.eval.is_pair;
            let prev_triple_found = self.eval.is_three_of_a_kind;
            if *count >= 4 {
                self.eval.is_four_of_a_kind = true;
            }
            if *count >= 3 {
                self.eval.is_three_of_a_kind = true;
                if prev_pair_found {
                    self.eval.is_full_house = true;
                }
            }
            if *count >= 2 {
                self.eval.is_pair = true;
                if prev_pair_found {
                    self.eval.is_two_pair = true;
                }
                if prev_triple_found {
                    self.eval.is_full_house = true;
                }
            }
        }

        if self.eval.is_straight && self.eval.is_flush {
            self.eval.is_straight_flush = true;
        }

        self.needs_evaluated = false;
    }

    pub fn is_type(&mut self, hand_type: &HandType) -> bool {
        self.evaluate();
        match hand_type {
            HandType::Pair => self.eval.is_pair,
            HandType::TwoPair => self.eval.is_two_pair,
            HandType::ThreeOfAKind => self.eval.is_three_of_a_kind,
            HandType::Straight => self.eval.is_straight,
            HandType::Flush => self.eval.is_flush,
            HandType::FullHouse => self.eval.is_full_house,
            HandType::FourOfAKind => self.eval.is_four_of_a_kind,
            HandType::StraightFlush => self.eval.is_straight_flush,
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
