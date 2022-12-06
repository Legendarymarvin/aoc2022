use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/06/input.txt").expect("No input :(");
    println!("{}", solve(&input, 4));
    println!("{}", solve(&input, 14));
}

fn solve(input: &String, length: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut prevs: Vec<char> = vec![];

    for i in 0..chars.len() {
        for j in 0..length {
            if prevs.len() >= length {
                return i + length - 1;
            }
            if prevs.contains(&chars[i+j]) {
                prevs = vec![];
                break;
            } else {
                prevs.push(chars[i+j]);
            }
        }
    }
    panic!("Something somewhere somehow went terribly wrong");
}