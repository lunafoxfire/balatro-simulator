use std::fmt;
use strum::EnumIter;

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
pub enum Suit {
    Spade,
    Heart,
    Diamond,
    Club
}

impl Suit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Suit::Spade => "♠",
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
            Suit::Club => "♣",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
