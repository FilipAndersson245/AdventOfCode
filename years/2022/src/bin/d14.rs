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

fn init_grid(input: &str) -> (Vec<Vec<bool>>, usize) {
    let mut grid_map = vec![vec![true; 1000]; 1000];
    let mut largestst_y = 0;
    input.lines().for_each(|line| {
        let mut iter = line.split(" -> ").map(parse_point);
        let mut from = iter.next().unwrap();

        for to in iter {
            for x in min_to_max(from.0, to.0) {
                for y in min_to_max(from.1, to.1) {
                    if y > largestst_y {
                        largestst_y = y;
                    }
                    grid_map[x][y] = false
                }
            }
            from = to;
        }
    });
    (grid_map, largestst_y)
}

fn get_possible_sand_locations(
    sand_pos: (usize, usize),
) -> ((usize, usize), (usize, usize), (usize, usize)) {
    let sand_down = (sand_pos.0, sand_pos.1 + 1);
    let sand_down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
    let sand_down_right = (sand_pos.0 + 1, sand_pos.1 + 1);
    (sand_down, sand_down_left, sand_down_right)
}

pub fn part_one(input: &str) -> Result<usize> {
    let (mut grid_map, largestst_y) = init_grid(input);

    let sand_starting_pos = (500, 0);
    let mut sand = 0;
    'outer: loop {
        let mut sand_pos = sand_starting_pos;
        loop {
            if sand_pos.1 == largestst_y {
                break 'outer;
            }

            let (sand_down, sand_down_left, sand_down_right) =
                get_possible_sand_locations(sand_pos);

            if grid_map[sand_down.0][sand_down.1] {
                sand_pos = sand_down;
            } else if grid_map[sand_down_left.0][sand_down_left.1] {
                sand_pos = sand_down_left;
            } else if grid_map[sand_down_right.0][sand_down_right.1] {
                sand_pos = sand_down_right;
            } else {
                grid_map[sand_pos.0][sand_pos.1] = false;
                sand += 1;
                break;
            }
        }
    }
    Ok(sand)
}

pub fn part_two(input: &str) -> Result<usize> {
    let (mut grid_map, largestst_y) = init_grid(input);

    let lava = largestst_y + 2;
    for x in 0..1000 {
        grid_map[x][lava] = false
    }

    let sand_starting_pos = (500, 0);
    let mut sand = 0;

    'outer: loop {
        let mut sand_pos = sand_starting_pos;
        loop {
            let (sand_down, sand_down_left, sand_down_right) =
                get_possible_sand_locations(sand_pos);

            if grid_map[sand_down.0][sand_down.1] {
                sand_pos = sand_down;
            } else if grid_map[sand_down_left.0][sand_down_left.1] {
                sand_pos = sand_down_left;
            } else if grid_map[sand_down_right.0][sand_down_right.1] {
                sand_pos = sand_down_right;
            } else {
                grid_map[sand_pos.0][sand_pos.1] = false;
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
//  took 178.109µs

// part_two: 26170
//  took 2.60146ms
