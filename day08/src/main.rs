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

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    let instructions = lines.next().expect("first line: instructions");

    lines.next().expect("second line: blank");

    let network: HashMap<String, (String, String)> = lines
        .map(|line| parse_node_line(&line))
        .collect::<HashMap<String, (String, String)>>(
    );

    let mut instructions = instructions.chars().cycle();
    let mut position = network.get_key_value("AAA").expect("AAA").0;
    let mut steps: usize = 0;
    while position != "ZZZ" {
        match instructions.next() {
            Some('L') => {
                position = &network.get(position).expect("left pos").0;
                steps += 1;
            }
            Some('R') => {
                position = &network.get(position).expect("left pos").1;
                steps += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("{steps}");
}

#[test]
#[rustfmt::skip]
fn test_parse_node_line() {
    assert_eq!(
        (String::from("AAA"), (String::from("BBB"), String::from("CCC"))),
        parse_node_line("AAA = (BBB, CCC)")
    );
}
