use std::fmt;
use strum::EnumIter;

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Rank {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Ten => "10",
            Rank::Nine => "9",
            Rank::Eight => "8",
            Rank::Seven => "7",
            Rank::Six => "6",
            Rank::Five => "5",
            Rank::Four => "4",
            Rank::Three => "3",
            Rank::Two => "2",
        }
    }

    pub fn as_val(&self) -> i32 {
        match self {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
