use std::{fs::File, io::Read};

#[derive(Debug,Clone)]
struct Stack {
    stack: Vec<char>,
}

#[derive(Debug, Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(amount: usize, from: usize, to: usize) -> Self {
        Self { amount, from, to }
    }
}

impl Stack {
    fn new(stack: Vec<char>) -> Self {
        Self { stack }
    }
}

#[derive(Debug,Clone)]
struct Stacks {
    stacks: Vec<Stack>,
    length: usize,
}

impl Stacks {
    fn new(stacks: Vec<Stack>, length: usize) -> Self {
        Self { stacks, length }
    }
}

fn main() {
    let mut file = File::open("./src/bin/day5.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    let (stack, instructions) = parse_input(&contents);
    solve_part1(stack.clone(),instructions.clone());
    solve_part2(stack,instructions);
}

fn parse_input(input: &str) -> (Stacks, Vec<Instruction>){
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let list: Vec<Vec<char>> = stacks
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let parsed_cranes: Vec<_> = list
        .iter()
        .map(|entry| {
            // println!("{:?}", entry);
            let e = entry.chunks(4).collect::<Vec<&[char]>>();
            let e: Vec<Vec<char>> = e
                .iter()
                .cloned()
                .map(|v| v.iter().take(3).cloned().collect::<Vec<char>>())
                .collect();
            e
        })
        .rev()
        .collect();

    let mut crane_stacks_vec = Stacks::new(vec![], parsed_cranes.len());

    for _ in 0..crane_stacks_vec.length {
        let s_1 = Stack::new(vec![]);
        crane_stacks_vec.stacks.push(s_1);
    }


    for s in &parsed_cranes{
            for (i, elem) in s.iter().enumerate() {
                let a = elem.get(1).unwrap();
                // println!("{:?} {}", a, i);
                if !a.is_whitespace() {
                    crane_stacks_vec.stacks.get_mut(i).unwrap().stack.push(*a);
                }
            }
    }

    let parse_instructions: Vec<_> = instructions
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|n| n.chars().all(|c| c.is_numeric()))
                .collect::<Vec<&str>>()
        })
        .map(|remaining| {
            let r: Vec<_> = remaining
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            Instruction::new(r[0], r[1] - 1, r[2] - 1)
        })
        .collect();

    (crane_stacks_vec, parse_instructions)
}
fn solve_part1(mut s_0: Stacks, parse_instructions: Vec<Instruction>) {
    for Instruction { amount, to, from } in parse_instructions {
        for _ in 0..amount {
            let moving_object = s_0.stacks.get_mut(from).unwrap().stack.pop().unwrap();
            s_0.stacks.get_mut(to).unwrap().stack.push(moving_object);
        }
    }

    let answer: String = s_0
        .stacks
        .iter()
        .filter_map(|v| 
            v.stack.last()
        )
        .collect();

    println!("PART 1: {}", answer);
}

fn solve_part2(mut s_0: Stacks, parse_instructions: Vec<Instruction>) {
    for Instruction { amount, to, from } in parse_instructions {
        let mut tomove: Vec<char> = vec![];
        for _ in 0..amount {
            let moving_object = s_0.stacks.get_mut(from).unwrap().stack.pop().unwrap();
            tomove.push(moving_object);
        }
        tomove.reverse();
        for o in tomove.iter() {
            s_0.stacks.get_mut(to).unwrap().stack.push(o.to_owned());
        }
    }

    let answer: String = s_0
        .stacks
        .iter()
        .filter_map(|v| 
            v.stack.last()
        )
        .collect();

    println!("PART 2: {}", answer);
}
