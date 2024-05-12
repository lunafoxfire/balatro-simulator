#[cfg(test)]
#[path = "setup.test.rs"]
mod tests;

use std::{cmp, collections::HashSet};
use itertools::Itertools;
use strum::{Display, EnumIter};
use super::{Card, Deck, Hand, HandType, Rank, Suit};

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter, Display)]
pub enum DeckType {
    Standard,
    NoFaceCards,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter, Display)]
pub enum TestType {
    SingleDraw,
    DrawIntoOpenStraightMissingTwo,
    DrawIntoOpenStraightMissingOne,
    DrawIntoFlushMissingTwo,
    DrawIntoFlushMissingOne,
    DrawIntoFullHouseFromThreeOfAKind,
    DrawIntoFullHouseFromTwoPair,
}

pub struct HandSetup {
    test_type: TestType,
    target_hand: Option<HandType>,
    initial_hand: HashSet<Card>,
    initial_hand_size: u32,
    initial_hand_rejects: HashSet<Card>,
}

impl HandSetup {
    pub fn test_type(&self) -> &TestType { &self.test_type }
    pub fn target_hand(&self) -> &Option<HandType> { &self.target_hand }
    pub fn initial_hand(&self) -> &HashSet<Card> { &self.initial_hand }
    pub fn initial_hand_size(&self) -> &u32 { &self.initial_hand_size }
    pub fn initial_hand_rejects(&self) -> &HashSet<Card> { &self.initial_hand_rejects }
}

impl HandSetup {
    fn new(test_type: TestType) -> Self {
        let mut hand_setup = HandSetup {
            test_type,
            target_hand: None,
            initial_hand: HashSet::new(),
            initial_hand_size: 0,
            initial_hand_rejects: HashSet::new(),
        };
        hand_setup.set_target_hand();
        hand_setup.set_initial_hand();
        hand_setup.set_initial_hand_rejects();
        hand_setup
    }

    fn set_target_hand(&mut self) {
        self.target_hand = match self.test_type {
            TestType::SingleDraw => None,
            TestType::DrawIntoOpenStraightMissingTwo => Some(HandType::Straight),
            TestType::DrawIntoOpenStraightMissingOne => Some(HandType::Straight),
            TestType::DrawIntoFlushMissingTwo => Some(HandType::Flush),
            TestType::DrawIntoFlushMissingOne => Some(HandType::Flush),
            TestType::DrawIntoFullHouseFromThreeOfAKind => Some(HandType::FullHouse),
            TestType::DrawIntoFullHouseFromTwoPair => Some(HandType::FullHouse),
        };
    }

