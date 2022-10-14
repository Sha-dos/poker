#[cfg(test)]
mod tests {
    use crate::{cards::{*, self}, logic::{self, HandType}};

    #[test]
    fn parse_hands() {
        assert_eq!(parse_cards("2C 3D 4S 5C 6H 7S 8C 9S TS JH".to_string()).unwrap(), (Hand::new(vec![
        Card { card_type: Type::Two, suit: Suit::Club },
        Card { card_type: Type::Three, suit: Suit::Diamond },
        Card { card_type: Type::Four, suit: Suit::Spade },
        Card { card_type: Type::Five, suit: Suit::Club },
        Card { card_type: Type::Six, suit: Suit::Heart },
        ]), Hand::new(vec![
        Card { card_type: Type::Seven, suit: Suit::Spade },
        Card { card_type: Type::Eight, suit: Suit::Club },
        Card { card_type: Type::Nine, suit: Suit::Spade },
        Card { card_type: Type::Ten, suit: Suit::Spade },
        Card { card_type: Type::Jack, suit: Suit::Heart },
        ])));
    }

    #[test]
    fn parse_hands2() {
        assert_eq!(parse_cards("QC KD 4S 5C 6H 7S 8C 9S TS JH".to_string()).unwrap(), (Hand::new(vec![
        Card { card_type: Type::Queen, suit: Suit::Club },
        Card { card_type: Type::King, suit: Suit::Diamond },
        Card { card_type: Type::Four, suit: Suit::Spade },
        Card { card_type: Type::Five, suit: Suit::Club },
        Card { card_type: Type::Six, suit: Suit::Heart },
        ]), Hand::new(vec![
        Card { card_type: Type::Seven, suit: Suit::Spade },
        Card { card_type: Type::Eight, suit: Suit::Club },
        Card { card_type: Type::Nine, suit: Suit::Spade },
        Card { card_type: Type::Ten, suit: Suit::Spade },
        Card { card_type: Type::Jack, suit: Suit::Heart },
        ])));
    }

    #[test]
    fn pair() {
        let (p1, p2) = cards::parse_cards("AC AC 4H 7S 9S 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(1, highest_score.1)
    }

    #[test]
    fn two() {
        let (p1, p2) = cards::parse_cards("AC AC 4H 4S 9S 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(2, highest_score.1)
    }

    #[test]
    fn three() {
        let (p1, p2) = cards::parse_cards("AC AC AH 4S 9S 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(3, highest_score.1)
    }

    #[test]
    fn straight() {
        let (p1, p2) = cards::parse_cards("2C 3C 4H 5S 6S 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(4, highest_score.1)
    }

    #[test]
    fn flush() {
        let (p1, p2) = cards::parse_cards("AC AC 4C AC 9C 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(5, highest_score.1)
    }

    #[test]
    fn full_house() {
        let (p1, p2) = cards::parse_cards("AC AC AC 9C 9C 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(6, highest_score.1)
    }

    #[test]
    fn four() {
        let (p1, p2) = cards::parse_cards("AC AC AC AC 9C 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(7, highest_score.1)
    }

    #[test]
    fn straight_flush() {
        let (p1, p2) = cards::parse_cards("2C 3C 4C 5C 6C 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(8, highest_score.1)
    }

    #[test]
    fn royal_flush() {
        let (p1, p2) = cards::parse_cards("TC JC QC KC AC 5H 3H AH JS 8H".to_string()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        assert_eq!(20, highest_score.1)
    }
}