use std::io;

#[derive(Debug)]
struct Card {
    _id: usize,
    win: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let input = input.strip_prefix("Card ").expect("Card");
        let (id, input) = parse_num_prefix(input);
        let id = id.unwrap();
        let input = input.strip_prefix(": ").expect("colon and space");
        let (win_str, have_str) = input.split_once('|').expect("pipe delimiter");
        Self {
            _id: id,
            win: win_str
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
            have: have_str
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        }
    }

    fn count_wins(&self) -> usize {
        self.have
            .iter()
            .filter(|h| self.win.iter().any(|w| w == *h))
            .count()
    }
}

pub fn main() {
    let mut cards: Vec<Card> = Vec::new();

    let lines = io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        cards.push(Card::from_str(&line));
    }

    let mut copies: Vec<usize> = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let card_copies = copies[i];
        let card_wins = c.count_wins();
        for next_copy in copies.iter_mut().skip(i + 1).take(card_wins) {
            *next_copy += card_copies;
        }
    }
    println!("{}", copies.iter().sum::<usize>());
}

fn parse_num_prefix(s: &str) -> (Option<usize>, &str) {
    let s = s.trim_start();
    if s.starts_with(|c| c >= '0' && c <= '9') {
        let num_end = s.find(|c| c < '0' || c > '9').unwrap_or(s.len());
        (s[..num_end].parse::<usize>().ok(), &s[num_end..])
    } else {
        (None, s)
    }
}
