use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
    str::FromStr,
};

use ahash::RandomState;
use lib::prelude::*;
use pathfinding::directed::astar;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
struct Grid {
    start: (i64, i64),
    end: (i64, i64),
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    blizzards_cache: HashMap<usize, HashSet<(i64, i64), RandomState>, RandomState>,
}

#[derive(Clone, Debug)]
struct Blizzard {
    start: (i64, i64),
    direction: Direction,
}

impl Blizzard {
    fn pos(&self, time: usize) -> (i64, i64) {
        match self.direction {
            Direction::Left => (self.start.0 - time as i64, self.start.1),
            Direction::Right => (self.start.0 + time as i64, self.start.1),
            Direction::Top => (self.start.0, self.start.1 - time as i64),
            Direction::Bottom => (self.start.0, self.start.1 + time as i64),
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect_vec();

        let start = (input.find('.').unwrap() as i64, 0i64);
        let end = (
            lines.last().unwrap().rfind('.').unwrap() as i64,
            lines.len() as i64 - 1,
        );

        let width = lines.first().unwrap().len();
        let height = (end.1 - start.1) as usize + 1;

        let mut blizzards = vec![];

        for (y, row) in lines.iter().enumerate() {
            for (x, cell) in row.chars().enumerate() {
                if matches!(cell, '#' | '.') {
                    continue;
                }
                let direction = match cell {
                    '^' => Direction::Top,
                    '>' => Direction::Right,
                    'v' => Direction::Bottom,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                };

                let blizzard = Blizzard {
                    start: (x as i64, y as i64),
                    direction,
                };
                blizzards.push(blizzard);
            }
        }

        let blizzards_cache = HashMap::default();

        Ok(Self {
            start,
            end,
            width,
            height,
            blizzards,
            blizzards_cache,
        })
    }
}

impl Grid {
    fn within_bounds(&self, square: &(i64, i64)) -> bool {
        square.0 > 0
            && square.0 < self.width as i64 - 1
            && (square.1 > 0 || *square == self.start)
            && (square.1 < self.height as i64 - 1 || *square == self.end)
    }

    fn pre_calculate_blizzards(&mut self, iterations: usize) {
        let already_cached = self.blizzards_cache.len();
        for epoch in already_cached..iterations {
            let mut next_blizzards = HashSet::default();

            for blizzard in &self.blizzards {
                let mut blizzard_pos = blizzard.pos(epoch);

                blizzard_pos.0 = (blizzard_pos.0 - 1).rem_euclid(self.width as i64 - 2) + 1;
                blizzard_pos.1 = (blizzard_pos.1 - 1).rem_euclid(self.height as i64 - 2) + 1;
                next_blizzards.insert(blizzard_pos);
            }
            self.blizzards_cache.insert(epoch, next_blizzards);
        }
    }

    fn travel(&mut self, start: (i64, i64), end: (i64, i64), start_min: usize) -> Result<usize> {
        let start_node = (start, start_min);
        let time = astar::astar(
            &start_node,
            |(pos, time)| {
                let time = time + 1;
                let (px, py) = *pos;

                vec![
                    (*pos, time),
                    ((px + 1, py), time),
                    ((px - 1, py), time),
                    ((px, py + 1), time),
                    ((px, py - 1), time),
                ]
                .into_iter()
                .filter(|(pos, time)| {
                    return !self.blizzards_cache.get(time).unwrap().contains(pos)
                        && self.within_bounds(pos);
                })
                .map(|p| (p, 1))
            },
            |((x, y), _)| i64::abs(x.sub(end.0)) + i64::abs(y.sub(end.1)),
            |(pos, _)| pos.0 == end.0 && pos.1 == end.1,
        )
        .unwrap()
        .0
        .last()
        .unwrap()
        .1;
        Ok(time)
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut grid = Grid::from_str(input).unwrap();
    grid.pre_calculate_blizzards(373 + 1); // depending on input increase this size.
    Ok(grid.travel(grid.start, grid.end, 0).unwrap())
}

pub fn part_two(input: &str) -> Result<usize> {
    let mut grid = Grid::from_str(input).unwrap();
    grid.pre_calculate_blizzards(997 + 1); // depending on input increase this size.
    let t = grid.travel(grid.start, grid.end, 0).unwrap();
    let t = grid.travel(grid.end, grid.start, t).unwrap();
    let t = grid.travel(grid.start, grid.end, t).unwrap();
    Ok(t)
}

fn main() -> Result<()> {
    let input = input!("d24.txt")?;

    run!("part_one", part_one(&input)?, 373);
    run!("part_two", part_two(&input)?, 997);
    Ok(())
}

// part_one: 373
//  took 47.991848ms

// part_two: 997
//  took 113.772351ms
