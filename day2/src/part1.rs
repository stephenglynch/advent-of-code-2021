use regex::Regex;

mod part1 {
    pub fn run(passwords: &str) {
        let re = Regex::new(r"(?m)^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        let mut count = 0;

        for cap in re.captures_iter(passwords) {
            let lower: i32 = cap[1].parse().unwrap();
            let upper: i32 = cap[2].parse().unwrap();
            let char = cap[3].chars().next().unwrap();
            let password = &cap[4];
            
            let c_count = password.chars().fold(0, |count, c| if c == char {count + 1} else {count});
            if c_count >= lower && c_count <= upper {
                count += 1;
            }
        }

        println!("NO valid passwords: {}", count);
    }
}