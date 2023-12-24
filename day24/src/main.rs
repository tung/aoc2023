#[derive(Debug)]
struct Line {
    x: f64,
    y: f64,
    _z: f64,
    dx: f64,
    dy: f64,
    _dz: f64,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (x, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<f64>().expect("x f64"), s))
            .expect("comma");
        let (y, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<f64>().expect("y f64"), s))
            .expect("comma");
        let (z, s) = s
            .split_once(" @ ")
            .map(|(n, s)| (n.parse::<f64>().expect("z f64"), s))
            .expect("at sign");
        let (dx, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<f64>().expect("dx f64"), s))
            .expect("comma");
        let (dy, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<f64>().expect("dy f64"), s))
            .expect("comma");
        let dz = s.parse::<f64>().expect("dz f64");
        Self {
            x,
            y,
            _z: z,
            dx,
            dy,
            _dz: dz,
        }
    }
}

fn line_intersection(a: &Line, b: &Line) -> Option<(f64, f64)> {
    let denom = b.dy * a.dx - b.dx * a.dy;
    if denom == 0.0 {
        return None;
    }
    let ua = (b.dx * (a.y - b.y) - b.dy * (a.x - b.x)) / denom;
    let ub = (a.dx * (a.y - b.y) - a.dy * (a.x - b.x)) / denom;
    if ua < 0.0 || ub < 0.0 {
        return None;
    }
    let x = a.x + ua * a.dx;
    let y = a.y + ua * a.dy;
    return Some((x, y));
}

fn count_intersections(lines: &[Line], min_value: f64, max_value: f64) -> usize {
    let mut count: usize = 0;
    for a in 0..lines.len() - 1 {
        for b in a + 1..lines.len() {
            if let Some((x, y)) = line_intersection(&lines[a], &lines[b]) {
                if x >= min_value && x <= max_value && y >= min_value && y <= max_value {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| Line::from(&line[..]))
        .collect::<Vec<_>>();
    println!(
        "{}",
        count_intersections(&lines[..], 200_000_000_000_000.0, 400_000_000_000_000.0),
    );
}
