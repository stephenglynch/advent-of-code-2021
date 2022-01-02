
fn identify_digit(pat: &str) -> Option<u8> {
    match pat.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None
    }
}

fn parse_input(s: &str) -> (Vec<&str>, Vec<Vec<&str>>) {

    let mut unique_codes = Vec::new();
    let mut outputs = Vec::new();

    for line in s.lines() {
        let mut spl = line.split('|');
        let left = spl.next().unwrap();
        let right = spl.next().unwrap();

        unique_codes.push(left);
        outputs.push(right.split_whitespace().collect());
    }

    (unique_codes, outputs)
}

fn main() {
    let raw = include_str!("input.txt");

    let (_, outs) = parse_input(raw);

    let mut total = 0;
    for out in outs {
        total += out.into_iter().map(&identify_digit).fold(0, |acc, x| {
            acc + match x {
                Some(_) => 1,
                None => 0
            }
        });
    }

    println!("answer = {}", total);
}
