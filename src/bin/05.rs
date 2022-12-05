use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/05/input.txt").expect("No input :(");
    solve(&input);
}

fn solve(input: &String) {
    let mut split = input.split("\n\n");
    let map = split.next().unwrap();
    let moves = split.next().unwrap();
    let mut matrix = build_matrix(map);

    moves.split("\n")
        .for_each(|line| {
            let split: Vec<usize> = line.split(" ")
                .filter(|s| s.parse::<i32>().is_ok())
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let crates = split[0];
            let origin = split[1] - 1;
            let destination = split[2] - 1;
            let moving = &matrix[origin][0..crates].to_vec();
            moving
                .iter()
                // part 2 => .rev()
                .for_each(| c| {
                    matrix[destination].insert(0, c.clone());
                });
            matrix[origin].drain(0..crates);
        });

    println!("{:?}", matrix.iter().map(|row| row[0].clone()).collect::<Vec<String>>());
}

fn build_matrix(map: &str) -> Vec<Vec<String>> {
    let mut matrix: Vec<Vec<String>> = vec![vec![]; 9];
    map.split("\n")
        .filter(|line| !line.contains("1"))
        .for_each(|line| {
            for n in 0..=8 {
                match line.chars().nth(1 + n * 4) {
                    None => continue,
                    Some(char) => {
                        if char.is_alphabetic() {
                            let new_char = char.clone().to_string();
                            matrix[n].push(new_char);
                        }
                    }
                }
            }
        });
    matrix
}
