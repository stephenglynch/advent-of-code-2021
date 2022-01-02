

fn parse_input(s: &str) -> Vec<i32> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}


fn calc_move_fuel(p1: i32, p2: i32) -> i32 {
    let dist = (p1 - p2).abs();
    dist * (dist + 1) / 2
}


fn calc_all_fuel(positions: &Vec<i32>, move_pos: i32) -> i32 {
    let mut fuel_used = 0;

    for pos in positions {
        fuel_used += calc_move_fuel(*pos, move_pos);
    }

    fuel_used
}


fn main() {
    let raw = include_str!("example.txt");

    let positions = parse_input(raw);

    let max_pos = positions.iter().copied().max().unwrap();

    let fuel_consumed: Vec<i32> = (1..max_pos).map(|i| calc_all_fuel(&positions, i)).collect();

    println!("{:?}", fuel_consumed);

    let min_fuel = fuel_consumed.iter().copied().min().unwrap();

    println!("answer = {}", min_fuel);
}
