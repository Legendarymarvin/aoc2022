use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/03/input.txt").expect("No input :(");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    println!("{}", input.split("\n")
        .map(|line: &str| line.split_at(line.len() / 2))
        .map(|(left, right)| {
            let first = left.chars().collect::<HashSet<char>>();
            let second = right.chars().collect::<HashSet<char>>();
            let common = first.intersection(&second);
            common.clone().collect::<String>().chars().collect::<Vec<char>>()[0]
        })
        .map(|c| get_priority(c))
        .sum::<i32>());
}

fn get_priority(c: char) -> i32 {
    if c.is_ascii_uppercase() {
        c as i32 - 64 + 26
    } else {
        c as i32 - 96
    }
}

fn part2(input: &String) {
    let lines:Vec<&str> = input.split("\n").collect();

    println!("{}", lines.chunks(3)
        .map(|chunk| sum_badge(chunk))
        .sum::<i32>());
}

fn sum_badge(chunk: &[&str]) -> i32 {
    let first: HashSet<char> = chunk[0].chars().collect();
    let second: HashSet<char> = chunk[1].chars().collect();
    let third: HashSet<char> = chunk[2].chars().collect();
    let fise: HashSet<char> = first.intersection(&second).copied().collect();
    let common: HashSet<char> = fise.intersection(&third).copied().collect();
    get_priority(common.iter().next().unwrap().clone())
}
