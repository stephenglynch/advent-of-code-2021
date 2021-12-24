use core::panic;


#[derive(Debug)]
struct Position {
    hor: i32,
    depth: i32,
    aim: i32,
}

fn move_submarine(pos: Position, s: &str) -> Position {

    let (action, val_s) = s.split_once(' ').unwrap();

    let val: i32 = val_s.parse().unwrap();

    match action {
        "forward" => Position {
            hor: pos.hor + val, 
            depth: pos.depth + pos.aim * val, 
            aim: pos.aim
        },
        "up" => Position {
            hor: pos.hor, 
            depth: pos.depth, 
            aim: pos.aim - val
        },
        "down" => Position {
            hor: pos.hor, 
            depth: pos.depth, 
            aim: pos.aim + val
        },
        _ => panic!()
    }
}

fn main() {

    let start = Position{hor: 0, depth: 0, aim: 0};

    let final_pos = include_str!("input.txt")
        .lines()
        .fold(start, move_submarine);

    println!("{:?}", final_pos);
    println!("answer = {}", final_pos.hor * final_pos.depth);
}
