use pathfinding::prelude::astar;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn up(p: &Pos) -> Self {
        Self { x: p.x, y: p.y - 1 }
    }

    fn down(p: &Pos) -> Self {
        Self { x: p.x, y: p.y + 1 }
    }

    fn left(p: &Pos) -> Self {
        Self { x: p.x - 1, y: p.y }
    }

    fn right(p: &Pos) -> Self {
        Self { x: p.x + 1, y: p.y }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    pos: Pos,
    dir: Dir,
    steps_left: u8,
}

impl Node {
    fn new(pos: Pos, dir: Dir, steps_left: u8) -> Self {
        Self {
            pos,
            dir,
            steps_left,
        }
    }
}

const MAX_STEPS: u8 = 3;

fn successors(node: &Node, map: &HashMap<Pos, i32>) -> Vec<(Node, i32)> {
    let mut s: Vec<(Node, i32)> = Vec::new();
    match node.dir {
        Dir::U => {
            let left_pos = Pos::left(&node.pos);
            if let Some(&left_cost) = map.get(&left_pos) {
                s.push((Node::new(left_pos, Dir::L, MAX_STEPS - 1), left_cost));
            }
            let right_pos = Pos::right(&node.pos);
            if let Some(&right_cost) = map.get(&right_pos) {
                s.push((Node::new(right_pos, Dir::R, MAX_STEPS - 1), right_cost));
            }
            if node.steps_left > 0 {
                let up_pos = Pos::up(&node.pos);
                if let Some(&up_cost) = map.get(&up_pos) {
                    s.push((Node::new(up_pos, Dir::U, node.steps_left - 1), up_cost));
                }
            }
        }
        Dir::D => {
            let left_pos = Pos::left(&node.pos);
            if let Some(&left_cost) = map.get(&left_pos) {
                s.push((Node::new(left_pos, Dir::L, MAX_STEPS - 1), left_cost));
            }
            let right_pos = Pos::right(&node.pos);
            if let Some(&right_cost) = map.get(&right_pos) {
                s.push((Node::new(right_pos, Dir::R, MAX_STEPS - 1), right_cost));
            }
            if node.steps_left > 0 {
                let down_pos = Pos::down(&node.pos);
                if let Some(&down_cost) = map.get(&down_pos) {
                    s.push((Node::new(down_pos, Dir::D, node.steps_left - 1), down_cost));
                }
            }
        }
        Dir::L => {
            let up_pos = Pos::up(&node.pos);
            if let Some(&up_cost) = map.get(&up_pos) {
                s.push((Node::new(up_pos, Dir::U, MAX_STEPS - 1), up_cost));
            }
            let down_pos = Pos::down(&node.pos);
            if let Some(&down_cost) = map.get(&down_pos) {
                s.push((Node::new(down_pos, Dir::D, MAX_STEPS - 1), down_cost));
            }
            if node.steps_left > 0 {
                let left_pos = Pos::left(&node.pos);
                if let Some(&left_cost) = map.get(&left_pos) {
                    s.push((Node::new(left_pos, Dir::L, node.steps_left - 1), left_cost));
                }
            }
        }
        Dir::R => {
            let up_pos = Pos::up(&node.pos);
            if let Some(&up_cost) = map.get(&up_pos) {
                s.push((Node::new(up_pos, Dir::U, MAX_STEPS - 1), up_cost));
            }
            let down_pos = Pos::down(&node.pos);
            if let Some(&down_cost) = map.get(&down_pos) {
                s.push((Node::new(down_pos, Dir::D, MAX_STEPS - 1), down_cost));
            }
            if node.steps_left > 0 {
                let right_pos = Pos::right(&node.pos);
                if let Some(&right_cost) = map.get(&right_pos) {
                    s.push((
                        Node::new(right_pos, Dir::R, node.steps_left - 1),
                        right_cost,
                    ));
                }
            }
        }
    }
    s
}

fn heuristic(pos: &Pos, goal: &Pos) -> i32 {
    (pos.x.abs_diff(goal.x) + pos.y.abs_diff(goal.y)) as i32
}

fn main() {
    let mut map: HashMap<Pos, i32> = HashMap::new();
    let mut goal = Pos { x: 0, y: 0 };
    for (y, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        if goal.y < y as i32 {
            goal.y = y as i32;
        }
        for (x, c) in line.chars().enumerate() {
            assert!(c >= '0' && c <= '9');
            if goal.x < x as i32 {
                goal.x = x as i32;
            }
            map.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c as i32 - '0' as i32,
            );
        }
    }
    let result_r = astar(
        &Node {
            pos: Pos { x: 0, y: 0 },
            dir: Dir::R,
            steps_left: MAX_STEPS - 1,
        },
        |node| successors(node, &map),
        |node| heuristic(&node.pos, &goal),
        |node| node.pos == goal,
    );
    let result_d = astar(
        &Node {
            pos: Pos { x: 0, y: 0 },
            dir: Dir::D,
            steps_left: MAX_STEPS - 1,
        },
        |node| successors(node, &map),
        |node| heuristic(&node.pos, &goal),
        |node| node.pos == goal,
    );
    println!(
        "{}",
        result_r
            .iter()
            .chain(result_d.iter())
            .map(|(_, cost)| cost)
            .min()
            .expect("path found"),
    );
}
