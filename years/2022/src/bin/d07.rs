use std::collections::HashMap;

use lib::prelude::*;
use relative_path::{RelativePath as Path, RelativePathBuf as PathBuf};

pub fn part_one(input: &str) -> Result<u64> {
    let (sizes, _) = solver(input);
    let res: u64 = sizes.iter().filter(|&&size| size < 100000).sum();
    Ok(res)
}

pub fn part_two(input: &str) -> Result<u64> {
    let (sizes, total) = solver(input);
    let rem = 70000000 - total;
    let needed = 30000000 - rem;
    let res = *sizes.iter().filter(|&&size| size >= needed).min().unwrap();

    Ok(res)
}

pub fn solver(input: &str) -> (Vec<u64>, u64) {
    let mut iter = input.lines().peekable();
    let _ = iter.next(); // Skip first cd as we start there already
    let mut cwd = PathBuf::new();
    let mut total = 0;

    let mut sizes = HashMap::<_, u64>::new();

    while let Some(line) = iter.next() {
        // Handle ls
        if line.starts_with("$ ls") {
            continue;
        }

        // Handle cd
        if line.starts_with("$ cd") {
            match line.split_ascii_whitespace().last() {
                Some(arg) => {
                    cwd = cwd.join_normalized(Path::new(arg));
                }
                _ => unreachable!(),
            }
            continue;
        }

        // Handle other
        let mut parts = line.split(' ');
        match parts.next() {
            Some("dir") => {}
            Some(s) => {
                let size = s.parse::<u64>().unwrap();
                total += size;

                let mut current = Some(cwd.clone());

                while let Some(d) = current.take() {
                    *sizes.entry(d.clone()).or_default() += s.parse::<u64>().unwrap();
                    current = d.parent().map(|p| p.to_owned());
                }
            }
            _ => unreachable!(),
        }
    }

    (sizes.values().cloned().collect_vec(), total)
}

fn main() -> Result<()> {
    let input = input!("d07.txt")?;
    run!("part_one", part_one(&input)?);
    run!("part_two", part_two(&input)?);
    Ok(())
}

// part_one: 1334506
//  took 530.078µs

// part_two: 7421137
//  took 518.961µs
