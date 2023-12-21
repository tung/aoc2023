use std::collections::VecDeque;

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(Vec<(usize, bool)>),
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<usize>,
}

#[derive(Debug)]
struct Message {
    src: usize,
    dest: usize,
    pulse: bool,
}

struct Network {
    modules: Vec<Module>,
    broadcast_index: usize,
    final_index: usize,
}

impl Network {
    fn new(lines: &[String]) -> Self {
        // create modules
        let mut modules: Vec<Module> = lines
            .iter()
            .map(|line| Module {
                module_type: match line.chars().next() {
                    Some('%') => ModuleType::FlipFlop(false),
                    Some('&') => ModuleType::Conjunction(Vec::new()),
                    Some('b') => ModuleType::Broadcast,
                    _ => panic!("module type"),
                },
                outputs: Vec::new(),
            })
            .collect::<Vec<Module>>();

        // gather valid module names
        let module_names = lines
            .iter()
            .map(|line| {
                line.trim_start_matches(&['%', '&'])
                    .split_once(' ')
                    .expect("space after module name")
                    .0
                    .to_string()
            })
            .collect::<Vec<String>>();

        // assign outputs
        for (module, line) in modules.iter_mut().zip(lines.iter()) {
            module
                .outputs
                .extend(
                    line.split_once(" -> ")
                        .expect("->")
                        .1
                        .split(", ")
                        .map(|output_name| {
                            module_names
                                .iter()
                                .position(|name| name == output_name)
                                .unwrap_or(module_names.len())
                        }),
                );
        }

        // initialize inputs for conjunction modules
        for i in 0..modules.len() {
            for o in 0..modules[i].outputs.len() {
                let dest = modules[i].outputs[o];
                if let Some(Module {
                    module_type: ModuleType::Conjunction(inputs),
                    ..
                }) = modules.get_mut(dest)
                {
                    inputs.push((i, false));
                }
            }
        }

        let broadcast_index = modules
            .iter()
            .position(|m| matches!(m.module_type, ModuleType::Broadcast))
            .expect("broadcast index");

        let final_index = modules
            .iter()
            .position(|m| m.outputs.iter().any(|&o| o == modules.len()))
            .expect("final index");

        Self {
            modules,
            broadcast_index,
            final_index,
        }
    }

    fn push_button(&mut self) -> Vec<usize> {
        let mut cycles: Vec<usize> = Vec::new();

        for broadcast_out_index in 0..self.modules[self.broadcast_index].outputs.len() {
            let mut pushes: usize = 0;

            'push_loop: loop {
                pushes += 1;
                let mut queue: VecDeque<Message> = VecDeque::new();
                queue.push_back(Message {
                    src: self.broadcast_index,
                    dest: self.modules[self.broadcast_index].outputs[broadcast_out_index],
                    pulse: false,
                });

                while let Some(msg) = queue.pop_front() {
                    if msg.dest >= self.modules.len() {
                        continue;
                    }
                    match &mut self.modules[msg.dest].module_type {
                        ModuleType::Broadcast => {}
                        ModuleType::FlipFlop(state) => {
                            if !msg.pulse {
                                *state = !*state;
                            }
                        }
                        ModuleType::Conjunction(inputs) => {
                            let input_memory = inputs
                                .iter_mut()
                                .find(|(i, _)| *i == msg.src)
                                .expect("input memory for src in conjunction module");
                            input_memory.1 = msg.pulse;
                        }
                    }
                    if msg.pulse && msg.dest == self.final_index {
                        cycles.push(pushes);
                        break 'push_loop;
                    }
                    for o in &self.modules[msg.dest].outputs {
                        match &self.modules[msg.dest].module_type {
                            ModuleType::Broadcast => {
                                queue.push_back(Message {
                                    src: msg.dest,
                                    dest: *o,
                                    pulse: msg.pulse,
                                });
                            }
                            ModuleType::FlipFlop(state) => {
                                if !msg.pulse {
                                    queue.push_back(Message {
                                        src: msg.dest,
                                        dest: *o,
                                        pulse: *state,
                                    });
                                }
                            }
                            ModuleType::Conjunction(inputs) => {
                                queue.push_back(Message {
                                    src: msg.dest,
                                    dest: *o,
                                    pulse: !inputs.iter().all(|(_, m)| *m),
                                });
                            }
                        }
                    }
                }
            }
        }

        cycles
    }
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
    let factors = values.map(prime_factors).collect::<Vec<Vec<_>>>();
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
    let mut network = Network::new(
        std::io::stdin()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>()
            .as_slice(),
    );
    let cycles = network.push_button();
    println!("{}", lcm(cycles.iter().copied()));
}
