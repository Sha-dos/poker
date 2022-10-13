#[derive(Clone, Copy, Debug)]
pub struct Hand {
    Vec<Card>
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    number: i32
    
}

#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}