    fn set_initial_hand(&mut self) {
        self.initial_hand = match self.test_type {
            TestType::SingleDraw => HashSet::new(),
            TestType::DrawIntoOpenStraightMissingTwo => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Four, Suit::Heart),
                Card::new(Rank::Five, Suit::Diamond),
            ]),
            TestType::DrawIntoOpenStraightMissingOne => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Four, Suit::Heart),
                Card::new(Rank::Five, Suit::Diamond),
                Card::new(Rank::Six, Suit::Club),
            ]),
            TestType::DrawIntoFlushMissingTwo => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Six, Suit::Spade),
                Card::new(Rank::Nine, Suit::Spade),
            ]),
            TestType::DrawIntoFlushMissingOne => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Six, Suit::Spade),
                Card::new(Rank::Eight, Suit::Spade),
                Card::new(Rank::Nine, Suit::Spade),
            ]),
            TestType::DrawIntoFullHouseFromThreeOfAKind => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Three, Suit::Heart),
                Card::new(Rank::Three, Suit::Diamond),
            ]),
            TestType::DrawIntoFullHouseFromTwoPair => HashSet::from([
                Card::new(Rank::Three, Suit::Spade),
                Card::new(Rank::Three, Suit::Heart),
                Card::new(Rank::Five, Suit::Diamond),
                Card::new(Rank::Five, Suit::Club),
            ]),
        };
        self.initial_hand_size = self.initial_hand.len().try_into().expect("Initial hand size should fit in a u32");
    }

    fn set_initial_hand_rejects(&mut self) {
        self.initial_hand_rejects = match self.test_type {
            TestType::SingleDraw => HashSet::new(),
            TestType::DrawIntoOpenStraightMissingTwo => self.cards_extending_straight(),
            TestType::DrawIntoOpenStraightMissingOne => self.cards_extending_straight(),
            TestType::DrawIntoFlushMissingTwo => self.cards_matching_suit(),
            TestType::DrawIntoFlushMissingOne => self.cards_matching_suit(),
            TestType::DrawIntoFullHouseFromThreeOfAKind => self.cards_matching_rank(),
            TestType::DrawIntoFullHouseFromTwoPair => self.cards_matching_rank(),
        };
    }

    fn cards_matching_suit(&self) -> HashSet<Card> {
        let mut set = HashSet::new();
        Card::for_every(|card| {
            for hand_card in self.initial_hand.iter() {
                if card.suit() == hand_card.suit() {
                    set.insert(card);
                }
            }
        });
        set
    }

    fn cards_matching_rank(&self) -> HashSet<Card> {
        let mut set = HashSet::new();
        Card::for_every(|card| {
            for hand_card in self.initial_hand.iter() {
                if card.rank() == hand_card.rank() {
                    set.insert(card);
                }
            }
        });
        set
    }

    fn cards_extending_straight(&self) -> HashSet<Card> {
        let mut set = HashSet::new();
        let sorted = self.initial_hand.iter()
            .sorted_by_key(|card| card.rank().as_val())
            .collect::<Vec<&Card>>();
        let first = sorted.first();
        let last = sorted.last();
        if let (Some(first), Some(last)) = (first, last) {
            let prev_val = first.rank().as_val() - 1;
            let next_val = last.rank().as_val() + 1;
            Card::for_every(|card| {
                let val = card.rank().as_val();
                if val == prev_val || val == next_val {
                    set.insert(card);
                }
            });
        }
        set
    }
}

pub struct Setup {
    deck: Deck,
    hand: Hand,
    deck_type: DeckType,
    hand_setup: HandSetup,
}

impl Setup {
    pub fn deck(&self) -> &Deck { &self.deck }
    pub fn hand(&mut self) -> &mut Hand { &mut self.hand }
    pub fn deck_type(&self) -> &DeckType { &self.deck_type }
    pub fn hand_setup(&self) -> &HandSetup { &self.hand_setup }
}

impl Setup {
    pub fn new(deck_type: DeckType, test_type: TestType) -> Self
    {
        Setup {
            deck: Deck::new(),
            hand: Hand::new(),
            deck_type,
            hand_setup: HandSetup::new(test_type),
        }
    }

    pub fn reset(&mut self) {
        self.hand.reset();
        for card in self.hand_setup.initial_hand.iter() {
            self.hand.add_card(*card);
        }

        self.deck.reset();
        Card::for_every(|card| {
            if self.hand_setup.initial_hand.contains(&card) { return; }
            if self.deck_type == DeckType::NoFaceCards {
                if let Rank::King | Rank::Queen | Rank::Jack = card.rank() {
                    return;
                }
            }
            self.deck.add_card(card);
        });
    }

    pub fn is_target_hand(&mut self) -> bool {
        match self.hand_setup.target_hand {
            None => false,
            Some(hand_type) => self.hand.is_type(&hand_type)
        }
    }

    pub fn shuffle(&mut self) {
        self.deck.shuffle();
    }

    pub fn draw_to(&mut self, target_size: u32) {
        let mut rejected_cards: Vec<Card> = Vec::new();
        while self.hand.size() < target_size {
            let next_card = self.deck.pop_next_card().expect("Deck ran out trying to initialize hand");
            if self.hand_setup.initial_hand_rejects.contains(&next_card) {
                rejected_cards.push(next_card);
            } else {
                self.hand.add_card(next_card);
            }
        }

        if rejected_cards.len() > 0 {
            for card in rejected_cards.into_iter() {
                self.deck.add_card(card);
            }
            self.deck.shuffle();
        }
    }

    pub fn redraw(&mut self) {
        let current_size = self.hand.size();
        let redraw_count = cmp::min(current_size - self.hand_setup.initial_hand_size, 5);
        self.hand.discard(redraw_count);
        self.deck.draw(&mut self.hand, redraw_count);
    }
}
