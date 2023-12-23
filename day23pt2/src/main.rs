use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' | 'v' | '>' => Ok(Tile::Path),
            '#' => Ok(Tile::Forest),
            _ => Err("unknown char"),
        }
    }
}

fn build_paths(
    start_pos: (i16, i16),
    mut dx: i16,
    mut dy: i16,
    map: &HashMap<(i16, i16), Tile>,
    paths: &mut HashMap<(i16, i16), HashMap<(i16, i16), usize>>,
) {
    let mut pos = start_pos;
    let mut steps: usize = 0;
    loop {
        pos.0 += dx;
        pos.1 += dy;
        steps += 1;
        let next_steps = [(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .filter(|(next_dx, next_dy)| *next_dx != -dx || *next_dy != -dy)
            .map(|(next_dx, next_dy)| (next_dx, next_dy, (pos.0 + next_dx, pos.1 + next_dy)))
            .filter(|(_, _, next_pos)| matches!(map.get(&next_pos), Some(Tile::Path)))
            .collect::<Vec<_>>();
        if next_steps.is_empty() {
            break;
        } else if next_steps.len() == 1 {
            if paths.contains_key(&next_steps[0].2) {
                steps += 1;
                paths.entry(start_pos).and_modify(|path| {
                    path.entry(next_steps[0].2)
                        .and_modify(|length| *length = (*length).max(steps))
                        .or_insert(steps);
                });
                paths
                    .entry(next_steps[0].2)
                    .or_insert_with(|| HashMap::new())
                    .entry(start_pos)
                    .and_modify(|length| *length = (*length).max(steps))
                    .or_insert(steps);
                break;
            } else {
                dx = *next_steps[0].0;
                dy = *next_steps[0].1;
            }
        } else {
            let new_point_of_interest = !paths.contains_key(&pos);
            paths.entry(start_pos).and_modify(|path| {
                path.entry(pos)
                    .and_modify(|length| *length = (*length).max(steps))
                    .or_insert(steps);
            });
            paths
                .entry(pos)
                .or_insert_with(|| HashMap::new())
                .entry(start_pos)
                .and_modify(|length| *length = (*length).max(steps))
                .or_insert(steps);
            if new_point_of_interest {
                for (next_dx, next_dy, _) in next_steps {
                    build_paths(pos, *next_dx, *next_dy, map, paths);
                }
            }
            break;
        }
    }
}

fn longest_hike(
    pos: (i16, i16),
    goal: (i16, i16),
    paths: &HashMap<(i16, i16), HashMap<(i16, i16), usize>>,
    mut visited: HashSet<(i16, i16)>,
) -> Option<usize> {
    if pos == goal {
        return Some(0);
    } else {
        visited.insert(pos);
        return paths[&pos]
            .iter()
            .filter(|(adjacent, _)| !visited.contains(adjacent))
            .map(|(adjacent, length)| {
                longest_hike(*adjacent, goal, paths, visited.clone()).map(|l| l + length)
            })
            .flatten()
            .max();
    }
}

fn main() {
    let mut map: HashMap<(i16, i16), Tile> = HashMap::new();
    let mut start_pos: Option<(i16, i16)> = None;
    let mut goal_pos: (i16, i16) = (0, 0);
    for (y, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i16, y as i16);
            let tile = Tile::try_from(c).unwrap();
            if matches!(tile, Tile::Path) {
                start_pos.get_or_insert(pos);
                goal_pos = pos;
            }
            map.insert(pos, tile);
        }
    }
    let start_pos = start_pos.expect("start pos");
    let mut paths: HashMap<(i16, i16), HashMap<(i16, i16), usize>> = HashMap::new();
    paths.insert(start_pos, HashMap::new());
    paths.insert(goal_pos, HashMap::new());
    build_paths(start_pos, 0, 1, &map, &mut paths);
    println!(
        "{}",
        longest_hike(start_pos, goal_pos, &paths, HashSet::new()).expect("path from start to goal"),
    );
}
