use std::io;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Number {
    pos: Position,
    width: i32,
    value: usize,
}

impl Number {
    fn is_adjacent(&self, sym: &Symbol) -> bool {
        let res = sym.pos.x >= self.pos.x - 1
            && sym.pos.x <= self.pos.x + self.width
            && sym.pos.y >= self.pos.y - 1
            && sym.pos.y <= self.pos.y + 1;
        res
    }
}

#[derive(Debug)]
struct Symbol {
    pos: Position,
    sym: char,
}

pub fn main() {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let lines = io::stdin().lines();
    for (row, line) in lines.enumerate() {
        let line = line.expect("line");
        let mut column = 0;
        for mut nums_or_syms in line.split('.') {
            while !nums_or_syms.is_empty() {
                let pos = Position {
                    x: column,
                    y: row as i32,
                };
                if nums_or_syms.starts_with(|ch| ch >= '0' && ch <= '9') {
                    let width = nums_or_syms
                        .find(|ch| ch < '0' || ch > '9')
                        .unwrap_or(nums_or_syms.len());
                    let (num_str, rest) = nums_or_syms.split_at(width);
                    numbers.push(Number {
                        pos,
                        width: width as i32,
                        value: num_str.parse::<usize>().expect("value"),
                    });
                    nums_or_syms = rest;
                    column += width as i32;
                } else {
                    let (sym, rest) = nums_or_syms.split_at(1);
                    symbols.push(Symbol {
                        pos,
                        sym: sym.chars().next().expect("symbol char"),
                    });
                    nums_or_syms = rest;
                    column += 1;
                }
            }
            column += 1;
        }
    }

    let sum = symbols
        .iter()
        .filter(|s| s.sym == '*')
        .map(|s| {
            numbers
                .iter()
                .filter(|n| n.is_adjacent(s))
                .copied()
                .collect::<Vec<Number>>()
        })
        .filter(|num_vec| num_vec.len() == 2)
        .map(|num_vec| num_vec[0].value * num_vec[1].value)
        .sum::<usize>();
    println!("{sum:?}");
}
