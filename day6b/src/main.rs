

type School = [u64; 9];

const sim_len: u32 = 256;

fn simulate_day(s: School) -> School {
    let max_days = 8;
    let mut new_school = [0; 9];

    // Spawned fished and reset fish
    new_school[8] = s[0];
    new_school[6] = s[0];

    for n in 0..max_days {
        new_school[n] += s[n+1];
    }
    // new_school[0] = 0[1];
    // new_school[1] = 0[2];
    // ...
    // new_school[7] = 0[8];

    new_school
}

fn simulate_school(mut school: School) -> School {
    for _ in 0..sim_len {
        school = simulate_day(school);
    }

    school
}

fn parse_input(s: &str) -> School {
    let mut school = [0; 9];
    let fishes= s.split(',').map(|x| -> usize {x.parse().unwrap()});
    for f in fishes {
        school[f] += 1;
    }

    school
}

fn no_fish(s: &School) -> u64 {
    s.iter().sum()
}

fn main() {
    let raw = include_str!("input.txt");

    let school = parse_input(raw);

    println!("{:?}", school);

    let school = simulate_school(school);

    println!("{:?}", school);

    println!("answer = {:?}", no_fish(&school))
}
