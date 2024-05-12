use std::fmt;
use strum::EnumIter;

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
pub enum HandType {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush
}

impl HandType {
    pub fn as_str(&self) -> &'static str {
        match self {
            HandType::Pair => "PAIR",
            HandType::TwoPair => "TWO_PAIR",
            HandType::ThreeOfAKind => "THREE_OF_A_KIND",
            HandType::Straight => "STRAIGHT",
            HandType::Flush => "FLUSH",
            HandType::FullHouse => "FULL_HOUSE",
            HandType::FourOfAKind => "FOUR_OF_A_KIND",
            HandType::StraightFlush => "STRAIGHT_FLUSH",
        }
    }
}

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
