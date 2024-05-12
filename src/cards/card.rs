use std::fmt;
use strum::IntoEnumIterator;
use super::{rank::Rank, suit::Suit};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn rank(&self) -> &Rank { &self.rank }
    pub fn suit(&self) -> &Suit { &self.suit }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card {
            rank,
            suit,
        }
    }

    pub fn as_str(&self) -> String {
        format!("{}{}", self.suit.as_str(), self.rank.as_str())
    }

    pub fn for_every<F>(mut f: F)
    where
        F: FnMut(Card) -> (),
    {
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                f(Card::new(rank, suit));
            }
        }
    }

    pub fn change_suit(&mut self, suit: Suit) {
        self.suit = suit;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
