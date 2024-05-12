use crate::cards::{Rank, Suit};
use super::*;

#[test]
fn hand_is_pair() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        assert!(hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::Pair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::Pair));
    }
}

#[test]
fn hand_is_two_pair() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::TwoPair));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::TwoPair));
    }
}

#[test]
fn hand_is_three_of_a_kind() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        assert!(hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::ThreeOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::Four, Suit::Club));
        assert!(hand.is_type(&HandType::ThreeOfAKind));
    }
}

#[test]
fn hand_is_four_of_a_kind() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::Ace, Suit::Diamond));
        assert!(hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::Ace, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::FourOfAKind));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::Four, Suit::Club));
        assert!(hand.is_type(&HandType::FourOfAKind));
    }
}

#[test]
fn hand_is_full_house() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        assert!(hand.is_type(&HandType::FullHouse));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Queen, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Nine, Suit::Club));
        hand.add_card(Card::new(Rank::Four, Suit::Diamond));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Club));
        assert!(hand.is_type(&HandType::FullHouse));
    }
}

#[test]
fn hand_is_flush() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::Six, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Nine, Suit::Spade));
        hand.add_card(Card::new(Rank::Three, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Diamond));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        assert!(!hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        assert!(!hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Heart));
        assert!(hand.is_type(&HandType::Flush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Club));
        hand.add_card(Card::new(Rank::Eight, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Heart));
        assert!(hand.is_type(&HandType::Flush));
    }
}

#[test]
fn hand_is_straight() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::Six, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::Ten, Suit::Club));
        hand.add_card(Card::new(Rank::Jack, Suit::Club));
        assert!(!hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Queen, Suit::Diamond));
        hand.add_card(Card::new(Rank::King, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Three, Suit::Spade));
        assert!(!hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Diamond));
        
        assert!(hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Three, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Diamond));
        assert!(hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::Queen, Suit::Spade));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Ten, Suit::Diamond));
        assert!(hand.is_type(&HandType::Straight));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Eight, Suit::Diamond));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Queen, Suit::Spade));
        assert!(hand.is_type(&HandType::Straight));
    }
}

#[test]
fn hand_is_straight_flush() {
    {
        let mut hand = Hand::new();
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Eight, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Diamond));
        hand.add_card(Card::new(Rank::Six, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Nine, Suit::Heart));
        hand.add_card(Card::new(Rank::Ten, Suit::Heart));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Queen, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Heart));
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Three, Suit::Heart));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Spade));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Diamond));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Eight, Suit::Heart));
        assert!(hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Spade));
        hand.add_card(Card::new(Rank::Three, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Diamond));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::Two, Suit::Heart));
        hand.add_card(Card::new(Rank::Three, Suit::Heart));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Heart));
        assert!(hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::Queen, Suit::Spade));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Ten, Suit::Diamond));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Heart));
        hand.add_card(Card::new(Rank::Queen, Suit::Heart));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Ten, Suit::Heart));
        assert!(hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Eight, Suit::Diamond));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Spade));
        hand.add_card(Card::new(Rank::Queen, Suit::Spade));
        assert!(!hand.is_type(&HandType::StraightFlush));
    }
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Eight, Suit::Heart));
        hand.add_card(Card::new(Rank::Jack, Suit::Heart));
        hand.add_card(Card::new(Rank::Five, Suit::Heart));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::Four, Suit::Heart));
        hand.add_card(Card::new(Rank::Seven, Suit::Heart));
        hand.add_card(Card::new(Rank::Six, Suit::Heart));
        hand.add_card(Card::new(Rank::Queen, Suit::Spade));
        assert!(hand.is_type(&HandType::StraightFlush));
    }
}

#[test]
fn hand_eval_updates_on_change() {
    {
        let mut hand = Hand::new();
        hand.add_card(Card::new(Rank::Ace, Suit::Spade));
        assert!(!hand.is_type(&HandType::Pair));

        hand.add_card(Card::new(Rank::Ace, Suit::Heart));
        assert!(hand.is_type(&HandType::Pair));
        assert!(!hand.is_type(&HandType::FullHouse));

        hand.add_card(Card::new(Rank::Ace, Suit::Club));
        hand.add_card(Card::new(Rank::King, Suit::Spade));
        hand.add_card(Card::new(Rank::King, Suit::Club));
        assert!(hand.is_type(&HandType::FullHouse));
    }
}
