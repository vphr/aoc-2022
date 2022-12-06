use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut file = File::open("./src/bin/day6.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    for line in contents.lines() {
        solve_part1(line);
        solve_part2(line);
    }
}
fn solve(input: &str, window_size: usize) -> usize {
    let list_of_chars: Vec<char> = input.chars().collect();
    let first_occurance: Vec<_> = list_of_chars
        .windows(window_size)
        .enumerate()
        .filter_map(|(u, c)|
            // println!("{:?}", (u,c, &a));
            (c.iter().collect::<HashSet<_>>().len() == window_size).then_some(u))
        .collect();

    first_occurance.first().unwrap() + window_size
}
fn solve_part1(input: &str) {
    println!("FIRST SOLUTION: {}", solve(input, 4));
}

fn solve_part2(input: &str) {
    println!("SECOND SOLUTION: {}", solve(input, 14));
}
