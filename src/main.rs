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
use std::{collections::HashMap, fs, ops::RangeInclusive, time::Instant};
use strum::IntoEnumIterator;

use crate::cards::TestType;

#[derive(Default, Debug)]
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

    pub fn get(&self, hand_type: &HandType) -> &u32 {
        match hand_type {
            HandType::Pair => &self.pair,
            HandType::TwoPair => &self.two_pair,
            HandType::ThreeOfAKind => &self.three_of_a_kind,
            HandType::Straight => &self.straight,
            HandType::Flush => &self.flush,
            HandType::FullHouse => &self.full_house,
            HandType::FourOfAKind => &self.four_of_a_kind,
            HandType::StraightFlush => &self.straight_flush,
        }
    }

    pub fn get_auto(&self) -> (HandType, &u32) {
        for hand_type in HandType::iter() {
            let val = self.get(&hand_type);
            if *val > 0 {
                return (hand_type, &val);
            }
        }
        panic!("No nonzero result found")
    }
}

struct Results {
    data: HashMap<DeckType,
            HashMap<TestType,
                HashMap<u32, HandCount>
            >
        >
}

impl Results {
    pub fn new() -> Self {
        Results {
            data: HashMap::new()
        }
    }

    fn get_name(deck_type: &DeckType, test_type: &TestType, hand_size: &u32) -> String {
        format!("{} | HandSize: {} | {}", test_type, hand_size, deck_type)
    }

    pub fn get(&self, deck_type: &DeckType, test_type: &TestType, hand_size: &u32) -> Option<&HandCount> {
        self.data.get(deck_type)?.get(test_type)?.get(hand_size)
    }

    pub fn set(&mut self, deck_type: &DeckType, test_type: &TestType, hand_size: &u32, data: HandCount) {
        self.data.entry(*deck_type).or_insert(HashMap::new())
            .entry(*test_type).or_insert(HashMap::new())
                .insert(*hand_size, data);
    }
}

const ITERATIONS: u32 = 1_000_000;
const HAND_SIZES: RangeInclusive<u32> = 5..=10;
const OUTPUT_DIR: &str = "./output";
fn main() {
    println!("Starting...");
    let now = Instant::now();

    let mut results = Results::new();
    for deck_type in DeckType::iter() {
        for test_type in TestType::iter() {
            run_setup(Setup::new(deck_type, test_type), &mut results);
        }
    }

    println!("Writing files...");
    fs::create_dir_all(OUTPUT_DIR).expect("Unable to create directory");
    for deck_type in DeckType::iter() {
        let mut output = String::new();
        let mut line1 = String::from("Simulation");
        for (i, _) in HAND_SIZES.enumerate() {
            if i == 0 {
                line1 += ",Hand Sizes"
            } else {
                line1 += ","
            }
        }
        let mut line2 = String::from("");
        for hand_size in HAND_SIZES {
            line2 += &format!(",{}", hand_size);
        }
        output += &format!("{}\n{}\n", line1, line2);

        for test_type in TestType::iter() {
            if test_type == TestType::SingleDraw {
                for hand_type in HandType::iter() {
                    let mut line = format!("SingleDraw{}", hand_type);
                    for hand_size in HAND_SIZES {
                        let data = results.get(&deck_type, &test_type, &hand_size).expect("Results entry not found");
                        let hand_count = data.get(&hand_type);
                        let percentage = *hand_count as f32 / ITERATIONS as f32;
                        line += &format!(",{}", percentage);
                    }
                    output += &format!("{}\n", line);
                }
            } else {
                let mut line = format!("{}", test_type);
                for hand_size in HAND_SIZES {
                    let data = results.get(&deck_type, &test_type, &hand_size).expect("Results entry not found");
                    let (_, hand_count) = data.get_auto();
                    let percentage = *hand_count as f32 / ITERATIONS as f32;
                    line += &format!(",{}", percentage);
                }
                output += &format!("{}\n", line);
            }
        }

        fs::write(format!("./output/{}.csv", deck_type), output).expect("Unable to write file");
    }

    let elapsed = now.elapsed();
    println!("Done in: {:?}", elapsed);
}

fn run_setup(mut setup: Setup, results: &mut Results) {
    for hand_size in HAND_SIZES {
        let name = Results::get_name(setup.deck_type(), setup.hand_setup().test_type(), &hand_size);
        println!("Running: {}", name);
        let mut hand_counts = HandCount::new();

        for _ in 0..ITERATIONS {
            setup.reset();
            setup.shuffle();
            setup.draw_to(hand_size);
            setup.redraw();

            match *setup.hand_setup().target_hand() {
                None => {
                    for hand_type in HandType::iter() {
                        if setup.hand().is_type(&hand_type) {
                            *hand_counts.val(&hand_type) += 1;
                        }
                    }
                },
                Some(target_hand) => {
                    if setup.hand().is_type(&target_hand) {
                        *hand_counts.val(&target_hand) += 1;
                    }
                }
            }
        }

        results.set(setup.deck_type(), setup.hand_setup().test_type(), &hand_size, hand_counts);
    }
}
