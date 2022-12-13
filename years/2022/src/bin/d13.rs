use std::cmp::Ordering;

use serde_derive::Deserialize;

use lib::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Number(x), Packet::Number(y)) => x.partial_cmp(y),
            (Packet::List(x), Packet::List(y)) => x.partial_cmp(y),
            (Packet::Number(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Number(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(|pairs| {
            let mut iter = pairs.split("\n");
            let left: Packet = serde_json::from_str(iter.next().unwrap()).unwrap();
            let right: Packet = serde_json::from_str(iter.next().unwrap()).unwrap();
            (left, right)
        })
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .fold(0, |acc, (idx, _)| acc + idx + 1))
}

pub fn part_two(input: &str) -> Result<usize> {
    let input = format!("\n[[6]]\n[[2]]\n{}", input);
    let data = input
        .split('\n')
        .filter(|&s| s.starts_with('['))
        .map(|s| {
            let packet: Packet = serde_json::from_str(s).unwrap();
            packet
        })
        .sorted()
        .collect_vec();

    let del1: Packet = serde_json::from_str("[[6]]").unwrap();
    let del2: Packet = serde_json::from_str("[[2]]").unwrap();
    let index1 = (data.binary_search(&del1)).unwrap() + 1;
    let index2 = data.binary_search(&del2).unwrap() + 1;

    Ok(index1 * index2)
}

fn main() -> Result<()> {
    let input = input!("d13.txt")?;

    run!("part_one", part_one(&input)?, 4643);
    run!("part_two", part_two(&input)?, 21614);
    Ok(())
}

// part_one: 4643
//  took 1.083162ms

// part_two: 21614
//  took 1.31186ms
