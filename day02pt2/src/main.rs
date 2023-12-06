use std::io;

#[derive(Debug)]
struct Game {
    draws: Vec<Draw>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let input = input.strip_prefix("Game ").expect("Game");
        let (id_str, input) = input.split_once(|ch| ch < '0' || ch > '9').expect("id str");
        let _id = id_str.parse::<usize>().expect("id usize");

        let mut draws = Vec::new();
        for draw_str in input.split(';') {
            draws.push(Draw::from_str(draw_str));
        }

        Self { draws }
    }

    fn power(&self) -> usize {
        let max_r = self.draws.iter().map(|d| d.r).max().expect("max r");
        let max_g = self.draws.iter().map(|d| d.g).max().expect("max g");
        let max_b = self.draws.iter().map(|d| d.b).max().expect("max b");
        max_r * max_g * max_b
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
}

pub fn main() {
    let mut games: Vec<Game> = Vec::new();
    let lines = io::stdin().lines();
    for line in lines {
        games.push(Game::from_str(line.expect("line").as_str()));
    }

    println!("{}", games.iter().map(Game::power).sum::<usize>());
}
