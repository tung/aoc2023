use std::collections::HashMap;

fn count_arrangements(
    cache: &mut HashMap<(String, Vec<u8>), i64>,
    s: &str,
    damage_seqs: &[u8],
) -> i64 {
    let s = s.trim_start_matches('.');
    let orig_s = s.to_string();
    let orig_damage_seqs = damage_seqs.to_vec();
    if let Some(&result) = cache.get(&(orig_s.clone(), orig_damage_seqs.clone())) {
        return result;
    }
    let count = s
        .strip_prefix('?')
        .map(|s| count_arrangements(cache, s, damage_seqs))
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
            cache.insert((orig_s, orig_damage_seqs), count);
            count
        } else if !damage_seqs.is_empty() {
            if let Some(s_suffix) = s.strip_prefix(&['.', '?']) {
                let result = count + count_arrangements(cache, s_suffix, damage_seqs);
                cache.insert((orig_s, orig_damage_seqs), result);
                result
            } else {
                // a gap must exist between damage sequences
                cache.insert((orig_s, orig_damage_seqs), count);
                count
            }
        } else {
            let result = count + count_arrangements(cache, s, damage_seqs);
            cache.insert((orig_s, orig_damage_seqs), result);
            result
        }
    } else if s.is_empty() {
        // good match: damage_seqs and s are both empty
        let result = count + 1;
        cache.insert((orig_s, orig_damage_seqs), result);
        result
    } else {
        // no match: damage_seqs is empty, but s is not empty
        cache.insert((orig_s, orig_damage_seqs), count);
        count
    }
}

fn main() {
    let sum: i64 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (row, line) = line.split_once(" ").unwrap();
            let row = format!("{row}?{row}?{row}?{row}?{row}");
            let damage_seqs = line
                .split(",")
                .map(|d| d.parse::<u8>().expect("damage u8"))
                .collect::<Vec<u8>>();
            let damage_seqs = damage_seqs
                .iter()
                .cycle()
                .take(damage_seqs.len() * 5)
                .copied()
                .collect::<Vec<u8>>();
            count_arrangements(&mut HashMap::new(), &row, &damage_seqs[..])
        })
        .sum();
    println!("{sum}");
}

#[test]
#[rustfmt::skip]
fn test_count_arrangements() {
    assert_eq!(1, count_arrangements(&mut HashMap::new(), "???.###", &[1, 1, 3]));
    assert_eq!(4, count_arrangements(&mut HashMap::new(), ".??..??...?##", &[1, 1, 3]));
    assert_eq!(1, count_arrangements(&mut HashMap::new(), "?#?#?#?#?#?#?#?", &[1, 3, 1, 6]));
    assert_eq!(1, count_arrangements(&mut HashMap::new(), "????.#...#...", &[4, 1, 1]));
    assert_eq!(4, count_arrangements(&mut HashMap::new(), "????.######..#####.", &[1, 6, 5]));
    assert_eq!(10, count_arrangements(&mut HashMap::new(), "?###????????", &[3, 2, 1]));
}
