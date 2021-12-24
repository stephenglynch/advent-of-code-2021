use core::panic;
use std::ops::Add;

#[derive(Debug)]
struct Position {
    hor: i32,
    depth: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Position) -> Position {
        Position {
            hor: self.hor + other.hor,
            depth: self.depth + other.depth,
        }
    }
}

fn convert(s: &str) -> Position {

    let (action, val_s) = s.split_once(' ').unwrap();

    let val = val_s.parse().unwrap();

    match action {
        "forward" => Position {hor: val, depth: 0},
        "up" => Position {hor: 0, depth: -val},
        "down" => Position {hor: 0, depth: val},
        _ => panic!()
    }
}

fn main() {

    let final_pos = include_str!("input.txt")
        .lines()
        .map(convert)
        .reduce(|acc, pos| acc + pos)
        .unwrap();

    println!("answer = {}", final_pos.hor * final_pos.depth);
}
