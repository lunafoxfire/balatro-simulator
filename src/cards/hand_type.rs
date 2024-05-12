use strum::{Display, EnumIter};

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter, Display)]
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
