use std::io;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let sum = io::stdin()
        .lines()
        .map(|l| l.expect("stdin lock"))
        .map(|l| first_digit(&l) * 10 + last_digit(&l))
        .sum::<usize>();
    println!("{sum}");
}

fn first_digit(s: &str) -> usize {
    let pos = s.find(&DIGITS).expect("first digit position");
    (s.as_bytes()[pos] as usize) - ('0' as usize)
}

fn last_digit(s: &str) -> usize {
    let pos = s.rfind(&DIGITS).expect("last digit position");
    (s.as_bytes()[pos] as usize) - ('0' as usize)
}
