use lib::prelude::*;
use std::collections::HashSet;

pub fn solver(input: &str, chunk_size: usize) -> Result<usize> {
    let input = input.chars().collect_vec();
    for i in 0..(input.len() - chunk_size) {
        let chunk = &input[i..(i + chunk_size)];
        let set: HashSet<_> = chunk.iter().collect();
        if set.len() == chunk_size {
            return Ok(i + chunk_size);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Result<usize> {
    solver(input, 4)
}

pub fn part_two(input: &str) -> Result<usize> {
    solver(input, 14)
}

fn main() -> Result<()> {
    let input = input!("d06.txt")?;
    run!("part_one", part_one(&input)?);
    run!("part_two", part_two(&input)?);
    Ok(())
}

// part_one: 1210
//  took 85.263µs

// part_two: 3476
//  took 756.983µs
