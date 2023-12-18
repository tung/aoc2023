fn main() {
    let mut pos: (i64, i64) = (0, 0);
    let mut points: Vec<(i64, i64)> = vec![pos];
    let mut perimeter: i64 = 0;
    for line in std::io::stdin().lines().map(Result::unwrap) {
        let line = line
            .trim_start_matches(|c| c != '#')
            .strip_prefix('#')
            .expect("#")
            .strip_suffix(')')
            .expect(")");
        let (hex_len, dir) = line.split_at(5);
        let len = i64::from_str_radix(hex_len, 16).expect("hex_len");
        perimeter += len;
        match dir {
            "0" => pos.0 += len,
            "1" => pos.1 += len,
            "2" => pos.0 -= len,
            "3" => pos.1 -= len,
            _ => panic!("unknown dir: {dir}"),
        }
        points.push(pos);
    }
    // shoelace formula for inner area
    let mut area: i64 = 0;
    for i in 0..points.len() - 1 {
        area += points[i].0 * points[i + 1].1 - points[i + 1].0 * points[i].1;
    }
    area /= 2;
    // Pick's theorem for final area
    let full_area = area + perimeter / 2 + 1;
    println!("{full_area}");
}
