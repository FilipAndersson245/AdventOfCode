use lib::prelude::*;

pub fn part_one(input: &str) -> Result<u64> {
    let res = solver(input)[0];
    Ok(res)
}

pub fn part_two(input: &str) -> Result<u64> {
    let res = solver(input).iter().take(3).sum();
    Ok(res)
}

fn solver(input: &str) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|person| {
            person
                .split('\n')
                .map(|food| food.parse::<u64>().unwrap_or(0))
                .sum()
        })
        .sorted()
        .rev()
        .collect()
}

fn main() -> Result<()> {
    let input = lib::input!("d01.txt")?;
    run!("part_one", part_one(&input)?, 72240);
    run!("part_two", part_two(&input)?, 210957);
    Ok(())
}

// part_one: 72240
//  took 40.467µs

// part_two: 210957
//  took 40.875µs
