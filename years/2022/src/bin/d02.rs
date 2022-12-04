use lib::{bench, prelude::*};

pub fn part_one(input: &str) -> Result<u64> {
    let res: u64 = input
        .lines()
        .map(|line| {
            let p1 = line.chars().next().unwrap();
            let p2 = line.chars().nth(2).unwrap();
            match (p1, p2) {
                ('A', 'X') => 4,
                ('A', 'Y') => 8,
                ('A', 'Z') => 3,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 7,
                ('C', 'Y') => 2,
                ('C', 'Z') => 6,
                _ => unreachable!(),
            }
        })
        .sum();

    Ok(res)
}

pub fn part_two(input: &str) -> Result<u64> {
    let res: u64 = input
        .lines()
        .map(|line| {
            let p1 = line.chars().next().unwrap();
            let p2 = line.chars().nth(2).unwrap();
            match (p1, p2) {
                ('A', 'X') => 3,
                ('A', 'Y') => 4,
                ('A', 'Z') => 8,
                ('B', 'X') => 1,
                ('B', 'Y') => 5,
                ('B', 'Z') => 9,
                ('C', 'X') => 2,
                ('C', 'Y') => 6,
                ('C', 'Z') => 7,
                _ => unreachable!(),
            }
        })
        .sum();

    Ok(res)
}

fn main() -> Result<()> {
    let input = input!("d02.txt")?;
    run!("part_one", part_one(&input)?, 13484);
    run!("part_two", part_two(&input)?, 13433);
    Ok(())
}

// part_one: 13484
//  took 37.666µs

// part_two: 13433
//  took 38.392µs
