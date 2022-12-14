use std::ops::AddAssign;

use lib::prelude::*;

pub fn part_one(input: &str) -> Result<u64> {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| char::to_digit(ch, 10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let mut count = 0;

    for x in 0..width {
        for y in 0..height {
            let mut bads = vec![];
            for (dx, dy) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
                let start = grid[x][y];
                let mut bad = false;
                let mut ox = x;
                let mut oy = y;

                loop {
                    if let Some(v) = ox.checked_add_signed(dx) {
                        ox = v;
                    } else {
                        break;
                    }

                    if let Some(v) = oy.checked_add_signed(dy) {
                        oy = v;
                    } else {
                        break;
                    }

                    if let Some(v) = grid.get(ox).and_then(|vec| vec.get(oy)) {
                        if *v >= start {
                            bad = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                bads.push(bad);
            }
            if bads.iter().any(|b| !(*b)) {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn part_two(input: &str) -> Result<u64> {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|ch| char::to_digit(ch, 10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let mut best = 0;

    for x in 0..width {
        for y in 0..height {
            let mut bads = vec![];
            let mut dists = vec![];
            for (dx, dy) in [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)] {
                let start = grid[x][y];
                let mut bad = false;
                let mut ox = x;
                let mut oy = y;
                dists.push(0);

                loop {
                    if let Some(v) = ox.checked_add_signed(dx) {
                        ox = v;
                    } else {
                        break;
                    }

                    if let Some(v) = oy.checked_add_signed(dy) {
                        oy = v;
                    } else {
                        break;
                    }

                    if let Some(v) = grid.get(ox).and_then(|vec| vec.get(oy)) {
                        if *v >= start {
                            bad = true;
                            dists.last_mut().unwrap().add_assign(1);
                            break;
                        }
                    } else {
                        break;
                    }

                    dists.last_mut().unwrap().add_assign(1);
                }
                bads.push(bad);
            }

            let contender = dists.iter().product();
            best = best.max(contender);
        }
    }

    Ok(best)
}

fn main() -> Result<()> {
    let input = input!("d08.txt")?;
    run!("part_one", part_one(&input)?, 1854);
    run!("part_two", part_two(&input)?, 527340);
    Ok(())
}

// part_one: 1854
//  took 606.825µs

// part_two: 527340
//  took 827.102µs
