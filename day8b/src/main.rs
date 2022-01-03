use std::collections::BTreeSet;
use std::collections::HashMap;


fn parse_line(s: &str) -> (Vec<BTreeSet<char>>, Vec<BTreeSet<char>>) {

    let mut spl = s.split('|');

    let left = spl.next().unwrap().split_whitespace().map(|s| {
        s.chars().collect()
    }).collect();

    let right = spl.next().unwrap().split_whitespace().map(|s| {
        s.chars().collect()
    }).collect();

    (left, right)
}


fn get_set_of_len(codes: &Vec<BTreeSet<char>>, len: usize) -> &BTreeSet<char> {
    codes.iter().filter(|x| x.len() == len).next().unwrap()
}


fn determine_mapping(codes: Vec<BTreeSet<char>>) -> HashMap<BTreeSet<char>, i32> {
    // Get sets that have unique size
    let one = get_set_of_len(&codes, 2);
    let four = get_set_of_len(&codes, 4);
    let seven = get_set_of_len(&codes, 3);
    let eight = get_set_of_len(&codes, 7);

    // Determine numbers that have 6 segments
    let len_sixes: Vec<BTreeSet<char>> = codes.iter().cloned().filter(|x| x.len() == 6).collect();
    // Find intersections with known numbers that are unique
    let six = len_sixes.iter().filter(|x| x.intersection(seven).count() == 2).next().unwrap();
    let nine = len_sixes.iter().filter(|x| x.intersection(four).count() == 4).next().unwrap();
    let zero = len_sixes.iter().filter(|x| *x != six && *x != nine).next().unwrap();

    // Determine numbers that have 5 segments
    let len_fives: Vec<BTreeSet<char>> = codes.iter().cloned().filter(|x| x.len() == 5).collect();
    // Find intersections with known numbers that are unique
    let three = len_fives.iter().filter(|x| x.intersection(seven).count() == 3).next().unwrap();
    let two = len_fives.iter().filter(|x| x.intersection(nine).count() == 4).next().unwrap();
    let five = len_fives.iter().filter(|x| *x != three && *x != two).next().unwrap();

    HashMap::from([
        (zero.to_owned(), 0),
        (one.to_owned(), 1),
        (two.to_owned(), 2),
        (three.to_owned(), 3),
        (four.to_owned(), 4),
        (five.to_owned(), 5),
        (six.to_owned(), 6),
        (seven.to_owned(), 7),
        (eight.to_owned(), 8),
        (nine.to_owned(), 9),
    ])
}

fn main() {
    let raw = include_str!("input.txt");

    
    let mut total = 0;

    for line in raw.lines() {
        let (unique_codes, out) = parse_line(line);

        let mapping = determine_mapping(unique_codes);

        for (n, encoded) in out.iter().rev().enumerate() {
            let value = mapping[encoded] * 10i32.pow(n as u32);
            total += value
        }
    }

    println!("answer = {}", total);
}
