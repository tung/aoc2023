use std::io;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUM_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let sum = io::stdin()
        .lines()
        .map(|l| l.expect("stdin lock"))
        //.map(|l| {
        //    println!("{} {}", first_number(&l), last_number(&l));
        //    l
        //})
        .map(|l| first_number(&l) * 10 + last_number(&l))
        .sum::<usize>();
    println!("{sum}");
}

fn first_number(s: &str) -> usize {
    s.find(&DIGITS)
        .map(|digit_pos| {
            (
                digit_pos,
                (s.as_bytes()[digit_pos] as usize) - ('0' as usize),
            )
        })
        .iter()
        .copied()
        .chain(
            NUM_WORDS
                .iter()
                .enumerate()
                .flat_map(|(i, num_word)| s.find(num_word).map(|pos| (pos, i))),
        )
        .min_by_key(|(pos, _)| *pos)
        .map(|(_, value)| value)
        .expect("first number")
}

fn last_number(s: &str) -> usize {
    s.rfind(&DIGITS)
        .map(|digit_pos| {
            (
                digit_pos,
                (s.as_bytes()[digit_pos] as usize) - ('0' as usize),
            )
        })
        .iter()
        .copied()
        .chain(
            NUM_WORDS
                .iter()
                .enumerate()
                .flat_map(|(i, num_word)| s.rfind(num_word).map(|pos| (pos, i))),
        )
        .max_by_key(|(pos, _)| *pos)
        .map(|(_, value)| value)
        .expect("last number")
}
