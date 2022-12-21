use lib::prelude::*;

#[derive(Debug, Clone)]
pub struct Numb {
    value: i64,
    orginal_idx: usize,
}

fn parse(input: &str) -> Vec<Numb> {
    input
        .lines()
        .enumerate()
        .map(|(orginal_idx, val)| {
            let value = val.parse::<i64>().unwrap();
            Numb { value, orginal_idx }
        })
        .collect_vec()
}

fn decrypt(numbers: &Vec<Numb>, cycles: i64) -> i64 {
    let mut numbers = numbers.clone();
    let message_size = numbers.len() as i64 - 1;
    for _ in 0..cycles {
        for current in 0..numbers.len() {
            let index = numbers
                .iter()
                .position(|x| x.orginal_idx == current)
                .unwrap();
            let mut new_index = index as i64 + numbers[index].value;
            new_index = ((new_index % message_size) + message_size) % message_size;
            let number = numbers.remove(index);
            numbers.insert(new_index as usize, number);
        }
    }

    let zero_index = numbers.iter().position(|x| x.value == 0).unwrap();
    let n1 = numbers[(zero_index + 1000) % numbers.len()].value;
    let n2 = numbers[(zero_index + 2000) % numbers.len()].value;
    let n3 = numbers[(zero_index + 3000) % numbers.len()].value;
    return n1 + n2 + n3;
}

pub fn part_one(input: &Vec<Numb>) -> Result<i64> {
    let res = decrypt(input, 1);
    Ok(res)
}

pub fn part_two(input: &Vec<Numb>) -> Result<i64> {
    let input = input
        .iter()
        .map(|Numb { value, orginal_idx }| Numb {
            value: value * 811589153,
            orginal_idx: *orginal_idx,
        })
        .collect_vec();
    let res = decrypt(&input, 10);
    Ok(res)
}

fn main() -> Result<()> {
    let input = input!("d20.txt")?;
    let input = parse(&input);

    run!("part_one", part_one(&input)?, 13883);
    run!("part_two", part_two(&input)?, 19185967576920);
    Ok(())
}

// part_one: 13883
//  took 4.929599ms

// part_two: 19185967576920
//  took 52.276749ms
