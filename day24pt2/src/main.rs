#[derive(Debug)]
struct Line {
    x: i64,
    y: i64,
    z: i64,
    dx: i16,
    dy: i16,
    dz: i16,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let (x, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<i64>().expect("x i64"), s))
            .expect("comma");
        let (y, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<i64>().expect("y i64"), s))
            .expect("comma");
        let (z, s) = s
            .split_once(" @ ")
            .map(|(n, s)| (n.parse::<i64>().expect("z i64"), s))
            .expect("at sign");
        let (dx, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<i16>().expect("dx i16"), s))
            .expect("comma");
        let (dy, s) = s
            .split_once(", ")
            .map(|(n, s)| (n.parse::<i16>().expect("dy i16"), s))
            .expect("comma");
        let dz = s.parse::<i16>().expect("dz i16");
        Self {
            x,
            y,
            z,
            dx,
            dy,
            dz,
        }
    }
}

fn line_intersection(
    ax: i64,
    ay: i64,
    adx: i16,
    ady: i16,
    bx: i64,
    by: i64,
    bdx: i16,
    bdy: i16,
) -> Option<(i64, i64)> {
    let adx = adx as i64;
    let ady = ady as i64;
    let bdx = bdx as i64;
    let bdy = bdy as i64;
    let denom = bdy * adx - bdx * ady;
    if denom == 0 {
        return None;
    }
    let ua = (bdx * (ay - by) - bdy * (ax - bx)) / denom;
    //let ub = (adx * (ay - by) - ady * (ax - bx)) / denom;
    let x = ax + ua * adx;
    let y = ay + ua * ady;
    return Some((x, y));
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| Line::from(&line[..]))
        .collect::<Vec<_>>();
    let mut x_divisors: Vec<i64> = (0..1000).collect::<Vec<_>>();
    let mut y_divisors: Vec<i64> = (0..1000).collect::<Vec<_>>();
    let mut z_divisors: Vec<i64> = (0..1000).collect::<Vec<_>>();
    for a in 0..lines.len() - 1 {
        for b in a + 1..lines.len() {
            if lines[a].dx == lines[b].dx {
                let dx = lines[a].dx as i64;
                let x_diff = lines[a].x.abs_diff(lines[b].x) as i64;
                x_divisors.retain(|&x_div| dx == x_div || x_diff % (x_div - dx) == 0);
            }
            if lines[a].dy == lines[b].dy {
                let dy = lines[a].dy as i64;
                let y_diff = lines[a].y.abs_diff(lines[b].y) as i64;
                y_divisors.retain(|&y_div| dy == y_div || y_diff % (y_div - dy) == 0);
            }
            if lines[a].dz == lines[b].dz {
                let dz = lines[a].dz as i64;
                let z_diff = lines[a].z.abs_diff(lines[b].z) as i64;
                z_divisors.retain(|&z_div| dz == z_div || z_diff % (z_div - dz) == 0);
            }
        }
    }
    println!("{x_divisors:?} {y_divisors:?} {z_divisors:?}");
    assert_eq!(1, x_divisors.len());
    assert_eq!(1, y_divisors.len());
    assert_eq!(1, z_divisors.len());
    let rock_dx = x_divisors[0] as i16;
    let rock_dy = y_divisors[0] as i16;
    let rock_dz = z_divisors[0] as i16;
    let (rock_x1, rock_y) = line_intersection(
        lines[0].x,
        lines[0].y,
        lines[0].dx - rock_dx,
        lines[0].dy - rock_dy,
        lines[1].x,
        lines[1].y,
        lines[1].dx - rock_dx,
        lines[1].dy - rock_dy,
    )
    .expect("xy intersection");
    let (rock_x2, rock_z) = line_intersection(
        lines[0].x,
        lines[0].z,
        lines[0].dx - rock_dx,
        lines[0].dz - rock_dz,
        lines[1].x,
        lines[1].z,
        lines[1].dx - rock_dx,
        lines[1].dz - rock_dz,
    )
    .expect("xz intersection");
    assert_eq!(rock_x1, rock_x2);
    println!("{rock_x1} {rock_y} {rock_z}");
    println!("{}", rock_x1 + rock_y + rock_z);
}
