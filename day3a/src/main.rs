
fn count_vertical(lines: Vec<&str>) -> Vec<u32> {
    let mut acc = vec![0; lines[0].len()];

    for line in lines {
        for (i, c ) in line.chars().enumerate() {
            acc[i] += if c == '1' {1} else {0};
        }
    }

    acc
}

fn calc_gamma_epsilon(counts: Vec<u32>, total: u32) -> (u32, u32) {

    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, c) in counts.iter().rev().enumerate() {
        if *c > total / 2  {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    (gamma, epsilon)
}

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let no_lines = lines.len();
    let counts = count_vertical(lines);

    println!("no_lines={} counts={:?}", no_lines, counts);

    let (gamma, epsilon) = calc_gamma_epsilon(counts, no_lines as u32);

    println!("gamma={} epsilon={}", gamma, epsilon);

    println!("answer = {:?}", gamma * epsilon);
}
