use lib::prelude::*;
use lib::rayon::prelude::*;
use std::collections::HashSet;

pub fn solver(input: &str, chunk_size: usize) -> Result<usize> {
    let input = input.bytes().collect_vec();
    for i in 0..(input.len() - chunk_size) {
        let chunk = &input[i..(i + chunk_size)];
        let set: HashSet<_> = chunk.iter().collect();
        if set.len() == chunk_size {
            return Ok(i + chunk_size);
        }
    }
    unreachable!()
}

pub fn solver_p(input: &str, chunk_size: usize) -> Result<usize> {
    let input = input.bytes().collect_vec();
    (0..input.len() - chunk_size)
        .into_par_iter()
        .find_first(|&idx| {
            let chunk = &input[idx..(idx + chunk_size)];
            let set: HashSet<_> = chunk.iter().collect();
            set.len() == chunk_size
        })
        .map(|idx| idx + chunk_size)
        .ok_or_else(|| unreachable!())
}

pub fn part_one(input: &str) -> Result<usize> {
    solver(input, 4)
}

pub fn part_one_p(input: &str) -> Result<usize> {
    solver_p(input, 4)
}

pub fn part_two(input: &str) -> Result<usize> {
    solver(input, 14)
}

pub fn part_two_p(input: &str) -> Result<usize> {
    solver_p(input, 14)
}

fn main() -> Result<()> {
    let input = input!("d06.txt")?;
    run!("part_one", part_one(&input)?, 1210);
    run!("part_one_par", part_one_p(&input)?, 1210);
    run!("part_two", part_two(&input)?, 3476);
    run!("part_two_par", part_two_p(&input)?, 3476);
    Ok(())
}

// part_one: 1210
//  took 84.21µs

// part_one_par: 1210
//  took 42.08µs

// part_two: 3476
//  took 754.368µs

// part_two_par: 3476
//  took 249.933µs
