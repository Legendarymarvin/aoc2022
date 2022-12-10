use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/08/input.txt").expect("No input :(");
    let mut grid: Vec<Vec<u32>> = vec![];

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            grid = vec![vec![0; line.len()]; line.len()];
        }
        for (j,x) in line.chars().enumerate() {
            grid[i][j] = x.to_digit(10).unwrap();
        }
    }

    let max = grid[0].len();
    let mut count = 0;
    for i in 0..max {
        for j in 0..max {
            if is_visible(&grid, i, j, max) {
                count += 1;
            }
        }
    }

    println!("Part 1: {:?}", count);


    let mut score = 0;
    for i in 0..max {
        for j in 0..max {
            let new_score = calc_score(&grid, i, j, max, score);
            if new_score > score {
                score = new_score;
            }
        }
    }
    println!("Part 2: {:?}", score);
}

fn calc_score(grid: &Vec<Vec<u32>>, y: usize, x: usize, max: usize, score: i32) -> i32 {
    let height = grid[y][x];

    let mut scores = vec![];
    let mut new_score = 0;
    for i in 1..=x {
        new_score += 1;
        if grid[y][x-i] >= height {
            break;
        }
    }
    scores.push(new_score);

    new_score = 0;
    for i in x+1..max {
        new_score += 1;
        if grid[y][i] >= height {
            break;
        }
    }
    scores.push(new_score);

    new_score = 0;
    for i in 1..=y {
        new_score += 1;
        if grid[y-i][x] >= height {
            break;
        }
    }
    scores.push(new_score);

    new_score = 0;
    for i in y+1..max {
        new_score += 1;
        if grid[i][x] >= height {
            break;
        }
    }
    scores.push(new_score);

    new_score = 1;
    scores.iter().for_each(|x| new_score *= x);
    println!("{} {} {} {} {:?}", y, x, height, new_score, scores);
    new_score
}

fn is_visible(grid: &Vec<Vec<u32>>, y: usize, x: usize, max: usize) -> bool {
    if x == 0 || y == 0 || x == max - 1 || y == max - 1 {
        return true;
    }
    let height = grid[y][x];

    let mut visible = true;
    for i in 0..x {
        visible = visible && grid[y][i] < height;
    }
    if visible {
        return true;
    }

    visible = true;
    for i in x+1..max {
        visible = visible && grid[y][i] < height;
    }
    if visible {
        return true;
    }

    visible = true;
    for i in 0..y {
        visible = visible && grid[i][x] < height;
    }
    if visible {
        return true;
    }

    visible = true;
    for i in y+1..max {
        visible = visible && grid[i][x] < height;
    }
    visible
}