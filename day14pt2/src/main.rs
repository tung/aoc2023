use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Round,
    Cube,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => unreachable!(),
        }
    }
}

struct Map {
    w: usize,
    h: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        Self {
            w: lines[0].len(),
            h: lines.len(),
            tiles: lines
                .iter()
                .flat_map(|l| l.chars().map(Tile::from))
                .collect::<Vec<Tile>>(),
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.w);
        assert!(y < self.h);
        y * self.w + x
    }

    fn calc_load(&self) -> usize {
        let mut sum: usize = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.tiles[self.index(x, y)] == Tile::Round {
                    sum += self.h - y;
                }
            }
        }
        sum
    }

    fn cycle_once(&mut self) {
        // tilt north
        for x in 0..self.w {
            let mut last_free_y: usize = 0;
            for y in 0..self.h {
                let curr = self.index(x, y);
                if self.tiles[curr] == Tile::Cube {
                    last_free_y = y + 1;
                } else if self.tiles[curr] == Tile::Round {
                    self.tiles[curr] = Tile::Empty;
                    let last_free = self.index(x, last_free_y);
                    self.tiles[last_free] = Tile::Round;
                    last_free_y += 1;
                }
            }
        }
        // tilt west
        for y in 0..self.h {
            let mut last_free_x: usize = 0;
            for x in 0..self.w {
                let curr = self.index(x, y);
                if self.tiles[curr] == Tile::Cube {
                    last_free_x = x + 1;
                } else if self.tiles[curr] == Tile::Round {
                    self.tiles[curr] = Tile::Empty;
                    let last_free = self.index(last_free_x, y);
                    self.tiles[last_free] = Tile::Round;
                    last_free_x += 1;
                }
            }
        }
        // tilt south
        for x in 0..self.w {
            let mut last_free_y: usize = self.h - 1;
            for y in 0..self.h {
                let y = self.h - y - 1;
                let curr = self.index(x, y);
                if self.tiles[curr] == Tile::Cube {
                    last_free_y = y.saturating_sub(1);
                } else if self.tiles[curr] == Tile::Round {
                    self.tiles[curr] = Tile::Empty;
                    let last_free = self.index(x, last_free_y);
                    self.tiles[last_free] = Tile::Round;
                    last_free_y = last_free_y.saturating_sub(1);
                }
            }
        }
        // tilt east
        for y in 0..self.h {
            let mut last_free_x: usize = self.w - 1;
            for x in 0..self.w {
                let x = self.w - x - 1;
                let curr = self.index(x, y);
                if self.tiles[curr] == Tile::Cube {
                    last_free_x = x.saturating_sub(1);
                } else if self.tiles[curr] == Tile::Round {
                    self.tiles[curr] = Tile::Empty;
                    let last_free = self.index(last_free_x, y);
                    self.tiles[last_free] = Tile::Round;
                    last_free_x = last_free_x.saturating_sub(1);
                }
            }
        }
    }

    fn do_cycles(&mut self, times: usize) {
        let mut previous: HashMap<Vec<Tile>, usize> = HashMap::new();
        previous.insert(self.tiles.clone(), 0);
        for i in 0..times {
            self.cycle_once();
            if let Some(&prev_time) = previous.get(&self.tiles) {
                println!("cycle found: {i} -> {prev_time}");
                for _ in 1..((times - prev_time) % (i - prev_time)) {
                    self.cycle_once();
                }
                break;
            } else {
                previous.insert(self.tiles.clone(), i);
            }
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                write!(
                    f,
                    "{}",
                    match self.tiles[self.index(x, y)] {
                        Tile::Empty => '.',
                        Tile::Round => 'O',
                        Tile::Cube => '#',
                    },
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut map = Map::new(&lines[..]);
    map.do_cycles(1_000_000_000);
    println!("{}", map.calc_load());
}

#[test]
#[rustfmt::skip]
fn test_tumble() {
    let input = &[
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let mut map = Map::new(&input);
    assert_eq!(
        "O....#....\n\
         O.OO#....#\n\
         .....##...\n\
         OO.#O....O\n\
         .O.....O#.\n\
         O.#..O.#.#\n\
         ..O..#O..O\n\
         .......O..\n\
         #....###..\n\
         #OO..#....\n",
         format!("{map:?}"),
     );
    map.cycle_once();
    assert_eq!(
        ".....#....\n\
         ....#...O#\n\
         ...OO##...\n\
         .OO#......\n\
         .....OOO#.\n\
         .O#...O#.#\n\
         ....O#....\n\
         ......OOOO\n\
         #...O###..\n\
         #..OO#....\n",
        format!("{map:?}"),
    );
}
