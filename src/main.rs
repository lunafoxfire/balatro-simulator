mod cards {
    mod card; pub use card::*;
    mod deck; pub use deck::*;
    mod hand_type; pub use hand_type::*;
    mod hand; pub use hand::*;
    mod rank; pub use rank::*;
    mod setup; pub use setup::*;
    mod suit; pub use suit::*;
}

use cards::{HandType, Setup, DeckType};
use std::{ops::RangeInclusive, time::Instant};
use strum::IntoEnumIterator;

use crate::cards::TestType;

#[derive(Default)]
struct HandCount {
    pair: u32,
    two_pair: u32,
    three_of_a_kind: u32,
    straight: u32,
    flush: u32,
    full_house: u32,
    four_of_a_kind: u32,
    straight_flush: u32,
}

impl HandCount {
    pub fn new() -> Self {
        HandCount {
            ..Default::default()
        }
    }

    pub fn val(&mut self, hand_type: &HandType) -> &mut u32 {
        match hand_type {
            HandType::Pair => &mut self.pair,
            HandType::TwoPair => &mut self.two_pair,
            HandType::ThreeOfAKind => &mut self.three_of_a_kind,
            HandType::Straight => &mut self.straight,
            HandType::Flush => &mut self.flush,
            HandType::FullHouse => &mut self.full_house,
            HandType::FourOfAKind => &mut self.four_of_a_kind,
            HandType::StraightFlush => &mut self.straight_flush,
        }
    }
}

const ITERATIONS: u32 = 1_000;
const HAND_SIZES: RangeInclusive<u32> = 5..=10;
fn main() {
    println!("Starting...\n");
    let now = Instant::now();

    for deck_setup in DeckType::iter() {
        for hand_setup in TestType::iter() {
            test_setup(Setup::new(deck_setup, hand_setup));
        }
    }

    let elapsed = now.elapsed();
    println!("Done in: {:?}", elapsed);
}

fn test_setup(mut setup: Setup) {
    for hand_size in HAND_SIZES {
        let name = format!("{} | HAND: {} | {}", setup.hand_setup().test_type(), hand_size, setup.deck_type());
        let mut hand_counts = HandCount::new();

        for _ in 0..ITERATIONS {
            setup.reset();
            setup.shuffle();
            setup.draw_to(hand_size);
            setup.redraw();

            match setup.hand_setup().target_hand() {
                None => {
                    for hand_type in HandType::iter() {
                        if setup.hand().is_type(&hand_type) {
                            *hand_counts.val(&hand_type) += 1;
                        }
                    }
                },
                Some(target_hand) => {
                    // TODO how should I do this better?
                    let target_hand = &target_hand.clone();
                    if setup.hand().is_type(target_hand) {
                        *hand_counts.val(target_hand) += 1;
                    }
                }
            }
        }

        println!("==== {} ====", name);
        match setup.hand_setup().target_hand() {
            None => {
                for hand_type in HandType::iter() {
                    println!("{}: {}", hand_type, *hand_counts.val(&hand_type));
                }
            },
            Some(target_hand) => {
                let target_hand = *target_hand;
                println!("{}: {}", target_hand, *hand_counts.val(&target_hand));
            }
        }
        println!("");
    }
}
