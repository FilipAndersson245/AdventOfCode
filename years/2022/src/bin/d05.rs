use lib::indexmap::IndexMap;
use lib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Command {
    how_much: usize,
    from: usize,
    to: usize,
}

pub fn part_one((state, commands): &(IndexMap<usize, Vec<char>>, Vec<Command>)) -> Result<String> {
    let mut state = state.clone();
    for command in commands.iter() {
        for _ in 0..command.how_much {
            let c = state.get_mut(&command.from).unwrap().pop().unwrap();
            state.get_mut(&command.to).unwrap().push(c);
        }
    }
    let mut s = "".to_string();
    state.sort_keys();
    state.into_values().for_each(|v| s.push(*v.last().unwrap()));
    Ok(s)
}

pub fn part_two((state, commands): &(IndexMap<usize, Vec<char>>, Vec<Command>)) -> Result<String> {
    let mut state = state.clone();
    for command in commands.iter() {
        let len = state.get(&command.from).unwrap().len();
        let d_v = state
            .get_mut(&command.from)
            .unwrap()
            .drain((len - command.how_much)..len)
            .collect_vec();
        for ch in d_v.iter() {
            state.get_mut(&command.to).unwrap().push(*ch);
        }
    }
    let mut s = "".to_string();
    state.sort_keys();
    state.into_values().for_each(|v| s.push(*v.last().unwrap()));
    Ok(s)
}

fn parse(input: &str) -> (IndexMap<usize, Vec<char>>, Vec<Command>) {
    let mut input_iter = input.split("\n\n");
    let initial_state_str = input_iter.next().unwrap();
    let command_str = input_iter.next().unwrap();

    let mut state: IndexMap<usize, Vec<char>> = IndexMap::new();
    initial_state_str
        .lines()
        .map(|line| line.chars())
        .for_each(|chars| {
            chars
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, ch)| ch.is_ascii_uppercase())
                .for_each(|(idx, ch)| {
                    let idx = idx + 1;
                    if let Some(s) = state.get_mut(&idx) {
                        s.push(ch);
                    } else {
                        state.insert(idx, vec![ch]);
                    }
                });
        });
    state.iter_mut().for_each(|s| s.1.reverse());

    let commands = command_str
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace().skip(1).step_by(2);
            let how_much = iter.next().unwrap().parse::<usize>().unwrap();
            let from = iter.next().unwrap().parse::<usize>().unwrap();
            let to = iter.next().unwrap().parse::<usize>().unwrap();
            Command { how_much, from, to }
        })
        .collect_vec();
    (state, commands)
}

fn main() -> Result<()> {
    let input = input!("d05.txt")?;
    let parsed_input = parse(&input);

    run!("part_one", part_one(&parsed_input)?, "BSDMQFLSP");
    run!("part_two", part_two(&parsed_input)?, "PGSQBFLDP");
    bench!("parsing", parse(&input));
    Ok(())
}

// part_one: BSDMQFLSP
//  took 62.797µs

// part_two: PGSQBFLDP
//  took 67.579µs

// parsing:
//  took 26.094µs
