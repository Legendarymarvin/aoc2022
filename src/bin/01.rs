use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("No input :(");
    let mut elves = vec![];
    let mut calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!("{}", elves[0]);
    println!("{}", elves[0] + elves[1] + elves[2]);
}