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

    fn points(&self) -> usize {
        let match_count = self
            .have
            .iter()
            .filter(|h| self.win.iter().any(|w| w == *h))
            .count();
        if match_count > 0 {
            1 << (match_count - 1)
        } else {
            0
        }
    }
}

pub fn main() {
    let mut cards: Vec<Card> = Vec::new();

    let lines = io::stdin().lines();
    for line in lines {
        let line = line.unwrap();
        cards.push(Card::from_str(&line));
    }
    println!("{}", cards.iter().map(Card::points).sum::<usize>());
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
