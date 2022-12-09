use std::collections::HashSet;

use lib::prelude::*;

fn solver(moves: &[(i32, i32, usize)], followers: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); followers + 1];
    let mut visited: HashSet<(i32, i32)> = HashSet::with_capacity(1024);
    visited.insert((0, 0));
    for &(dx, dy, steps) in moves {
        for _ in 0..steps {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..rope.len() {
                let (xh, yh) = rope[i - 1];
                if (xh - rope[i].0).abs() > 1 || (yh - rope[i].1).abs() > 1 {
                    rope[i].0 += (xh - rope[i].0).signum();
                    rope[i].1 += (yh - rope[i].1).signum();
                }
            }
            visited.insert(rope[followers]);
        }
    }
    visited.len()
}

fn parse(input: &str) -> Vec<(i32, i32, usize)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            let (dx, dy) = match iter.next().unwrap() {
                "U" => (0, 1),
                "D" => (0, -1),
                "R" => (1, 0),
                "L" => (-1, 0),
                _ => unreachable!(),
            };
            let step_size = iter.next().unwrap().parse::<usize>().unwrap();
            (dx, dy, step_size)
        })
        .collect_vec()
}

pub fn part_one(input: &[(i32, i32, usize)]) -> Result<usize> {
    Ok(solver(&input, 1))
}

pub fn part_two(input: &[(i32, i32, usize)]) -> Result<usize> {
    Ok(solver(&input, 9))
}

fn main() -> Result<()> {
    let input = input!("d09.txt")?;

    let parsed_input = parse(&input);
    run!("part_one", part_one(&parsed_input)?, 6486);
    run!("part_two", part_two(&parsed_input)?, 2678);
    Ok(())
}

// part_one: 6486
//  took 351.358µs

// part_two: 2678
//  took 464.906µs
