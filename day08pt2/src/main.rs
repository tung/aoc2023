use std::collections::HashMap;

fn parse_node_line(line: &str) -> (String, (String, String)) {
    let mut tokens = line.split_whitespace();
    let node_name = tokens.next().expect("node name").to_string();
    tokens.next().expect("=");
    let left_path = tokens
        .next()
        .expect("left path")
        .strip_prefix("(")
        .expect("left paren prefix")
        .strip_suffix(",")
        .expect("left comma suffix")
        .to_string();
    let right_path = tokens
        .next()
        .expect("right path")
        .strip_suffix(")")
        .expect("right paren suffix")
        .to_string();
    (node_name, (left_path, right_path))
}

fn count_steps(
    instructions: &str,
    start: &str,
    network: &HashMap<String, (String, String)>,
) -> usize {
    let mut instructions = instructions.chars().cycle();
    let mut position = network.get_key_value(start).expect("start in network").0;
    let mut steps: usize = 0;
    while !position.ends_with('Z') {
        match instructions.next() {
            Some('L') => {
                position = &network.get(position).expect("L").0;
                steps += 1;
            }
            Some('R') => {
                position = &network.get(position).expect("L").1;
                steps += 1;
            }
            _ => unreachable!(),
        }
    }
    steps
}

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    let instructions = lines.next().expect("first line: instructions");

    lines.next().expect("second line: blank");

    let network: HashMap<String, (String, String)> = lines
        .map(|line| parse_node_line(&line))
        .collect::<HashMap<String, (String, String)>>(
    );

    let path_lengths = network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| count_steps(&instructions, k, &network))
        .collect::<Vec<usize>>();
    let mut path_multipliers: Vec<usize> = vec![1; path_lengths.len()];
    loop {
        let all_equal = {
            let first_product = path_lengths[0] * path_multipliers[0];
            path_lengths
                .iter()
                .zip(path_multipliers.iter())
                .all(|(l, m)| l * m == first_product)
        };
        if all_equal {
            break;
        }
        let lowest_pos = path_lengths
            .iter()
            .zip(path_multipliers.iter())
            .enumerate()
            .min_by_key(|(_, (&l, &m))| l * m)
            .expect("lowest length * multiplier")
            .0;
        path_multipliers[lowest_pos] += 1;
        if lowest_pos == 0 && path_multipliers[0] % 10000000 == 0 {
            println!("{}", path_multipliers[0]);
        }
    }
    println!("{}", path_lengths[0] * path_multipliers[0]);
}

#[test]
#[rustfmt::skip]
fn test_parse_node_line() {
    assert_eq!(
        (String::from("AAA"), (String::from("BBB"), String::from("CCC"))),
        parse_node_line("AAA = (BBB, CCC)")
    );
}
