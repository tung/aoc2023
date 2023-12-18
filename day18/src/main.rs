use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Color(u8, u8, u8);

fn paint_line(map: &mut HashMap<(i32, i32), Color>, cursor: &mut (i32, i32), line: &str) {
    let mut tokens = line.split_whitespace();
    let (dx, dy): (i32, i32) = match tokens.next() {
        Some("U") => (0, -1),
        Some("D") => (0, 1),
        Some("L") => (-1, 0),
        Some("R") => (1, 0),
        _ => panic!("U, D, L or R expected"),
    };
    let steps = tokens
        .next()
        .expect("steps")
        .parse::<usize>()
        .expect("steps as usize");
    let color_str = tokens
        .next()
        .expect("color")
        .strip_prefix("(#")
        .expect("color prefix")
        .strip_suffix(")")
        .expect("color suffix");
    let (red_str, color_str) = color_str.split_at(2);
    let (green_str, color_str) = color_str.split_at(2);
    let (blue_str, _color_str) = color_str.split_at(2);
    let red = u8::from_str_radix(red_str, 16).expect("red u8");
    let green = u8::from_str_radix(green_str, 16).expect("green u8");
    let blue = u8::from_str_radix(blue_str, 16).expect("blue u8");
    let color = Color(red, green, blue);
    for _ in 0..steps {
        cursor.0 += dx;
        cursor.1 += dy;
        map.insert(*cursor, color);
    }
}

fn flood_fill(map: &mut HashMap<(i32, i32), Color>) {
    let mut to_visit: Vec<(i32, i32)> = vec![(1, 1)];
    let inner_color = Color(0, 0, 0);
    while !to_visit.is_empty() {
        let pos = to_visit.pop().expect("to_visit non-empty");
        let up = (pos.0, pos.1 - 1);
        let down = (pos.0, pos.1 + 1);
        let left = (pos.0 - 1, pos.1);
        let right = (pos.0 + 1, pos.1);
        map.insert(pos, inner_color);
        if !map.contains_key(&up) {
            to_visit.push(up);
        }
        if !map.contains_key(&down) {
            to_visit.push(down);
        }
        if !map.contains_key(&left) {
            to_visit.push(left);
        }
        if !map.contains_key(&right) {
            to_visit.push(right);
        }
    }
}

fn main() {
    let mut map: HashMap<(i32, i32), Color> = HashMap::new();
    let mut cursor: (i32, i32) = (0, 0);
    for line in std::io::stdin().lines().map(Result::unwrap) {
        paint_line(&mut map, &mut cursor, &line);
    }
    flood_fill(&mut map);
    println!("{}", map.iter().count());
}
