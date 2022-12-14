use std::collections::HashSet;

use lib::prelude::*;
use pathfinding::prelude::{astar, bfs};

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid = input
        .split('\n')
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
    Ok(astar(
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
                None
            })
        },
        |&(x, y)| (end_pos.0.abs_diff(x) + end_pos.1.abs_diff(y)),
        |&p| p == end_pos,
    )
    .unwrap_or_else(|| (vec![], usize::MAX))
    .1)
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut grid = input
        .split('\n')
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

    let multiple_start_pos: HashSet<_> = HashSet::from_iter(multiple_start_pos.into_iter());

    let res = bfs(
        &end_pos,
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
                if current <= other + 1 {
                    return Some((x, y));
                }
                None
            })
        },
        |&p| multiple_start_pos.contains(&p),
    ).unwrap_or_default()
    .len()
        - 1;

    Ok(res)
}

fn main() -> Result<()> {
    let input = input!("d12.txt")?;

    run!("part_one", part_one(&input)?, 408);
    run!("part_two", part_two(&input)?, 399);
    Ok(())
}

// part_one: 408
//  took 249.759µs

// part_two: 399
//  took 207.564µs
