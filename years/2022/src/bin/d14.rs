use fasthash::sea::Hash64;
use std::collections::HashMap;

use lib::prelude::*;

fn parse_point(s: &str) -> (usize, usize) {
    s.split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect_tuple::<_>()
        .unwrap()
}

fn min_to_max(a: usize, b: usize) -> std::ops::RangeInclusive<usize> {
    a.min(b)..=a.max(b)
}

pub fn part_one(input: &str) -> Result<usize> {
    // let mut grid = vec![vec![' '; 1000]; 1000];
    let mut largestst_y = 0;
    let mut grid_map = HashMap::with_capacity_and_hasher(8192, Hash64);
    input.lines().for_each(|line| {
        let mut iter = line.split(" -> ").map(parse_point);
        let mut from = iter.next().unwrap();

        while let Some(to) = iter.next() {
            for x in min_to_max(from.0, to.0) {
                for y in min_to_max(from.1, to.1) {
                    if y > largestst_y {
                        largestst_y = y;
                    }
                    grid_map.insert((x, y), '#');
                }
            }
            from = to;
        }
    });

    let sand_starting_pos = (500, 0);
    let mut sand = 0;
    'outer: loop {
        let mut sand_pos = sand_starting_pos;
        loop {
            if sand_pos.1 == largestst_y {
                break 'outer;
            }

            let sand_down = (sand_pos.0, sand_pos.1 + 1);
            let sand_down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let sand_down_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            if !grid_map.contains_key(&sand_down) {
                sand_pos = sand_down;
            } else if !grid_map.contains_key(&sand_down_left) {
                sand_pos = sand_down_left;
            } else if !grid_map.contains_key(&sand_down_right) {
                sand_pos = sand_down_right;
            } else {
                grid_map.insert(sand_pos, '+');
                sand += 1;
                break;
            }
        }
    }
    Ok(sand)
}

pub fn part_two(input: &str) -> Result<usize> {
    // let mut grid = vec![vec![' '; 1000]; 1000];
    let mut largestst_y = 0;
    let mut grid_map = HashMap::with_capacity_and_hasher(8192, Hash64);
    input.lines().for_each(|line| {
        let mut iter = line.split(" -> ").map(parse_point);
        let mut from = iter.next().unwrap();

        while let Some(to) = iter.next() {
            for x in min_to_max(from.0, to.0) {
                for y in min_to_max(from.1, to.1) {
                    if y > largestst_y {
                        largestst_y = y;
                    }
                    grid_map.insert((x, y), '#');
                }
            }
            from = to;
        }
    });

    let lava = largestst_y + 2;
    for x in 0..=1000 {
        grid_map.insert((x, lava), '#');
    }

    let sand_starting_pos = (500, 0);
    let mut sand = 0;
    'outer: loop {
        let mut sand_pos = sand_starting_pos;
        loop {
            let sand_down = (sand_pos.0, sand_pos.1 + 1);
            let sand_down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
            let sand_down_right = (sand_pos.0 + 1, sand_pos.1 + 1);

            if !grid_map.contains_key(&sand_down) {
                sand_pos = sand_down;
            } else if !grid_map.contains_key(&sand_down_left) {
                sand_pos = sand_down_left;
            } else if !grid_map.contains_key(&sand_down_right) {
                sand_pos = sand_down_right;
            } else {
                grid_map.insert(sand_pos, '+');
                sand += 1;
                if sand_pos == sand_starting_pos {
                    break 'outer;
                };
                break;
            }
        }
    }
    Ok(sand)
}

fn main() -> Result<()> {
    let input = input!("d14.txt")?;

    run!("part_one", part_one(&input)?, 614);
    run!("part_two", part_two(&input)?, 26170);
    Ok(())
}

// part_one: 614
//  took 367.664µs

// part_two: 26170
//  took 18.195992ms
