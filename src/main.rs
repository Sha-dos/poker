mod cards;
mod poker_error;

fn main() {
    cards::parse_cards("8C TS KC 9H 4S 7D 2S 5D 3S AC".to_string());
}