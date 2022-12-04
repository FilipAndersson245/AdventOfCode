use lib::prelude::*;
type Ranges = (u64, u64, u64, u64);

pub fn part_one(input: &[Ranges]) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|(p1l, p1u, p2l, p2u)| (p1l >= p2l && p1u <= p2u) || (p2l >= p1l && p2u <= p1u))
        .count())
}

pub fn part_two(input: &[Ranges]) -> Result<usize> {
    Ok(input
        .iter()
        .filter(|(p1l, p1u, p2l, p2u)| p1l <= p2u && p1u >= p2l)
        .count())
}

pub fn parse(input: &str) -> Vec<Ranges> {
    input
        .lines()
        .flat_map(|line| line.split(&[',', '-']))
        .map(|digit| digit.parse::<u64>().unwrap())
        .tuple_windows()
        .step_by(4)
        .collect_vec()
}

fn main() -> Result<()> {
    let input = input!("d04.txt")?;
    let parsed_input = parse(&input);

    run!("part_one", part_one(&parsed_input)?, 475);
    run!("part_two", part_two(&parsed_input)?, 825);
    bench!("parsing", parse(&input));
    Ok(())
}

// part_one: 475
//  took 907ns

// part_two: 825
//  took 801ns

// parsing
//  took 65.553µs
