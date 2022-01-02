
fn new_fish() -> u32 {
    8
}

fn simulate_school(mut school: Vec<u32>) -> Vec<u32> {
    let sim_len = 80;

    // println!("{:?}", school);

    for _ in 0..sim_len {

        let mut no_new_fish = 0;
        
        for f in school.iter_mut() {
            if *f == 0 {
                no_new_fish += 1;
                *f = 6;
            } else {
                *f -= 1;
            }
        }

        for _ in 0..no_new_fish {
            school.push(new_fish())
        }

        // println!("{:?}", school);
    }

    school
}

fn parse_input(s: &str) -> Vec<u32> {
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let raw = include_str!("input.txt");

    let school = parse_input(raw);

    let school = simulate_school(school);

    println!("answer = {:?}", school.len())
}
