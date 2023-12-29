use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Step {
    pos: (i16, i16),
    dx: i16,
    dy: i16,
    steps_taken: i16,
}

fn find_path_cost(
    map: &HashMap<(i16, i16), u8>,
    width: i16,
    height: i16,
    min_steps: i16,
    max_steps: i16,
) -> Option<u16> {
    let start = (0i16, 0i16);
    let goal = (width.saturating_sub(1), height.saturating_sub(1));
    let mut seen: HashSet<Step> = HashSet::new();
    let mut heap: BinaryHeap<(Reverse<u16>, Step)> = BinaryHeap::new();

    heap.push((
        Reverse(0),
        Step {
            pos: start,
            dx: 1,
            dy: 0,
            steps_taken: 0,
        },
    ));
    heap.push((
        Reverse(0),
        Step {
            pos: start,
            dx: 0,
            dy: 1,
            steps_taken: 0,
        },
    ));

    while let Some(current) = heap.pop() {
        if current.1.steps_taken >= min_steps {
            if current.1.pos == goal {
                return Some(current.0 .0);
            }

            let left_step = Step {
                pos: (
                    current.1.pos.0 + current.1.dy,
                    current.1.pos.1 - current.1.dx,
                ),
                dx: current.1.dy,
                dy: -current.1.dx,
                steps_taken: 1,
            };
            if !seen.contains(&left_step) {
                if let Some(cost) = map.get(&left_step.pos) {
                    seen.insert(left_step);
                    heap.push((Reverse(current.0 .0 + *cost as u16), left_step));
                }
            }

            let right_step = Step {
                pos: (
                    current.1.pos.0 - current.1.dy,
                    current.1.pos.1 + current.1.dx,
                ),
                dx: -current.1.dy,
                dy: current.1.dx,
                steps_taken: 1,
            };
            if !seen.contains(&right_step) {
                if let Some(cost) = map.get(&right_step.pos) {
                    seen.insert(right_step);
                    heap.push((Reverse(current.0 .0 + *cost as u16), right_step));
                }
            }
        }

        if current.1.steps_taken < max_steps {
            let forward_step = Step {
                pos: (
                    current.1.pos.0 + current.1.dx,
                    current.1.pos.1 + current.1.dy,
                ),
                dx: current.1.dx,
                dy: current.1.dy,
                steps_taken: current.1.steps_taken + 1,
            };
            if !seen.contains(&forward_step) {
                if let Some(cost) = map.get(&forward_step.pos) {
                    seen.insert(forward_step);
                    heap.push((Reverse(current.0 .0 + *cost as u16), forward_step));
                }
            }
        }
    }

    None
}

fn main() {
    let map = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (
                            i16::try_from(x).expect("i16 x"),
                            i16::try_from(y).expect("i16 y"),
                        ),
                        c.to_digit(10).expect("digit c") as u8,
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(i16, i16), u8>>();
    let width = map.keys().map(|(x, _)| *x).max().expect("map width") + 1;
    let height = map.keys().map(|(_, y)| *y).max().expect("map height") + 1;
    let heat_loss = find_path_cost(&map, width, height, 4, 10).expect("path cost");
    println!("{heat_loss}");
}
