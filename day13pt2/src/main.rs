fn reflects_at_row(rows: &[String], r: usize) -> bool {
    if r >= rows.len() - 1 {
        return false;
    }
    let mut mismatches: usize = 0;
    for i in 0..(r + 1).min(rows.len() - r - 1) {
        let above = r - i;
        let below = r + i + 1;
        if rows[above] != rows[below] {
            for (ca, cb) in rows[above].chars().zip(rows[below].chars()) {
                if ca != cb {
                    mismatches += 1;
                }
            }
            if mismatches > 1 {
                break;
            }
        }
    }
    mismatches == 1
}

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

fn main() {
    let mut maps: Vec<Vec<String>> = Vec::new();
    let mut rows: Vec<String> = Vec::new();
    for line in std::io::stdin().lines().map(Result::unwrap) {
        if line.is_empty() && !rows.is_empty() {
            maps.push(rows);
            rows = Vec::new();
        }
        if !line.is_empty() {
            rows.push(line.to_string());
        }
    }
    if !rows.is_empty() {
        maps.push(rows);
    }

    let sum = maps
        .iter()
        .map(|rows| {
            let reflect_row = (0..rows.len())
                .position(|r| reflects_at_row(rows, r))
                .map(|r| r + 1)
                .unwrap_or(0);
            let t_rows = transpose(rows);
            let reflect_column = (0..t_rows.len())
                .position(|r| reflects_at_row(&t_rows, r))
                .map(|r| r + 1)
                .unwrap_or(0);
            reflect_row * 100 + reflect_column
        })
        .sum::<usize>();
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
