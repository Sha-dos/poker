use crate::cards::Card;
use crate::cards::Hand;
use crate::cards::Type;

#[derive(Debug, Eq, PartialEq)]
pub enum Player {
    Player1,
    Player2,
    Null,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,
    StraightFlush,
    RoyalFlush,
}

impl HandType {
    fn to_score(&self) -> i8 {
        match self {
            HandType::HighCard => 0,
            HandType::Pair => 1,
            HandType::TwoPair => 2,
            HandType::Three => 3,
            HandType::Straight => 4,
            HandType::Flush => 5,
            HandType::FullHouse => 6,
            HandType::Four => 7,
            HandType::StraightFlush => 8,
            HandType::RoyalFlush => 20,
            _ => 0,
        }
    }
}

pub fn get_highest_score(p1: Vec<HandType>, p1_highest_card: Card, p2: Vec<HandType>, p2_highest_card: Card) -> (Player, u8, u8, HandType) {
    let p1_highest = p1[p1.len() - 1].to_score();
    let p2_highest = p2[p2.len() - 1].to_score();

    // println!("Scores: {}, {}", p1_highest, p2_highest);

    let mut winning_player: Player = Player::Null;
    let mut reason: &HandType = &HandType::HighCard;

    if p1_highest - p2_highest < 0 {
        winning_player = Player::Player2;
        reason = &p2[&p2.len() - 1];
    } else if p1_highest - p2_highest > 0 {
        winning_player = Player::Player1;
        reason = &p1[&p1.len() - 1];
    } else if p1_highest_card.card_type as i8 - (p2_highest_card.card_type as i8) < 0 {
        winning_player = Player::Player2;
    } else if p1_highest_card.card_type as i8 - p2_highest_card.card_type as i8 > 0 {
        winning_player = Player::Player1;
    }

    let winner_ret = match winning_player {
        Player::Player1 => (winning_player, p1_highest as u8, p2_highest as u8, reason.clone()),
        Player::Player2 => (winning_player, p2_highest as u8, p1_highest as u8, reason.clone()),
        Player::Null => (winning_player, 0, 0, reason.clone()),
    };

    winner_ret
}

pub fn get_highest(hand: &mut Hand) -> Card {
    hand.cards.sort();

    hand.cards[4]
}

pub fn get_score_vectors(p1: &Hand, p2: &Hand) -> (Vec<HandType>, Vec<HandType>) {
    let mut p1_hand_type = get_hand_type(p1.clone());
    let mut p2_hand_type = get_hand_type(p2.clone());

    p1_hand_type.sort();
    p2_hand_type.sort();

    //println!("P1: {:?} P2: {:?}", p1_hand_type[p1_hand_type.len() - 1], p2_hand_type[p2_hand_type.len() - 1]);

    (p1_hand_type, p2_hand_type)
}

fn get_hand_type(mut hand: Hand) -> Vec<HandType> {
    let hr = histogram(&hand);

    let card_multaples: (i8, i8, i8) = (hr[0].1, hr[1].1, hr[2].1);

    //println!("Multaples: {:?}", card_multaples);

    let mut flush: bool = false;
    let mut straight: bool = false;

    let mut vec_of_types: Vec<HandType> = Vec::new();

    hand.cards.sort();

    let first = hand.cards[0].suit;
    flush = hand.cards.iter().all(|&item| item.suit == first);
    //println!("Flush: {}", flush);

    if hand.cards[4].card_type as u8 == 12 && hand.cards[0].card_type as u8 == 8 { vec_of_types.push(HandType::RoyalFlush) }

    if hand.cards[4].card_type as u8 - hand.cards[0].card_type as u8 == 4 { straight = true; }

    if flush && straight { vec_of_types.push(HandType::StraightFlush) }
    else if flush { vec_of_types.push(HandType::Flush) }
    else if straight { vec_of_types.push(HandType::Straight) }
    else { vec_of_types.push(HandType::HighCard) }

    match card_multaples {
        (4, 1, 0) => vec_of_types.push(HandType::Four),
        (2, 1, 1) => vec_of_types.push(HandType::Pair),
        (3, 1, 1) => vec_of_types.push(HandType::Three),
        (3, 2, 0) => vec_of_types.push(HandType::FullHouse),
        (2, 2, 1) => vec_of_types.push(HandType::TwoPair),
        _ => {
            vec_of_types.push(HandType::HighCard)
        }
    };

    vec_of_types
}

fn histogram(hand: &Hand) -> Vec<(Type, i8)> {
    let mut card_amount: Vec<(Type, i8)> = vec![(Type::Null, 0)];

    for i in Type::iterator() {
        card_amount.push((i, hand.cards.iter().filter(|&n| n.card_type == i).count() as i8));
    }

    card_amount.sort_by(|a, b| b.1.cmp(&a.1));

    card_amount
}