use std::collections::HashSet;

use lib::prelude::*;
use regex::Regex;

pub fn part_one(input: &Vec<((i32, i32), (i32, i32))>) -> Result<usize> {
    let mut bcxs = HashSet::new();
    let row = 2000000;
    let zones = input
        .iter()
        .filter_map(|((sx, sy), (bx, by))| {
            let manhattan = (sx - bx).abs() + (sy - by).abs() - 1;
            let manhattan = manhattan - (row - sy).abs() + 1;
            if *by == row {
                bcxs.insert(*bx);
            }
            // println!("{manhattan}");
            if manhattan < 0 {
                return None;
            }
            let reachx = sx - manhattan;
            let reachy = sx + manhattan;
            return Some((reachx, reachy));
        })
        .collect_vec();

    let mut sensors: HashSet<i32> = HashSet::new();
    for (x, y) in zones {
        for i in x..=y {
            sensors.insert(i);
        }
    }
    for k in bcxs {
        sensors.remove(&k);
    }

    Ok(sensors.len())
}

pub fn part_two(input: &Vec<((i32, i32), (i32, i32))>) -> Result<usize> {
    Ok(0)
}

fn parse(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let regex = Regex::new(
        r"Sensor at x=([-]*\d+), y=([-]*\d+): closest beacon is at x=([-]*\d+), y=([-]*\d+)",
    )
    .unwrap();
    regex
        .captures_iter(input)
        .map(|cap| {
            let sensor_x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let sensor_y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_x = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_y = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
            ((sensor_x, sensor_y), (beacon_x, beacon_y))
        })
        .collect_vec()
}

fn main() -> Result<()> {
    let input = input!("d15.txt")?;
    let input = parse(&input);

    run!("part_one", part_one(&input)?, 4717631);
    run!("part_two", part_two(&input)?);
    Ok(())
}
