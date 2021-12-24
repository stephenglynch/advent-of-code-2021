
fn filter_common_bit(lines: Vec<&str>, index: usize) -> Vec<&str> {
    let mut ones = 0;
    let mut zeros = 0;

    for l in lines.iter() {
        if l.chars().nth(index).unwrap() == '1' {
            ones += 1;
        } else {
            zeros += 1;
        }
    }

    let common =  if ones >= zeros {'1'} else {'0'};

    lines.into_iter().filter(|l| l.chars().nth(index).unwrap() == common).collect()
}


fn filter_uncommon_bit(lines: Vec<&str>, index: usize) -> Vec<&str> {
    let mut ones = 0;
    let mut zeros = 0;

    for l in lines.iter() {
        if l.chars().nth(index).unwrap() == '1' {
            ones += 1;
        } else {
            zeros += 1;
        }
    }

    let uncommon =  if ones < zeros {'1'} else {'0'};

    lines.into_iter().filter(|l| l.chars().nth(index).unwrap() == uncommon).collect()
}


fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let width = lines[0].len();

    let mut oxygen_lines = lines.clone();
    for i in (0..width) {
        oxygen_lines = filter_common_bit(oxygen_lines, i);
        println!("i={} len={} v={:?}", i, oxygen_lines.len(), oxygen_lines);
        if oxygen_lines.len() == 1 {
            break;
        }
    }

    let mut co2_lines = lines.clone();
    for i in (0..width) {
        co2_lines = filter_uncommon_bit(co2_lines, i);
        println!("i={} len={} v={:?}", i, co2_lines.len(), co2_lines);
        if co2_lines.len() == 1 {
            break;
        }
    }

    let o2 = u32::from_str_radix(oxygen_lines[0], 2).unwrap();
    let co2 = u32::from_str_radix(co2_lines[0], 2).unwrap();

    println!("o2 = {}", o2);
    println!("co2 = {}", co2);
    println!("answer = {}", o2 * co2);
}
