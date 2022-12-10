use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/10/input.txt").expect("No input :(");
    let mut strengths = vec![];
    let mut cycle = 0;
    let mut next_cycle = 20;
    let mut x = 1;

    let mut grid: Vec<char> = vec![];

    for line in input.lines() {
        cycle += 1;
        draw(&mut grid, cycle, x);

        if cycle >= next_cycle {
            strengths.push(x * cycle);
            next_cycle += 40;
        }

        if line.starts_with("noop") {
            continue;
        }

        let (op, val) = line.split_once(" ").unwrap();
        let val = val.parse::<i32>().unwrap();

        if op == "addx" {
            cycle += 1;
            if cycle >= next_cycle {
                strengths.push(x * cycle);
                next_cycle += 40;
            }
            x += val;
            draw(&mut grid, cycle, x);
        }
    }
    println!("Part1: {:?}", strengths.iter().sum::<i32>());

    let mut i = 0;
    loop {
        println!("{:?}", &grid[i..i+39].iter().collect::<String>());
        i += 40;
        if i >= grid.len() {
            break;
        }
    }
}

fn draw(grid: &mut Vec<char>, cycle: i32, x: i32) {
    let sprite = cycle % 40;
    if x == sprite || x == sprite - 1 || x == sprite + 1 {
        grid.push('#');
    } else {
        grid.push('.');
    }
}

