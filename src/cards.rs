use crate::poker_error::Error;
use crate::cards::Type::*;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Card {
    pub card_type: Type,
    pub suit: Suit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
    Null,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Null,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand {
            cards
        }
    }
}

impl Type {
    pub fn iterator() -> impl Iterator<Item = Type> {
        [Ace,
         Two,
         Three,
         Four,
         Five,
         Six,
         Seven,
         Eight,
         Nine,
         Ten,
         Jack,
         Queen,
         King,
         Null,].iter().copied()
    }
}

pub fn parse_cards(line: String) -> Result<(Hand, Hand), Error> {
    let vector: Vec<&str> = line.split_whitespace().collect();

    let mut parsed: Vec<Card> = Vec::new();

    for i in vector {
        let p = parse_token(i.chars().nth(0).unwrap(), i.chars().nth(1).unwrap());
        parsed.push(p.unwrap());
    }

    let p1: Hand = Hand::new(parsed[0..5].to_vec());
    let p2: Hand = Hand::new(parsed[5..10].to_vec());

    Ok((p1, p2))
}

fn parse_token(card_type: char, card_suit: char) -> Result<Card, Error> {
    let mut _card_type = Type::Null;
    let mut _card_suit = Suit::Null;

    match card_type {
        'A' => _card_type = Type::Ace,
        '2' => _card_type = Type::Two,
        '3' => _card_type = Type::Three,
        '4' => _card_type = Type::Four,
        '5' => _card_type = Type::Five,
        '6' => _card_type = Type::Six,
        '7' => _card_type = Type::Seven,
        '8' => _card_type = Type::Eight,
        '9' => _card_type = Type::Nine,
        'T' => _card_type = Type::Ten,
        'J' => _card_type = Type::Jack,
        'Q' => _card_type = Type::Queen,
        'K' => _card_type = Type::King,
        _ => return Err(Error::FailedToParse),
    }

    match card_suit {
        'C' => _card_suit = Suit::Club,
        'S' => _card_suit = Suit::Spade,
        'D' => _card_suit = Suit::Diamond,
        'H' => _card_suit = Suit::Heart,
        _ => return Err(Error::FailedToParse),
    }

    Ok(Card { card_type: _card_type, suit: _card_suit })
}