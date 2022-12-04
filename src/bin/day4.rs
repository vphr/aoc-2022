use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./src/bin/day4.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    solve_part1(&contents);
    solve_part2(&contents);
}

struct Assignment {
    start: u32,
    end: u32,
}
impl Assignment {
    fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
    fn contains_inside(&self, other: Assignment) -> bool {
        if self.start >= other.start && self.end <= other.end
            || self.start <= other.start && self.end >= other.end
        {
            return true;
        }
        false
    }
    fn overlap(&self, other: Assignment) -> bool {
        let first_vec: Vec<u32> = (self.start..self.end + 1).collect();
        (other.start..other.end + 1)
            .into_iter()
            .any(|x| first_vec.contains(&x))
    }
}

fn solve_part1(input: &str) {
    let res: usize = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            (split[0], split[1])
        })
        .map(|(f, s)| {
            let first_split: Vec<u32> = f.split('-').map(|f| f.parse::<u32>().unwrap()).collect();
            let first_assignment = Assignment::new(first_split[0], first_split[1]);
            let second_split: Vec<u32> = s.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
            let second_assignment = Assignment::new(second_split[0], second_split[1]);
            first_assignment.contains_inside(second_assignment)
        })
        .filter(|cond| *cond)
        .count();
    println!("PART1: {:?}", res);
}
fn solve_part2(input: &str) {
    let res: usize = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            (split[0], split[1])
        })
        .map(|(f, s)| {
            let first_split: Vec<u32> = f.split('-').map(|f| f.parse::<u32>().unwrap()).collect();
            let first_assignment = Assignment::new(first_split[0], first_split[1]);
            let second_split: Vec<u32> = s.split('-').map(|s| s.parse::<u32>().unwrap()).collect();
            let second_assignment = Assignment::new(second_split[0], second_split[1]);
            first_assignment.overlap(second_assignment)
        })
        .filter(|cond| *cond)
        .count();
    println!("PART2: {:?}", res);
}
