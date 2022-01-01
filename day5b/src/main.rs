use std::collections::HashMap; 
use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt;


 #[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}


enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Other
}


type VentMap = HashMap<Point, i32>;


impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(",");
        let x = coords.next().unwrap().parse()?;
        let y = coords.next().unwrap().parse()?;

        Ok(Point {x: x, y: y})
    }
}


fn display_map(m: &VentMap) {
    let mut max_x = 0;
    let mut max_y = 0;

    for p in m.keys() {
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }

    for i in 0..=max_x {
        for j in 0..=max_y {
            let p = Point {x: j, y: i};
            match m.get(&p) {
                Some(n) => print!("{:01}", n),
                None => print!(".")
            }
        }
        println!("");
    }
}


fn parse_line(s: &str) -> (Point, Point) {
    let mut pair = s.split(" -> ");
    let start = pair.next().unwrap().parse().unwrap();
    let end = pair.next().unwrap().parse().unwrap();

    (start, end)
}


fn fill_vertical_line(m: &mut VentMap, x: i32, y1: i32, y2: i32) {
    assert!(y1 < y2);
    for y in y1..=y2 {
        let p = Point {x: x, y: y};
        let e = m.entry(p).or_insert(0);
        *e += 1;
    }
}


fn fill_horizontal_line(m: &mut VentMap, y: i32, x1: i32, x2: i32) {
    assert!(x1 < x2);
    for x in x1..=x2 {
        let p = Point {x: x, y: y};
        let e = m.entry(p).or_insert(0);
        *e += 1;
    }
}

fn fill_x_y_line(m: &mut VentMap, x1: i32, y1: i32, x2: i32, y2: i32) {
    assert!(x1 < x2);
    assert!(y1 < y2);
    let end = x2 - x1;
    for n in 0..=end {
        let p = Point {x: x1 + n, y: y1 + n};
        let e = m.entry(p).or_insert(0);
        *e += 1;
    }
} 

fn fill_x_negy_line(m: &mut VentMap, x1: i32, y1: i32, x2: i32, y2: i32) {
    assert!(x1 < x2);
    assert!(y2 < y1);
    let end = x2 - x1;
    for n in 0..=end {
        let p = Point {x: x1 + n, y: y1 - n};
        let e = m.entry(p).or_insert(0);
        *e += 1;
    }
}

fn calc_direction(p1: Point, p2: Point) -> Direction {
    let x_diff = p2.x - p1.x;
    let y_diff = p2.y - p1.y;

    if y_diff > 0 && x_diff == 0 {
        Direction::North
    } else if y_diff > 0 && x_diff > 0 {
        Direction::NorthEast
    } else if y_diff == 0 && x_diff > 0 {
        Direction::East
    } else if y_diff < 0 && x_diff > 0 {
        Direction::SouthEast
    } else if y_diff < 0 && x_diff == 0 {
        Direction::South
    } else if y_diff < 0 && x_diff < 0 {
        Direction::SouthWest
    } else if y_diff == 0 && x_diff < 0 {
        Direction::West
    } else if y_diff > 0 && x_diff < 0 {
        Direction::NorthWest
    } else {
        Direction::Other
    }
}


fn add_points_to_map(m: &mut VentMap, p1: Point, p2: Point) {

    let direction = calc_direction(p1, p2);
    let Point {x: x1, y: y1} = p1;
    let Point {x: x2, y: y2} = p2;

    match direction {
        Direction::North     => fill_vertical_line(m, x1, y1, y2),
        Direction::NorthEast => fill_x_y_line(m, x1, y1, x2, y2),
        Direction::East      => fill_horizontal_line(m, y1, x1, x2),
        Direction::SouthEast => fill_x_negy_line(m, x1, y1, x2, y2),
        Direction::South     => fill_vertical_line(m, x1, y2, y1),
        Direction::SouthWest => fill_x_y_line(m, x2, y2, x1, y1),
        Direction::West      => fill_horizontal_line(m, y1, x2, x1),
        Direction::NorthWest => fill_x_negy_line(m, x2, y2, x1, y1),
        Direction::Other     => panic!()
    }
}


fn main() {
    let raw = include_str!("input.txt");

    let vent_lines: Vec<(Point, Point)> = raw.lines().map(|l| parse_line(l)).collect();

    //let vent_lines = vent_lines.into_iter().map(|(p1,p2)| (p2, p1));

    let mut vent_map = HashMap::new();

    for (p1, p2) in vent_lines {
        add_points_to_map(&mut vent_map, p1, p2);
    }

    // Calculate number of overlapping lines
    let mut total = 0;
    for no_vents in vent_map.values() {
        if *no_vents > 1 {
            total += 1;
        }
    }

    println!("answer = {}", total);
}
