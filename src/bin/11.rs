use std::fs;
use std::collections::{HashMap};

#[derive(Debug, Clone)]
enum Opp {
    Add(Option<u64>),
    Mul(Option<u64>),
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u64,
    items: Vec<u64>,
    test: u64,
    first_target: u64,
    second_target: u64,
    inspection_counter: u64,
    operation: Opp,
}

impl Monkey {
    fn new(id: u64, items: Vec<u64>, test: u64, first_target: u64, second_target: u64, operation: Opp) -> Monkey {
        Monkey {
            id,
            items,
            test,
            first_target,
            second_target,
            operation,
            inspection_counter: 0,
        }
    }
}

fn test(tester: u64, item: u64) -> bool {
    item % tester == 0
}

fn main() {
    let input = fs::read_to_string("inputs/11/input.txt").expect("No input :(");
    let mut monkeys: Vec<Monkey> = vec![];

    for chunk in input.split("\n\n") {
        let lines: Vec<&str> = chunk.lines().collect();
        let id = lines[0].split("y ").skip(1).next().unwrap().replace(":", "").parse::<u64>().unwrap();
        let items = lines[1].split(": ").skip(1).next().unwrap().split(", ").map(|x| x.parse::<u64>().unwrap()).collect();
        let opp = match lines[2].contains("*") {
            true => Opp::Mul(lines[2].split("* ").skip(1).next().unwrap().parse::<u64>().ok()),
            false => Opp::Add(lines[2].split("+ ").skip(1).next().unwrap().parse::<u64>().ok()),
        };
        let test = lines[3].split("by ").skip(1).next().unwrap().parse::<u64>().unwrap();
        let first_target = lines[4].split("monkey ").skip(1).next().unwrap().parse::<u64>().unwrap();
        let second_target = lines[5].split("monkey ").skip(1).next().unwrap().parse::<u64>().unwrap();
        monkeys.push(Monkey::new(id, items, test, first_target, second_target, opp));
    }

    let mut throws: HashMap<u64, Vec<u64>> = HashMap::new();
    let modulo: u64 = monkeys.iter().map(|x| x.test).product();
    //for _ in 1..=20 {
    for _ in 1..=10000 {
        for monkey in &mut monkeys {
            if throws.contains_key(&monkey.id) {
                monkey.items.append(&mut throws.get_mut(&monkey.id).unwrap());
                throws.remove(&monkey.id);
            }

            for item in &monkey.items {
                monkey.inspection_counter += 1;
                let mut new_worry = item.clone();
                match monkey.operation {
                    Opp::Add(Some(x)) => new_worry += x,
                    Opp::Mul(Some(x)) => new_worry *= x,
                    Opp::Add(None) => new_worry += item.clone(),
                    Opp::Mul(None) => new_worry *= item.clone(),
                }

                new_worry = new_worry % modulo;
                //Part 1:
                //new_worry = new_worry / 3;

                match test(monkey.test, new_worry) {
                    true => throws.entry(monkey.first_target).or_insert(vec![]).push(new_worry),
                    false => throws.entry(monkey.second_target).or_insert(vec![]).push(new_worry),
                };
            }

            monkey.items = vec![];
        }
    }

    monkeys.sort_by(|a, b| a.inspection_counter.cmp(&b.inspection_counter));
    monkeys.reverse();
    println!("{}", monkeys[0].inspection_counter * monkeys[1].inspection_counter);

}