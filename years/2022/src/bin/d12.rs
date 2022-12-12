use std::collections::VecDeque;

use lib::prelude::*;
use pathfinding::prelude::astar;
use regex::Regex;

fn nr_of_steps(grid: &Vec<Vec<u8>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> usize {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    astar(
        &start_pos,
        |&(x_c, y_c)| {
            let current = grid[x_c][y_c];
            vec![
                (x_c.wrapping_sub(1), y_c, current),
                (x_c + 1, y_c, current),
                (x_c, y_c + 1, current),
                (x_c, y_c.wrapping_sub(1), current),
            ]
            .into_iter()
            .filter_map(|(x, y, current)| {
                if x >= grid_height || y >= grid_width {
                    return None;
                }
                let other = grid[x][y];
                if current + 1 >= other {
                    return Some(((x, y), 1));
                }
                return None;
            })
        },
        |&(x, y)| (end_pos.0.abs_diff(x) + end_pos.1.abs_diff(y)),
        |&p| p == end_pos,
    )
    .unwrap_or_else(|| (vec![], usize::MAX))
    .1
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid = input
        .split("\n")
        .map(|a| a.as_bytes().to_vec())
        .collect_vec();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for i in 0..grid_height {
        for j in 0..grid_width {
            match grid[i][j] {
                b'S' => {
                    start_pos = (i, j);
                    grid[i][j] = b'a'
                }
                b'E' => {
                    end_pos = (i, j);
                    grid[i][j] = b'z'
                }
                _ => {}
            }
        }
    }
    Ok(nr_of_steps(&grid, start_pos, end_pos))
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut grid = input
        .split("\n")
        .map(|a| a.as_bytes().to_vec())
        .collect_vec();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut multiple_start_pos = vec![];
    let mut end_pos = (0, 0);

    for i in 0..grid_height {
        for j in 0..grid_width {
            match grid[i][j] {
                b'a' => {
                    multiple_start_pos.push((i, j));
                }
                b'E' => {
                    end_pos = (i, j);
                    grid[i][j] = b'z'
                }
                _ => {}
            }
        }
    }

    let res = multiple_start_pos
        .iter()
        .map(|&start_pos| nr_of_steps(&grid, start_pos, end_pos))
        .min()
        .unwrap();
    Ok(res)
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .map(|a| a.as_bytes().to_vec())
        .collect_vec()
}

fn main() -> Result<()> {
    let input = input!("d12.txt")?;

    run!("part_one", part_one(&input)?, 408);
    run!("part_two", part_two(&input)?, 399);
    Ok(())
}

// part_one: 408
//  took 234.04µs

// part_two: 399
//  took 42.561435ms
