use std::collections::HashMap;

#[derive(Debug)]
struct Test {
    prop: char,
    op: char,
    value: usize,
}

#[derive(Debug)]
struct Rule {
    test: Option<Test>,
    dest: String,
}

fn combos(
    workflows: &HashMap<String, Vec<Rule>>,
    workflow_name: &String,
    ranges: [[usize; 2]; 4],
) -> usize {
    if workflow_name == "A" {
        ranges.iter().map(|r| r[1] - r[0] + 1).product::<usize>()
    } else if workflow_name == "R" || ranges.iter().any(|r| r[0] > r[1]) {
        0
    } else {
        let mut result = 0;
        let mut r = ranges;
        let rules = workflows.get(workflow_name).expect("rules");
        for rule in rules {
            match rule.test {
                Some(Test {
                    prop,
                    op: '<',
                    value,
                }) => {
                    let pi = ['x', 'm', 'a', 's']
                        .iter()
                        .position(|&c| c == prop)
                        .expect("prop index");
                    let mut sub_r = r;
                    sub_r[pi][1] = value - 1;
                    result += combos(workflows, &rule.dest, sub_r);
                    r[pi][0] = value;
                }
                Some(Test {
                    prop,
                    op: '>',
                    value,
                }) => {
                    let pi = ['x', 'm', 'a', 's']
                        .iter()
                        .position(|&c| c == prop)
                        .expect("prop index");
                    let mut sub_r = r;
                    sub_r[pi][0] = value + 1;
                    result += combos(workflows, &rule.dest, sub_r);
                    r[pi][1] = value;
                }
                None => result += combos(workflows, &rule.dest, r),
                _ => panic!("unknown op"),
            }
        }
        result
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(Result::unwrap);

    // Read in workflows.
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    loop {
        let line = lines.next().unwrap_or_else(|| "".to_string());
        if line == "" {
            break;
        }
        let (workflow_name, rules_str) = line
            .split_once('{')
            .map(|(wn, rs)| (wn, rs.strip_suffix('}').expect("workflow '}'")))
            .expect("workflow '{'");
        let rules: Vec<Rule> = rules_str
            .split(',')
            .map(|rule_str| {
                if let Some((test_str, dest)) = rule_str.split_once(':') {
                    if let Some((prop_str, value_str)) = test_str.split_once('<') {
                        Rule {
                            test: Some(Test {
                                prop: prop_str.chars().next().expect("prop char"),
                                op: '<',
                                value: value_str.parse::<usize>().expect("value usize"),
                            }),
                            dest: dest.to_string(),
                        }
                    } else if let Some((prop_str, value_str)) = test_str.split_once('>') {
                        Rule {
                            test: Some(Test {
                                prop: prop_str.chars().next().expect("prop char"),
                                op: '>',
                                value: value_str.parse::<usize>().expect("value usize"),
                            }),
                            dest: dest.to_string(),
                        }
                    } else {
                        panic!("unknown op");
                    }
                } else {
                    Rule {
                        test: None,
                        dest: rule_str.to_string(),
                    }
                }
            })
            .collect::<Vec<Rule>>();
        workflows.insert(workflow_name.to_string(), rules);
    }

    let combo_count = combos(
        &workflows,
        &"in".to_string(),
        [[1, 4000], [1, 4000], [1, 4000], [1, 4000]],
    );
    println!("{combo_count}");
}
