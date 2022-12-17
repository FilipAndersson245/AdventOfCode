use std::{collections::HashSet, ops::AddAssign};

use lib::prelude::*;

const CHAMBER_WIDTH: i32 = 7;

const SHAPES: [(usize, [(i32, i32); 5]); 5] = {
    [
        (4, [(0, 0), (1, 0), (2, 0), (3, 0), (0, 0)]),
        (5, [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        (5, [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        (4, [(0, 0), (0, 1), (0, 2), (0, 3), (0, 0)]),
        (4, [(0, 0), (0, 1), (1, 0), (1, 1), (0, 0)]),
    ]
};

pub struct Chamber {
    movements: Vec<i32>,
    curr_shape: Vec<(i32, i32)>,
    occupied: HashSet<(i32, i32)>,
    stopped_rocks: usize,
    height: usize,
    is_next_down: bool,
    move_clock: usize,
    next_shape_idx: usize,
}

impl Chamber {
    pub fn new(line: &str) -> Self {
        Chamber {
            occupied: HashSet::with_capacity(2048),
            curr_shape: Vec::with_capacity(8),
            next_shape_idx: 0,
            stopped_rocks: 0,
            height: 0,
            is_next_down: false,
            move_clock: 0,
            movements: line
                .chars()
                .map(|ch| match ch {
                    '<' => -1,
                    '>' => 1,
                    _ => unreachable!(),
                })
                .collect_vec(),
        }
    }

    fn tick(&mut self) {
        // Create new shape as step 0 if it does not exist.
        if self.curr_shape.is_empty() {
            let (shape_size, shape) = SHAPES[self.next_shape_idx];
            self.next_shape_idx = (self.next_shape_idx + 1) % SHAPES.len();
            let mut shape_spawned = Vec::with_capacity(shape_size);

            let base_y = self.height as i32 + 4;
            for (x, y) in shape.iter().take(shape_size) {
                let new_x = x + 2;
                let new_y = base_y + y;
                shape_spawned.push((new_x, new_y));
            }
            self.curr_shape = shape_spawned;
            self.is_next_down = false;
            return;
        }

        // Calculate movement either down or left/right.
        let movement = if self.is_next_down {
            (0, -1)
        } else {
            let movement = self.movements[self.move_clock];
            self.move_clock = (self.move_clock + 1) % self.movements.len();
            (movement, 0)
        };

        // Adds movement and calculate if its possible, either if there is a wall or rock there.
        let can_move = self.curr_shape.iter().all(|(x, y)| {
            let new_x = x + movement.0;
            let new_y = y + movement.1;
            (0..CHAMBER_WIDTH).contains(&new_x)
                && new_y > 0
                && !self.occupied.contains(&(new_x, new_y))
        });

        if can_move {
            // If we can move mutate shape.
            for (x, y) in self.curr_shape.iter_mut() {
                x.add_assign(movement.0);
                y.add_assign(movement.1);
            }
        } else if self.is_next_down {
            // Drain current shape, and mark it occupied.
            for (x, y) in self.curr_shape.drain(..) {
                self.occupied.insert((x, y));
                self.height = self.height.max(y as usize);
            }
            self.stopped_rocks.add_assign(1);
            self.is_next_down = false;
        }
        self.is_next_down = !self.is_next_down;
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut chamber = Chamber::new(&input);
    while chamber.stopped_rocks < 2022 {
        chamber.tick();
    }

    Ok(chamber.height)
}

pub fn part_two(_input: &str) -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = input!("d17.txt")?;

    run!("part_one", part_one(&input)?, 3102);
    run!("part_two", part_two(&input)?);
    Ok(())
}
