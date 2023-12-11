#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn main() {
    let mut galaxies = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point::new(x as i64, y as i64))
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Point>>();

    // Expand along the x axis.
    let max_x = galaxies.iter().map(|p| p.x).max().expect("max_x");
    let x_gaps = (0..max_x)
        .filter(|&x| !galaxies.iter().any(|g| x == g.x))
        .collect::<Vec<i64>>();
    for galaxy in galaxies.iter_mut() {
        galaxy.x += x_gaps.iter().filter(|&&x| galaxy.x > x).count() as i64 * 999_999;
    }

    // Expand along the y axis.
    let max_y = galaxies.iter().map(|p| p.y).max().expect("max_y");
    let y_gaps = (0..max_y)
        .filter(|&y| !galaxies.iter().any(|g| y == g.y))
        .collect::<Vec<i64>>();
    for galaxy in galaxies.iter_mut() {
        galaxy.y += y_gaps.iter().filter(|&&y| galaxy.y > y).count() as i64 * 999_999;
    }

    let mut sum: u64 = 0;
    for second in (1..galaxies.len()).rev() {
        for first in 0..second {
            sum += galaxies[second].x.abs_diff(galaxies[first].x)
                + galaxies[second].y.abs_diff(galaxies[first].y);
        }
    }
    println!("{sum}");
}
