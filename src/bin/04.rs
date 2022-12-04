use std::fs;
use std::str::Split;

fn main() {
    let input = fs::read_to_string("inputs/04/input.txt").expect("No input :(");
    println!("{}", solve(&input, "1"));
    println!("{}", solve(&input, "2"));
}

fn solve(input: &str, part: &str) -> usize {
    input.split("\n")
        .map(|line| {
            let mut split = line.split(",");
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(left, right)| {
            let mut left = left.split("-");
            let mut right = right.split("-");
            ((parse_next(&mut left), parse_next(&mut left)), (parse_next(&mut right), parse_next(&mut right)) )
        })
        .filter(|((lLower, lUpper), (rLower, rUpper))| {
            (part.contains("1") && (lLower <= rLower && lUpper >= rUpper) || (rLower <= lLower && rUpper >= lUpper))
                ||
            (part.contains("2") && (lLower <= rLower && lUpper >= rLower) || (rLower <= lLower && rUpper >= lLower))
        })
        .count()
}

fn parse_next(split: &mut Split<&str>) -> i32 {
    split.next().unwrap().parse::<i32>().unwrap()
}