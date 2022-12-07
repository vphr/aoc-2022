use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone)]
enum ContainingFiles {
    SubDirectory(String),
    FileSize(usize),
}
fn main() {
    let mut file = File::open("./src/bin/day7.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    solve(&contents);
}
fn solve(input: &str) {
    let mut hash: HashMap<String, Vec<ContainingFiles>> = HashMap::new();
    let mut current_dir_path: Vec<&str> = vec![];
    for line in input.lines() {
        let mut files_in_this_dir: Vec<ContainingFiles> = vec![];
        if line.starts_with('$') {
            let line = line.strip_prefix("$ ").unwrap();
            if line.cmp("ls").is_eq() {
            } else {
                let (action, target) = line.split_once(" ").unwrap();
                if target.cmp("..").is_eq() {
                    current_dir_path.pop();
                } else if action.cmp("cd").is_eq() {
                    current_dir_path.push(target);
                }
            }
        } else {
            let (file_type, file_name) = line.split_once(' ').unwrap();
            if file_type.contains("dir") {
                // println!("{}", file_name);
                files_in_this_dir.push(ContainingFiles::SubDirectory(file_name.to_owned()));
            } else {
                let file_size = file_type.parse::<usize>().unwrap();
                files_in_this_dir.push(ContainingFiles::FileSize(file_size));
            }
            let mut tttt: Vec<_> = files_in_this_dir.iter().cloned().collect();
            let formatted_hashkey = current_dir_path.join("");
            if hash.get_mut(&formatted_hashkey).is_some() {
                hash.entry(formatted_hashkey).and_modify(|files| files.append(&mut tttt));
            } else {
                hash.insert(formatted_hashkey, tttt);
            }
        }
    }
    // println!("{:#?}", hash);
    let answer1: usize = hash
        .clone()
        .iter()
        .map(|(a, v)| get_size_of_subdir(&hash, v, a))
        .filter(|c| c < &100000)
        .sum();

    println!("part 1 {:?} ", answer1,);
    let mut parsed_directories: Vec<_> = hash
        .iter()
        .map(|(a, v)| (a, get_size_of_subdir(&hash, v, a)))
        .collect();
    let max_val: usize = 70000000;
    let max_entry: usize = 50822529;
    let space_required: usize = 30000000;
    let space_available = max_val - max_entry;
    let to_remove = space_required - space_available;

    parsed_directories.sort_by(|(_, a), (_, b)| a.cmp(b));
    // println!("{:#?}", tmp);
    let mut answer: Vec<_> = parsed_directories.iter().filter(|(_, c)| c > &to_remove).collect();
    answer.sort_by(|(_, a), (_, b)| a.cmp(b));
    let answer = answer.first().unwrap();
    println!("part 2 {:?} ", answer.1,);
}

fn get_size_of_subdir(
    h: &HashMap<String, Vec<ContainingFiles>>,
    input: &[ContainingFiles],
    subdir: &str,
) -> usize {
    let sum = input
        .iter()
        .map(|vv| match vv {
            ContainingFiles::SubDirectory(k) => {
                let formatted_key = &format!("{}{}", subdir, k);
                get_size_of_subdir(h, h.get(formatted_key).unwrap(), formatted_key)
            }
            ContainingFiles::FileSize(s) => *s,
        })
        .sum();
    sum
}
