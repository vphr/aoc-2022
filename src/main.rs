use std::{error::Error, fs::File, io::Read, process::Command};
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

fn main() {
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
    contents.sort_by(|a,b| b.cmp(a));
    let res = contents.iter().take(3).sum::<u32>();
    println!("{:?}", res);
}

    // let mut a: Vec<u32> = vec![];
    //
    // let result = contents.iter().max().unwrap().to_owned();
    // println!("{:?}", result);
    // a.push(result);
    // let contents :Vec<u32>= contents.into_iter().filter(|f| result != *f).collect();
    // let result = contents.iter().max().unwrap().to_owned();
    // println!("{:?}", result);
    // a.push(result);
    // let contents :Vec<u32>= contents.into_iter().filter(|f| result != *f).collect();
    // let result = contents.iter().max().unwrap().to_owned();
    // println!("{:?}", result);
    // a.push(result);
    // let b :u32= a.into_iter().sum();

    println!("{:?}", b);
    



    // for elem in contents {
    //     println!("{:?}", elem);
    //     println!("{:?}", a);
    //     if !a.is_empty() {
    //
    //         println!("{:?}", a.last().unwrap() < &elem);
    //         println!("{:?}", a.last().unwrap());
    //     }
    //     if a.is_empty() || a.last().unwrap() < &elem {
    //         a.insert(0, elem)
    //     }
    // }
    //
    // let b: Vec<u32> = a.into_iter().take(3).collect();
}
