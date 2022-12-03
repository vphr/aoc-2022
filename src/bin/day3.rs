use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut file = File::open("./src/bin/day3.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    solve_part1(&contents);
    solve_part2(&contents);
}

fn solve_part1(input: &str) {
    let split: u32 = input
        .lines()
        .map(|f| (f.len() / 2, f))
        .map(|(s, st)| st.split_at(s))
        .map(|f| {
            let t =
                f.0.chars()
                    .flat_map(|c| f.1.match_indices(c))
                    .take(1)
                    .map(|(u, c)| c)
                    .collect::<Vec<&str>>()[0];

            let val: char = t.chars().take(1).collect::<Vec<char>>()[0];
            let mut num: u32 = 0;
            if val.is_uppercase() {
                let ascii = val as u32;
                num = ascii - 38;
            } else {
                let ascii = val as u32;
                num = ascii - 96
            }
            num
        })
        .sum::<u32>();

    println!("{:?}", split);
}
fn solve_part2(input: &str) {
    let mut split = input.lines();
    let mut res: Vec<(&str, &str, &str)> = vec![];

    while let (Some(a), Some(b), Some(c)) = (split.next(), split.next(), split.next()) {
        res.push((a, b, c));
    }
    let mut hm: HashMap<char, u32> = HashMap::new();
    let result: u32 = res
        .iter()
        .map(|(a, b, c)| {
            hm.clear();
            for a_chars in a.chars() {
                hm.insert(a_chars, 1);
            }
            for b_chars in b.chars() {
                if let Some(c) = hm.get_mut(&b_chars) {
                    *c = 2;
                } else {
                    continue;
                }
            }
            for c_chars in c.chars() {
                if let Some(c) = hm.get_mut(&c_chars) {
                    if *c == 2 {
                        *c = 3;
                    }
                } else {
                    continue;
                }
            }
            let res = hm.iter().max_by(|a, b| a.1.cmp(b.1)).map(|(c, u)| c.to_owned()).unwrap();
            let mut num = 0;
            if res.is_uppercase() {
                let ascii = res as u32;
                num = ascii - 38;
            } else {
                let ascii = res as u32;
                num = ascii - 96
            }
            num
        })
        .sum::<u32>();

    println!("{:?}", result);
}
