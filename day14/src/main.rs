fn transpose(rows: &[String]) -> Vec<String> {
    let mut new_rows: Vec<String> = Vec::new();
    let mut char_iters = rows
        .iter()
        .map(|s| s.chars())
        .collect::<Vec<std::str::Chars>>();
    loop {
        let row = char_iters
            .iter_mut()
            .map(Iterator::next)
            .flatten()
            .collect::<String>();
        if row == "" {
            break;
        } else {
            new_rows.push(row);
        }
    }
    new_rows
}

fn calc_load(s: &str) -> usize {
    let mut load: usize = 0;
    let mut last_empty: usize = 0;
    for (i, c) in s.chars().enumerate() {
        if c == '#' {
            last_empty = i + 1;
        } else if c == 'O' {
            load += s.len() - last_empty;
            last_empty += 1;
        }
    }
    load
}

fn main() {
    let transposed = transpose(
        &std::io::stdin()
            .lines()
            .map(Result::unwrap)
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
    );
    let sum: usize = transposed.iter().map(|s| calc_load(s)).sum();
    println!("{sum}");
}

#[test]
#[rustfmt::skip]
fn test_transpose() {
    let input = &[
        "###",
        ".#.",
        "...",
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let expected = &[
        "#..",
        "##.",
        "#..",
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    assert_eq!(expected, &transpose(&input));
}

#[test]
#[rustfmt::skip]
fn test_calc_load() {
    assert_eq!(0, calc_load(""));
    assert_eq!(1, calc_load("O"));
    assert_eq!(2, calc_load("O."));
    assert_eq!(2, calc_load(".O"));
    assert_eq!(3, calc_load("O.."));
    assert_eq!(3, calc_load(".O."));
    assert_eq!(3, calc_load("..O"));
    assert_eq!(3, calc_load("OO"));
    assert_eq!(5, calc_load("OO."));
    assert_eq!(6, calc_load("OOO"));
    assert_eq!(1, calc_load("#O"));
    assert_eq!(1, calc_load(".#O"));
    assert_eq!(1, calc_load("......#O"));
    assert_eq!(9, calc_load("#O#O#O"));
    assert_eq!(15, calc_load("#O.#O.#O."));
    assert_eq!(15, calc_load("#.O#.O#.O"));
}
