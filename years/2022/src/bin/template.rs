use lib::prelude::*;

pub fn part_one(input: &str) -> Result<usize> {
    Ok(0)
}

pub fn part_two(input: &str) -> Result<usize> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = input!("d14.txt")?;

    run!("part_one", part_one(&input)?);
    run!("part_two", part_two(&input)?);
    Ok(())
}
