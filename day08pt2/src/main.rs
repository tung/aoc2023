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

fn prime_factors(mut value: usize) -> Vec<(u32, usize)> {
    let mut result: Vec<(u32, usize)> = vec![];
    let mut prime = 2;
    while value > 1 {
        let mut times = 0;
        while value % prime == 0 {
            times += 1;
            value /= prime;
        }
        result.push((times, prime));
        for i in prime + 1.. {
            if !result.iter().any(|(_, p)| i % p == 0) {
                prime = i;
                break;
            }
        }
    }
    result
}

fn lcm<I: Iterator<Item = usize>>(values: I) -> usize {
    let mut result: usize = 1;
    let factors = values
        .map(prime_factors)
        .collect::<Vec<Vec<_>>>();
    for i in 0..factors.iter().map(Vec::len).max().unwrap_or(0) {
        let (max_factor, prime) = factors
            .iter()
            .map(|fs| fs.get(i).copied().unwrap_or((0, 0)))
            .max()
            .unwrap_or((0, 0));
        if max_factor > 0 {
            result *= prime.pow(max_factor);
        }
    }
    result
}

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    let instructions = lines.next().expect("first line: instructions");

    lines.next().expect("second line: blank");

    let network: HashMap<String, (String, String)> = lines
        .map(|line| parse_node_line(&line))
        .collect::<HashMap<String, (String, String)>>(
    );

    let result = lcm(network
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| count_steps(&instructions, k, &network)));
    println!("{result}");
}

#[test]
#[rustfmt::skip]
fn test_parse_node_line() {
    assert_eq!(
        (String::from("AAA"), (String::from("BBB"), String::from("CCC"))),
        parse_node_line("AAA = (BBB, CCC)")
    );
}

#[test]
#[rustfmt::skip]
fn test_prime_factors() {
    assert_eq!(vec![(2, 2), (1, 3)], prime_factors(12));
    assert_eq!(vec![(2, 2), (1, 3), (0, 5), (1, 7)], prime_factors(84));
}

#[test]
#[rustfmt::skip]
fn test_lcm() {
    assert_eq!(504, lcm([8, 9, 21].iter().copied()));
}
