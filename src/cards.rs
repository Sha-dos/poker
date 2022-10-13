use crate::poker_error::Error;

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

#[derive(Clone, Debug)]
pub struct Card {
    card_type: Type,
    suit: Suit,
}

#[derive(Clone, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

#[derive(Clone, Debug)]
pub enum Type {
    Ace,
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
    Null,
}

impl Hand {
    fn default() -> Hand {
        let cards = vec![Card { card_type: Type::Ace, suit: Suit::Club }];
        Hand {
            cards
        }
    }
}

impl Card {
    fn to_card(card_type: char, suit: Suit) -> Result<Card, Error> {
        let mut _card_type = Type::Null;

        match card_type {
            'A' => _card_type = Type::Ace,
            '2' => _card_type = Type::Ace,
            '3' => _card_type = Type::Ace,
            '4' => _card_type = Type::Ace,
            '5' => _card_type = Type::Ace,
            '6' => _card_type = Type::Ace,
            '7' => _card_type = Type::Ace,
            '8' => _card_type = Type::Ace,
            '9' => _card_type = Type::Ace,
            'T' => _card_type = Type::Ace,
            'J' => _card_type = Type::Ace,
            'Q' => _card_type = Type::Ace,
            'K' => _card_type = Type::Ace,
            _ => return Error::FailedToParse,
        }

        Ok(Card { card_type: (), suit: () })
    }
}

pub fn parse_cards(line: String) -> Result<(), Error> {
    let vector: Vec<&str> = line.split_whitespace().collect();

    let parsed: Vec<Card> =

    let mut p1: Hand = Hand::default();
    let mut p2: Hand = Hand::default();

    for i in vector {
        println!("{}", i);
    }

    Ok(())
}