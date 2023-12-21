use std::collections::HashSet;

fn take_step(
    garden: &HashSet<(usize, usize)>,
    w: usize,
    h: usize,
    positions: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
    for pos in positions {
        if pos.0 > 0 && !garden.contains(&(pos.0 - 1, pos.1)) {
            new_positions.insert((pos.0 - 1, pos.1));
        }
        if pos.0 < w - 1 && !garden.contains(&(pos.0 + 1, pos.1)) {
            new_positions.insert((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 && !garden.contains(&(pos.0, pos.1 - 1)) {
            new_positions.insert((pos.0, pos.1 - 1));
        }
        if pos.1 < h - 1 && !garden.contains(&(pos.0, pos.1 + 1)) {
            new_positions.insert((pos.0, pos.1 + 1));
        }
    }
    new_positions
}

fn main() {
    let mut garden: HashSet<(usize, usize)> = HashSet::new();
    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let mut garden_width: usize = 0;
    let mut garden_height: usize = 0;
    for (y, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        garden_height += 1;
        for (x, c) in line.chars().enumerate() {
            if garden_height == 1 {
                garden_width += 1;
            }
            if c == '#' {
                garden.insert((x, y));
            }
            if c == 'S' {
                positions.insert((x, y));
            }
        }
    }
    for _ in 0..64 {
        positions = take_step(&garden, garden_width, garden_height, &positions);
    }
    println!("{}", positions.len());
}
