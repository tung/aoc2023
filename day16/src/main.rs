use std::collections::HashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Beam {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

struct Map {
    w: i32,
    h: i32,
    tiles: Vec<char>,
    visited: Vec<bool>,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let tiles = lines
            .iter()
            .flat_map(|line| line.chars())
            .collect::<Vec<char>>();
        let visited_len = tiles.len();
        Self {
            w: lines.get(0).map(String::len).unwrap_or(0) as i32,
            h: lines.len() as i32,
            tiles,
            visited: vec![false; visited_len],
        }
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.w + x) as usize
    }

    fn fire_beam(&mut self) {
        let mut beams: Vec<Beam> = Vec::new();
        beams.push(Beam {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        });
        let mut past_beams: HashSet<Beam> = HashSet::new();
        while !beams.is_empty() {
            beams.retain(|b| {
                b.x >= 0 && b.x < self.w && b.y >= 0 && b.y < self.h && !past_beams.contains(b)
            });
            let mut new_beams: Vec<Beam> = Vec::new();
            for beam in beams.iter_mut() {
                past_beams.insert(*beam);
                let index = self.index(beam.x, beam.y);
                self.visited[index] = true;
                match (beam.dx, beam.dy, self.tiles[index]) {
                    (_, _, '.') => {}
                    // up beam
                    (0, -1, '/') => {
                        // reflect right
                        beam.dx = 1;
                        beam.dy = 0;
                    }
                    (0, -1, '\\') => {
                        // reflect left
                        beam.dx = -1;
                        beam.dy = 0;
                    }
                    (0, -1, '|') => {}
                    (0, -1, '-') => {
                        // split left and right
                        beam.dx = -1;
                        beam.dy = 0;
                        new_beams.push(Beam {
                            x: beam.x,
                            y: beam.y,
                            dx: 1,
                            dy: 0,
                        });
                    }
                    // down beam
                    (0, 1, '/') => {
                        // reflect left
                        beam.dx = -1;
                        beam.dy = 0;
                    }
                    (0, 1, '\\') => {
                        // reflect right
                        beam.dx = 1;
                        beam.dy = 0;
                    }
                    (0, 1, '|') => {}
                    (0, 1, '-') => {
                        // split left and right
                        beam.dx = -1;
                        beam.dy = 0;
                        new_beams.push(Beam {
                            x: beam.x,
                            y: beam.y,
                            dx: 1,
                            dy: 0,
                        });
                    }
                    // left beam
                    (-1, 0, '/') => {
                        // reflect down
                        beam.dx = 0;
                        beam.dy = 1;
                    }
                    (-1, 0, '\\') => {
                        // reflect up
                        beam.dx = 0;
                        beam.dy = -1;
                    }
                    (-1, 0, '|') => {
                        // split up and down
                        beam.dx = 0;
                        beam.dy = -1;
                        new_beams.push(Beam {
                            x: beam.x,
                            y: beam.y,
                            dx: 0,
                            dy: 1,
                        });
                    }
                    (-1, 0, '-') => {}
                    // right beam
                    (1, 0, '/') => {
                        // reflect up
                        beam.dx = 0;
                        beam.dy = -1;
                    }
                    (1, 0, '\\') => {
                        // reflect down
                        beam.dx = 0;
                        beam.dy = 1;
                    }
                    (1, 0, '|') => {
                        // split up and down
                        beam.dx = 0;
                        beam.dy = -1;
                        new_beams.push(Beam {
                            x: beam.x,
                            y: beam.y,
                            dx: 0,
                            dy: 1,
                        });
                    }
                    (1, 0, '-') => {}
                    _ => panic!(),
                }
            }
            beams.extend(new_beams.drain(..));
            for beam in beams.iter_mut() {
                beam.x += beam.dx;
                beam.y += beam.dy;
            }
        }
    }

    fn count_energized(&self) -> usize {
        self.visited.iter().filter(|&&v| v).count()
    }
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let mut map = Map::new(&lines[..]);
    map.fire_beam();
    println!("{}", map.count_energized());
}
