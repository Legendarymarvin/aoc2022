use std::{collections::HashMap, path::PathBuf, str::FromStr, fs};

#[derive(Default)]
struct Contents {
    dirs: Vec<PathBuf>,
    files: i32,
}

pub fn main() {
    let input = fs::read_to_string("inputs/07/input.txt").expect("No input :(");
    let dirs = read_file_system(&input);
    let sizes: Vec<i32> = dirs
        .iter()
        .map(|e| calculate_size(&dirs, e.0))
        .collect();
    println!("Part 1: {}", sizes.iter().filter(|size| *size < &100_000).sum::<i32>());

    let root_size = sizes.iter().max().unwrap();
    // let root_size = calculate_size(&dirs, &PathBuf::from_str("/").unwrap());
    let free_space = 70_000_000 - root_size;
    let needed = 30_000_000 - free_space;
    println!("Part 2: {}", sizes.iter()
        .filter(|&size| {
            size >= &needed
        })
        .min().unwrap());
}

fn read_file_system(input: &str) -> HashMap<PathBuf, Contents> {
    let mut file_system: HashMap<PathBuf, Contents> = HashMap::new();
    let mut cwd = PathBuf::from_str("/").unwrap();

    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd ..") {
                cwd.pop();
            } else if line.starts_with("$ cd") {
                file_system.entry(cwd.clone()).or_default();
                cwd.push(line.split(" ").nth(2).unwrap());
            }
            // ls gets implicitly ignored
            continue;
        } else {
            if line.starts_with("dir") {
                let name = line.split(" ").nth(1).unwrap();
                let path= cwd.join(name);

                file_system.entry(path.clone()).or_default();
                file_system
                    .get_mut(&cwd)
                    .unwrap()
                    .dirs
                    .push(path)
            } else {
                let size = line.split(" ").nth(0).unwrap().parse::<i32>().unwrap();
                file_system
                    .get_mut(&cwd)
                    .unwrap()
                    .files += size;
            }
        }
    }

    file_system
}

fn calculate_size(tree: &HashMap<PathBuf, Contents>, path: &PathBuf) -> i32 {
    let dir_size = tree[path]
        .dirs
        .iter()
        .map(|dir| calculate_size(tree, &dir))
        .sum::<i32>();
    tree[path].files + dir_size
}
