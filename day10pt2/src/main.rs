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

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.w + x) as usize
    }

    fn get_tile(&self, x: i32, y: i32) -> Tile {
        if x >= 0 && x < self.w && y >= 0 && y < self.h {
            self.tiles.get(self.index(x, y)).copied().unwrap()
        } else {
            Tile {
                up: false,
                down: false,
                left: false,
                right: false,
            }
        }
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        assert!(x >= 0);
        assert!(x < self.w);
        assert!(y >= 0);
        assert!(y < self.h);
        let i = self.index(x, y);
        self.tiles.get_mut(i).unwrap()
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
    let mut loop_pipe = vec![false; map.tiles.len()];
    let (start_x, start_y) = map.find_start_pos().expect("start pos");
    let mut x = start_x;
    let mut y = start_y;
    loop_pipe[map.index(x, y)] = true;
    *map.get_tile_mut(x, y) = Tile {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    let (mut dx, mut dy) = if map.get_tile(x, y - 1).down {
        map.get_tile_mut(x, y).up = true;
        (0, -1)
    } else if map.get_tile(x, y + 1).up {
        map.get_tile_mut(x, y).down = true;
        (0, 1)
    } else if map.get_tile(x - 1, y).right {
        map.get_tile_mut(x, y).left = true;
        (-1, 0)
    } else if map.get_tile(x + 1, y).left {
        map.get_tile_mut(x, y).right = true;
        (1, 0)
    } else {
        panic!("no start direction")
    };
    x += dx;
    y += dy;
    while x != start_x || y != start_y {
        loop_pipe[map.index(x, y)] = true;
        let t = map.get_tile(x, y);
        (dx, dy) = if t.up
            && !(dx == 0 && dy == 1)
            && (map.get_tile(x, y - 1).down || (x == start_x && y - 1 == start_y))
        {
            map.get_tile_mut(x, y).up = true;
            map.get_tile_mut(x, y - 1).down = true;
            (0, -1)
        } else if t.down
            && !(dx == 0 && dy == -1)
            && (map.get_tile(x, y + 1).up || (x == start_x && y + 1 == start_y))
        {
            map.get_tile_mut(x, y).down = true;
            map.get_tile_mut(x, y + 1).up = true;
            (0, 1)
        } else if t.left
            && !(dx == 1 && dy == 0)
            && (map.get_tile(x - 1, y).right || (x - 1 == start_x && y == start_y))
        {
            map.get_tile_mut(x, y).left = true;
            map.get_tile_mut(x - 1, y).right = true;
            (-1, 0)
        } else if t.right
            && !(dx == -1 && dy == 0)
            && (map.get_tile(x + 1, y).left || (x + 1 == start_x && y == start_y))
        {
            map.get_tile_mut(x, y).right = true;
            map.get_tile_mut(x + 1, y).left = true;
            (1, 0)
        } else {
            panic!("no step direction {start_x} {start_y} {x} {y} {dx} {dy}")
        };
        x += dx;
        y += dy;
    }
    let mut enclosed: usize = 0;
    for y in 0..map.h {
        let mut count_enclosed = false;
        let mut last_left_corner = Tile {
            up: false,
            down: false,
            left: false,
            right: false,
        };
        for x in 0..map.w {
            if loop_pipe[map.index(x, y)] {
                let t = map.get_tile(x, y);
                if t.up && t.down {
                    count_enclosed = !count_enclosed;
                } else if t.up || t.down {
                    if t.right {
                        last_left_corner = t;
                    } else {
                        assert!(t.left);
                        if (last_left_corner.up && t.down) || (last_left_corner.down && t.up) {
                            count_enclosed = !count_enclosed;
                        }
                    }
                }
            } else if count_enclosed {
                enclosed += 1;
            }
        }
    }
    println!("{enclosed}");
}
