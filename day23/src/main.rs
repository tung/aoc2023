use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Tile::*;
        match c {
            '.' => Ok(Path),
            '#' => Ok(Forest),
            '^' => Ok(SlopeUp),
            'v' => Ok(SlopeDown),
            '<' => Ok(SlopeLeft),
            '>' => Ok(SlopeRight),
            _ => Err("unknown char"),
        }
    }
}

fn longest_hike(
    mut pos: (i16, i16),
    mut path_so_far: HashSet<(i16, i16)>,
    map: &HashMap<(i16, i16), Tile>,
    goal: (i16, i16),
) -> usize {
    if pos == goal {
        return path_so_far.len();
    }
    let mut next_steps: Vec<(i16, i16)> = Vec::new();
    loop {
        path_so_far.insert(pos);
        next_steps.clear();
        next_steps.extend(
            [(0, -1), (0, 1), (-1, 0), (1, 0)]
                .iter()
                .map(|(dx, dy)| (dx, dy, (pos.0 + dx, pos.1 + dy)))
                .filter_map(|(dx, dy, next_step)| {
                    if !path_so_far.contains(&next_step)
                        && match map.get(&next_step) {
                            Some(Tile::Path) => true,
                            Some(Tile::Forest) => false,
                            Some(Tile::SlopeUp) if *dy != 1 => true,
                            Some(Tile::SlopeDown) if *dy != -1 => true,
                            Some(Tile::SlopeLeft) if *dx != 1 => true,
                            Some(Tile::SlopeRight) if *dx != -1 => true,
                            _ => false,
                        }
                    {
                        Some(next_step)
                    } else {
                        None
                    }
                }),
        );
        if next_steps.len() == 1 {
            if next_steps[0] == goal {
                return path_so_far.len();
            } else {
                pos = next_steps[0];
            }
        } else {
            return next_steps
                .iter()
                .copied()
                .map(|pos| longest_hike(pos, path_so_far.clone(), &map, goal))
                .max()
                .unwrap_or(0);
        }
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
    //println!("{map:?}\n{start_pos:?}\n{goal_pos:?}");
    println!(
        "{}",
        longest_hike(start_pos, HashSet::new(), &map, goal_pos)
    );
}
