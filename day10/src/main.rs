#[derive(Clone, Copy, Debug)]
struct Tile {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl From<char> for Tile {
    #[rustfmt::skip]
    fn from(c: char) -> Self {
        match c {
            '|' => Self { up: true,  down: true,  left: false, right: false },
            '-' => Self { up: false, down: false, left: true,  right: true  },
            'L' => Self { up: true,  down: false, left: false, right: true  },
            'J' => Self { up: true,  down: false, left: true,  right: false },
            '7' => Self { up: false, down: true,  left: true,  right: false },
            'F' => Self { up: false, down: true,  left: false, right: true  },
            '.' => Self { up: false, down: false, left: false, right: false },
            'S' => Self { up: true,  down: true,  left: true,  right: true  },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    w: i32,
    h: i32,
    tiles: Vec<Tile>,
}

impl Map {
    fn new() -> Self {
        Self {
            w: 0,
            h: 0,
            tiles: Vec::new(),
        }
    }

    fn find_start_pos(&self) -> Option<(i32, i32)> {
        self.tiles
            .iter()
            .position(|t| t.up && t.down && t.left && t.right)
            .map(|raw_pos| (raw_pos as i32 % self.w, raw_pos as i32 / self.w))
    }

    fn get_tile(&self, x: i32, y: i32) -> Tile {
        assert!(x >= 0);
        assert!(x < self.w);
        assert!(y >= 0);
        assert!(y < self.h);
        self.tiles.get((y * self.w + x) as usize).copied().unwrap()
    }
}

fn main() {
    let mut map = Map::new();
    for line in std::io::stdin().lines().map(Result::unwrap) {
        let line_tiles = line.chars().map(Tile::from).collect::<Vec<Tile>>();
        map.w = line_tiles.len() as i32;
        map.h += 1;
        map.tiles.extend(line_tiles.iter());
    }
    let mut steps: usize = 0;
    let (start_x, start_y) = map.find_start_pos().expect("start pos");
    let mut x = start_x;
    let mut y = start_y;
    let (mut dx, mut dy) = if map.get_tile(x, y - 1).down {
        (0, -1)
    } else if map.get_tile(x, y + 1).up {
        (0, 1)
    } else if map.get_tile(x - 1, y).right {
        (-1, 0)
    } else if map.get_tile(x + 1, y).left {
        (1, 0)
    } else {
        panic!("no start direction")
    };
    x += dx;
    y += dy;
    steps += 1;
    while x != start_x || y != start_y {
        let t = map.get_tile(x, y);
        (dx, dy) = if t.up && !(dx == 0 && dy == 1) && map.get_tile(x, y - 1).down {
            (0, -1)
        } else if t.down && !(dx == 0 && dy == -1) && map.get_tile(x, y + 1).up {
            (0, 1)
        } else if t.left && !(dx == 1 && dy == 0) && map.get_tile(x - 1, y).right {
            (-1, 0)
        } else if t.right && !(dx == -1 && dy == 0) && map.get_tile(x + 1, y).left {
            (1, 0)
        } else {
            panic!("no step direction")
        };
        x += dx;
        y += dy;
        steps += 1;
    }
    println!("{}", steps / 2);
}
