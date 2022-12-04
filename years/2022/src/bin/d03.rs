use std::collections::HashSet;

use lib::{bench, prelude::*};

fn char_to_value(ch: char) -> u64 {
    match ch {
        'a'..='z' => ch as u64 - 96,
        'A'..='Z' => ch as u64 - 38,
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Result<u64> {
    let input = input.lines().map(|n| n.split_at(n.len() / 2));
    let mut tot = 0;
    for (first, last) in input {
        let last: HashSet<char> = HashSet::from_iter(last.chars());
        let ch = first.chars().find(|it| last.contains(it)).unwrap();
        tot += char_to_value(ch)
    }
    Ok(tot)
}

pub fn part_two(input: &str) -> Result<u64> {
    let mut input = input.lines().peekable();
    let mut tot = 0;
    while input.peek().is_some() {
        let first = input.next().unwrap();
        let second: HashSet<char> = HashSet::from_iter(input.next().unwrap().chars());
        let third: HashSet<char> = HashSet::from_iter(input.next().unwrap().chars());
        let ch = first
            .chars()
            .find(|it| second.contains(it) && third.contains(it))
            .unwrap();
        tot += char_to_value(ch)
    }
    Ok(tot)
}

fn main() -> Result<()> {
    let input = input!("d03.txt")?;
    run!("part_one", part_one(&input)?, 7742);
    run!("part_two", part_two(&input)?, 2276);
    Ok(())
}

// part_one: 7742
//  took 141.412µs

// part_two: 2276
//  took 161.193µs
