use std::io;

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let input = input.strip_prefix("Game ").expect("Game");
        let (id_str, input) = input.split_once(|ch| ch < '0' || ch > '9').expect("id str");
        let id = id_str.parse::<usize>().expect("id usize");

        let mut draws = Vec::new();
        for draw_str in input.split(';') {
            draws.push(Draw::from_str(draw_str));
        }

        Self { id, draws }
    }

    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.draws.iter().all(|d| d.is_possible(r, g, b))
    }
}

#[derive(Debug)]
struct Draw {
    r: usize,
    g: usize,
    b: usize,
}

impl Draw {
    fn from_str(input: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for color_str in input.split(',') {
            let (count_str, color_str) = color_str
                .trim()
                .split_once(char::is_whitespace)
                .expect("count and color");
            let count = count_str.parse::<usize>().expect("count");
            if color_str.starts_with("red") {
                r = count;
            } else if color_str.starts_with("green") {
                g = count;
            } else if color_str.starts_with("blue") {
                b = count;
            } else {
                panic!("color_str = {color_str}");
            }
        }

        Self { r, g, b }
    }

    fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.r <= r && self.g <= g && self.b <= b
    }
}

pub fn main() {
    let mut games: Vec<Game> = Vec::new();
    let lines = io::stdin().lines();
    for line in lines {
        games.push(Game::from_str(line.expect("line").as_str()));
    }

    println!(
        "{}",
        games
            .iter()
            .filter(|g| g.is_possible(12, 13, 14))
            .map(|g| g.id)
            .sum::<usize>(),
    );
}
