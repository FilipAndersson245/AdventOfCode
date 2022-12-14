use std::collections::VecDeque;

use lib::prelude::*;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(usize),
    Mult(usize),
    MultSelf,
}

pub struct Monkey {
    inventory: VecDeque<usize>,
    operator: Operation,
    test: usize,
    true_throw_to: usize,
    false_throw_to: usize,
    inspections: usize,
}

impl Monkey {
    fn inspect_and_throw(&mut self, divide_by_3: bool) -> (usize, usize) {
        // returns (target,worry_level)
        self.inspections += 1;
        let mut item = self.inventory.pop_front().unwrap();
        match self.operator {
            Operation::Add(n) => item += n,
            Operation::Mult(n) => item *= n,
            Operation::MultSelf => item *= item,
        }
        if divide_by_3 {
            item /= 3;
        }

        if item % self.test == 0 {
            (self.true_throw_to, item)
        } else {
            (self.false_throw_to, item)
        }
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let regex = Regex::new(
        r"Monkey (\d+):\n  Starting items: (.+)\n  Operation: new =(.+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)",
    ).unwrap();

    let mut monkeys = Vec::with_capacity(32);

    for cap in regex.captures_iter(input) {
        let inventory = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|digit| digit.parse::<usize>().unwrap())
            .collect();

        let operator = match cap
            .get(3)
            .unwrap()
            .as_str()
            .split_ascii_whitespace()
            .collect_vec()
            .as_slice()
        {
            ["old", "+", digit] => {
                let digit = digit.parse::<usize>().unwrap();
                Operation::Add(digit)
            }
            ["old", "*", "old"] => Operation::MultSelf,
            ["old", "*", digit] => {
                let digit = digit.parse::<usize>().unwrap();
                Operation::Mult(digit)
            }
            [..] => unreachable!(),
        };

        let test = cap.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let true_throw_to = cap.get(5).unwrap().as_str().parse::<usize>().unwrap();
        let false_throw_to = cap.get(6).unwrap().as_str().parse::<usize>().unwrap();
        monkeys.push(Monkey {
            inventory,
            operator,
            test,
            true_throw_to,
            false_throw_to,
            inspections: 0,
        })
    }

    monkeys
}

pub fn part_one(input: Vec<Monkey>) -> Result<usize> {
    let mut monkeys = input;

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].inventory.is_empty() {
                let next = monkeys[i].inspect_and_throw(true);
                monkeys[next.0].inventory.push_back(next.1);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
    Ok(monkeys[0].inspections * monkeys[1].inspections)
}

pub fn part_two(input: Vec<Monkey>) -> Result<usize> {
    let mut monkeys = input;

    let mut destress = 1;
    for monkey in &monkeys {
        destress *= monkey.test;
    }

    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].inventory.is_empty() {
                let next = monkeys[i].inspect_and_throw(false);
                monkeys[next.0].inventory.push_back(next.1 % destress);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspections.partial_cmp(&a.inspections).unwrap());
    Ok(monkeys[0].inspections * monkeys[1].inspections)
}

fn main() -> Result<()> {
    let input = input!("d11.txt")?;

    run!("part_one", part_one(parse(&input))?);
    run!("part_two", part_two(parse(&input))?);
    Ok(())
}
