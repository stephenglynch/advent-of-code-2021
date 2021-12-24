

fn main() {

    let data = include_str!("input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let no_increases = data.windows(4).filter(|w| w[3] > w[0]).count();

    println!("number of increases = {}", no_increases);
}

