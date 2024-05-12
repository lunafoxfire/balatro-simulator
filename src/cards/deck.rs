use std::fmt;
use rand::seq::SliceRandom;
use super::{card::Card, hand::Hand};

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn cards(&self) -> &Vec<Card> { &self.cards }
    pub fn size(&self) -> u32 { self.cards().len().try_into().expect("Deck size should fit in a u32") }
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: Vec::new()
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
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self, hand: &mut Hand, count: u32) {
        for _ in 0..count {
            if let Some(card) = self.cards.pop() {
                hand.add_card(card);
            }
        }
    }

    pub fn pop_next_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
