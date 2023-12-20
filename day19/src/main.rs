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

    // Read in parts.
    let parts: Vec<[usize; 4]> = lines
        .map(|line| {
            let mut props = line.split(',').map(|p| {
                p.trim_matches(|c: char| !c.is_ascii_digit())
                    .parse::<usize>()
                    .expect("prop usize")
            });
            [
                props.next().expect("prop x"),
                props.next().expect("prop m"),
                props.next().expect("prop a"),
                props.next().expect("prop s"),
            ]
        })
        .collect::<Vec<[usize; 4]>>();

    let rating = parts
        .iter()
        .map(|part| {
            let mut workflow_name = &"in".to_string();
            while let Some(workflow) = workflows.get(workflow_name) {
                for rule in workflow {
                    let test_result = match rule.test {
                        Some(Test {
                            prop,
                            op: '<',
                            value,
                        }) => {
                            let pi = ['x', 'm', 'a', 's']
                                .iter()
                                .position(|&c| c == prop)
                                .expect("prop index");
                            part[pi] < value
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
                            part[pi] > value
                        }
                        _ => true,
                    };
                    if test_result {
                        workflow_name = &rule.dest;
                        break;
                    }
                }
            }
            if workflow_name == "A" {
                part.iter().sum::<usize>()
            } else {
                assert!(workflow_name == "R");
                0
            }
        })
        .sum::<usize>();
    println!("{rating}");
}
