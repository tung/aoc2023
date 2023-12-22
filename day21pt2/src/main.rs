use std::collections::HashSet;
use std::io::Write;

fn take_step(
    garden: &HashSet<(isize, isize)>,
    w: isize,
    h: isize,
    positions: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let mut new_positions: HashSet<(isize, isize)> = HashSet::new();
    for pos in positions {
        let mut x = pos.0;
        let mut left_x = pos.0 - 1;
        let mut right_x = pos.0 + 1;
        let mut y = pos.1;
        let mut up_y = pos.1 - 1;
        let mut down_y = pos.1 + 1;
        while x < 0 {
            x += w;
        }
        while left_x < 0 {
            left_x += w;
        }
        while right_x < 0 {
            right_x += w;
        }
        while y < 0 {
            y += h;
        }
        while up_y < 0 {
            up_y += h;
        }
        while down_y < 0 {
            down_y += h;
        }
        x %= w;
        left_x %= w;
        right_x %= w;
        y %= h;
        up_y %= h;
        down_y %= h;
        if !garden.contains(&(left_x, y)) {
            new_positions.insert((pos.0 - 1, pos.1));
        }
        if !garden.contains(&(right_x, y)) {
            new_positions.insert((pos.0 + 1, pos.1));
        }
        if !garden.contains(&(x, up_y)) {
            new_positions.insert((pos.0, pos.1 - 1));
        }
        if !garden.contains(&(x, down_y)) {
            new_positions.insert((pos.0, pos.1 + 1));
        }
    }
    new_positions
}

fn main() {
    let mut garden: HashSet<(isize, isize)> = HashSet::new();
    let mut positions: HashSet<(isize, isize)> = HashSet::new();
    let mut garden_width: isize = 0;
    let mut garden_height: isize = 0;
    for (y, line) in std::io::stdin().lines().map(Result::unwrap).enumerate() {
        garden_height += 1;
        for (x, c) in line.chars().enumerate() {
            if garden_height == 1 {
                garden_width += 1;
            }
            if c == '#' {
                garden.insert((x as isize, y as isize));
            }
            if c == 'S' {
                positions.insert((x as isize, y as isize));
            }
        }
    }
    println!("making quadratic with points for x = 0, 1, 2...");
    let mut samples: Vec<isize> = Vec::new();
    for i in 1..=garden_width / 2 + garden_width * 2 {
        positions = take_step(&garden, garden_width, garden_height, &positions);
        if i == garden_width / 2
            || i == garden_width / 2 + garden_width
            || i == garden_width / 2 + garden_width * 2
        {
            samples.push(positions.len() as isize);
            print!("{} ", positions.len());
            let _ = std::io::stdout().flush();
        }
    }
    println!();
    assert_eq!(3, samples.len());
    let a = (samples[0] - 2 * samples[1] + samples[2]) / 2;
    let b = (-3 * samples[0] + 4 * samples[1] - samples[2]) / 2;
    let c = samples[0];
    let n = 26_501_365 / garden_width;
    println!("{}", a * n * n + b * n + c);
}
