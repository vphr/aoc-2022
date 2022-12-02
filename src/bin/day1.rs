use std::{fs::File, io::Read};
fn solve_part1() {
    let mut file = File::open("./src/day1.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let contents: Vec<u32> = contents
        .split("\n\n")
        .into_iter()
        .map(|elem| {
            elem.split('\n')
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|f| !f.is_empty())
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|f| f.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    let result = contents.iter().max();
    println!("{:?}", result);
}

fn solve_part2() {
    let mut file = File::open("./src/day1.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let mut contents: Vec<u32> = contents
        .split("\n\n")
        .into_iter()
        .map(|elem| {
            elem.split('\n')
                .collect::<Vec<&str>>()
                .into_iter()
                .filter(|f| !f.is_empty())
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|f| f.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    contents.sort_by(|a,b| b.cmp(a));
    let res = contents.iter().take(3).sum::<u32>();
    println!("{:?}", res)
}
fn main() {
    solve_part1();
    solve_part2();
}

