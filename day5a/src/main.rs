use std::collections::HashMap; 
use std::num::ParseIntError;
use std::str::FromStr;
use std::cmp::min;
use std::cmp::max;


 #[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: u32,
    y: u32
}


type VentMap = HashMap<Point, u32>;


impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(",");
        let x = coords.next().unwrap().parse()?;
        let y = coords.next().unwrap().parse()?;

        Ok(Point {x: x, y: y})
    }
}


fn parse_line(s: &str) -> (Point, Point) {
    let mut pair = s.split(" -> ");
    let start = pair.next().unwrap().parse().unwrap();
    let end = pair.next().unwrap().parse().unwrap();

    (start, end)
}

fn add_points_to_map(m: &mut VentMap, p1: Point, p2: Point) {

    let horizontal;
    let stat_dim;
    let dyn_dim_start;
    let dyn_dim_end;

    if p1.x == p2.x {
        horizontal = true;
        stat_dim = p1.x;
        dyn_dim_start = min(p1.y, p2.y);
        dyn_dim_end = max(p1.y, p2.y);
    } else if p1.y == p2.y {
        horizontal = false;
        stat_dim = p1.y;
        dyn_dim_start = min(p1.x, p2.x);
        dyn_dim_end = max(p1.x, p2.x);
    } else {
        return;
    }

    for dyn_dim in dyn_dim_start ..= dyn_dim_end {
        let p;
        if horizontal {
            p = Point {x: stat_dim, y: dyn_dim};
        } else {
            p = Point {x: dyn_dim, y: stat_dim};
        }
        let e = m.entry(p).or_insert(0);
        *e += 1;
    }
}


fn main() {
    let raw = include_str!("input.txt");

    let vent_lines = raw.lines().map(|l| parse_line(l));

    let mut vent_map = HashMap::new();

    for (p1, p2) in vent_lines {
        add_points_to_map(&mut vent_map, p1, p2);
    }

    let mut total = 0;
    for no_vents in vent_map.values() {
        if *no_vents > 1 {
            total += 1;
        }
    }

    println!("answer = {}", total);
}
