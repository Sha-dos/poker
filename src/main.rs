mod cards;
mod poker_error;
mod logic;

use std::{fs::File, io::{self, BufRead}, path::Path};

use logic::HandType;

use crate::logic::Player;

fn main() {
    let mut total_p1 = 0;
    let mut total_p2 = 0;

    let lines = read_lines("src\\poker.txt").unwrap();

    for line in lines {
        let (p1, p2) = cards::parse_cards(line.unwrap()).unwrap();

        let score_vectors: (Vec<HandType>, Vec<HandType>) = logic::get_score_vectors(&p1, &p2);

        let highest_score = logic::get_highest_score(score_vectors.0, logic::get_highest(&mut p1.clone()), score_vectors.1, logic::get_highest(&mut p2.clone()));

        //println!("Score: {} - {} Winner: {:?}, Because: {:?}", highest_score.1, highest_score.2, highest_score.0, highest_score.3);

        if highest_score.0 == Player::Player1 { total_p1 += 1; }
        else if highest_score.0 == Player::Player2 { total_p2 += 1; }
    }

    println!("P1: {}, P2: {}", total_p1, total_p2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}