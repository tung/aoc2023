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

        Self {
            modules,
            broadcast_index,
        }
    }

    fn push_button(&mut self) -> (usize, usize) {
        let mut lows: usize = 0;
        let mut highs: usize = 0;
        let mut queue: VecDeque<Message> = VecDeque::new();
        queue.push_back(Message {
            src: 0,
            dest: self.broadcast_index,
            pulse: false,
        });

        while let Some(msg) = queue.pop_front() {
            if msg.pulse {
                highs += 1;
            } else {
                lows += 1;
            }
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

        (lows, highs)
    }
}

fn main() {
    let mut network = Network::new(
        std::io::stdin()
            .lines()
            .map(Result::unwrap)
            .collect::<Vec<String>>()
            .as_slice(),
    );
    let mut low_pulses: usize = 0;
    let mut high_pulses: usize = 0;
    for _ in 0..1000 {
        let (lows, highs) = network.push_button();
        low_pulses += lows;
        high_pulses += highs;
    }
    println!("{}", low_pulses * high_pulses);
}
