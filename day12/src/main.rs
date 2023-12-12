fn count_arrangements(s: &str, damage_seqs: &[usize]) -> i32 {
    let s = s.trim_start_matches('.');
    let count = s
        .strip_prefix('?')
        .map(|s| count_arrangements(s, damage_seqs))
        .unwrap_or(0);
    if let Some((&d, damage_seqs)) = damage_seqs.split_first() {
        let mut d = d;
        let mut s = s;
        while d > 0 {
            if let Some(s_suffix) = s.strip_prefix(&['#', '?']) {
                s = s_suffix;
                d -= 1;
            } else {
                break;
            }
        }
        if d > 0 {
            // not enough damage to match
            count
        } else if !damage_seqs.is_empty() {
            if let Some(s_suffix) = s.strip_prefix(&['.', '?']) {
                count + count_arrangements(s_suffix, damage_seqs)
            } else {
                // a gap must exist between damage sequences
                count
            }
        } else {
            count + count_arrangements(s, damage_seqs)
        }
    } else if s.is_empty() {
        // good match: damage_seqs and s are both empty
        count + 1
    } else {
        // no match: damage_seqs is empty, but s is not empty
        count
    }
}

fn main() {
    let sum: i32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (row, line) = line.split_once(" ").unwrap();
            let damage_seqs = line
                .split(",")
                .map(|d| d.parse::<usize>().expect("damage usize"))
                .collect::<Vec<usize>>();
            count_arrangements(row, &damage_seqs[..])
        })
        .sum();
    println!("{sum}");
}

#[test]
#[rustfmt::skip]
fn test_count_arrangements() {
    assert_eq!(1, count_arrangements("???.###", &[1, 1, 3]));
    assert_eq!(4, count_arrangements(".??..??...?##", &[1, 1, 3]));
    assert_eq!(1, count_arrangements("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]));
    assert_eq!(1, count_arrangements("????.#...#...", &[4, 1, 1]));
    assert_eq!(4, count_arrangements("????.######..#####.", &[1, 6, 5]));
    assert_eq!(10, count_arrangements("?###????????", &[3, 2, 1]));
}
