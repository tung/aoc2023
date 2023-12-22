use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Brick {
    min_x: u16,
    min_y: u16,
    min_z: u16,
    max_x: u16,
    max_y: u16,
    max_z: u16,
}

impl Brick {
    fn collide_xy(&self, other: &Self) -> bool {
        self.min_x <= other.max_x
            && self.max_x >= other.min_x
            && self.min_y <= other.max_y
            && self.max_y >= other.min_y
    }
}

fn main() {
    let mut bricks: Vec<Brick> = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut coords = line
                .split(|c: char| !c.is_ascii_digit())
                .map(|num_str| num_str.parse::<u16>().expect("u16 coord"));
            Brick {
                min_x: coords.next().expect("min_x"),
                min_y: coords.next().expect("min_y"),
                min_z: coords.next().expect("min_z"),
                max_x: coords.next().expect("max_x"),
                max_y: coords.next().expect("max_y"),
                max_z: coords.next().expect("max_z"),
            }
        })
        .collect::<_>();
    loop {
        let mut bricks_fell = false;
        for a in 0..bricks.len() {
            let mut new_min_z = 1;
            for b in 0..bricks.len() {
                if a == b {
                    continue;
                }
                if bricks[a].collide_xy(&bricks[b]) && bricks[a].min_z > bricks[b].max_z {
                    new_min_z = new_min_z.max(bricks[b].max_z + 1);
                }
            }
            if new_min_z != bricks[a].min_z {
                let z_diff = bricks[a].min_z - new_min_z;
                bricks[a].min_z -= z_diff;
                bricks[a].max_z -= z_diff;
                bricks_fell = true;
            }
        }
        if !bricks_fell {
            break;
        }
    }
    let aboves: HashMap<Brick, HashSet<Brick>> = bricks
        .iter()
        .map(|b| {
            (
                *b,
                bricks
                    .iter()
                    .filter(|above| b.collide_xy(above) && above.min_z == b.max_z + 1)
                    .copied()
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<_>();
    let belows: HashMap<Brick, HashSet<Brick>> = bricks
        .iter()
        .map(|b| {
            (
                *b,
                bricks
                    .iter()
                    .filter(|below| b.collide_xy(below) && b.min_z == below.max_z + 1)
                    .copied()
                    .collect::<HashSet<_>>(),
            )
        })
        .collect::<_>();
    let sum = bricks
        .iter()
        .map(|brick| {
            let mut set: HashSet<Brick> = HashSet::new();
            let mut queue: VecDeque<Brick> = VecDeque::new();
            set.insert(*brick);
            for above in aboves[brick].iter() {
                if belows[above].is_subset(&set) {
                    set.insert(*above);
                    queue.push_back(*above);
                }
            }
            while let Some(q) = queue.pop_front() {
                for above in aboves[&q].iter() {
                    if belows[above].is_subset(&set) {
                        set.insert(*above);
                        queue.push_back(*above);
                    }
                }
            }
            set.len() - 1
        })
        .sum::<usize>();
    println!("{sum}");
}
