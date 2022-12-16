use lib::prelude::*;
use memoize::memoize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct State {
    location: String,
    current_time: usize,
    current_flow: usize,
    oppened: HashSet<String>,
}
#[derive(Clone, Debug)]
pub struct Grid {
    reachable: HashMap<String, Vec<String>>,
    flow_rate: HashMap<String, i32>,
}

fn foo(grid: Grid) -> i32 {
    0
}

pub fn part_one(grid: &Grid) -> Result<usize> {
    Ok(0)
}

// pub fn part_one(grid: &Grid) -> Result<usize> {
//     let possible_pressure: usize = grid
//         .flow_rate
//         .iter()
//         .map(|(_, pressure)| pressure)
//         .sum::<usize>()
//         * 30;
//     // let mut current_min = 30;
//     // let state = State {
//     //     location: "AA".to_string(),
//     //     current_flow: 0,
//     //     current_time: 0,
//     //     oppened: HashSet::new(),
//     // };

//     let a = pathfinding::directed::dijkstra::dijkstra_all(
//         &("AA".to_string(), 0, 0),
//         |(pos, round, current)| {
//             grid.reachable[pos]
//                 .iter()
//                 .map(|reach| {
//                     (
//                         (reach.to_owned(), round + 1, current + grid.flow_rate[reach]),
//                         possible_pressure - ((round + 1) * current) - grid.flow_rate[reach],
//                     )
//                 })
//                 .collect_vec()
//         },
//     );
//     println!("{a:?}");

//     Ok(0)
// }

// pub fn part_two(input: &Vec<(String, usize, Vec<String>)>) -> Result<usize> {
//     Ok(0)
// }

// fn parse(input: &str) -> Grid {
//     let mut flow_rate = HashMap::new();
//     let mut reachable = HashMap::new();
//     input.lines().for_each(|s| {
//         let (_, s) = s.split_once(' ').unwrap();
//         let (valve, s) = s.split_once(' ').unwrap();
//         let (_, s) = s.split_once('=').unwrap();
//         let (flow, s) = s.split_once(';').unwrap();
//         let tunnels = s.trim_start_matches(|s: char| s.is_lowercase() || s.is_whitespace());
//         println!("{tunnels}");
//         let valve = valve.to_string();
//         let flow = flow.parse::<i32>().unwrap();
//         let tunnels = tunnels.split(", ").map(|s| s.to_string()).collect_vec();

//         flow_rate.insert(valve.to_string(), flow);
//         reachable.insert(valve.to_string(), tunnels);

//         // data.insert(valve, (flow_rate, tunnels));
//         // (valve, flow_rate, tunnels);
//     });
//     Grid {
//         reachable,
//         flow_rate,
//     }
// }

fn parse(input: &str) -> Grid {
    let mut flow_rate = HashMap::new();
    let mut reachable = HashMap::new();
    input.lines().for_each(|s| {
        let (_, s) = s.split_once(' ').unwrap();
        let (valve, s) = s.split_once(' ').unwrap();
        let (_, s) = s.split_once('=').unwrap();
        let (flow, s) = s.split_once(';').unwrap();
        let tunnels = s.trim_start_matches(|s: char| s.is_lowercase() || s.is_whitespace());
        let flow = flow.parse::<i32>().unwrap();
        let tunnels = tunnels.split(", ").collect_vec();

        flow_rate.insert(valve, flow);
        reachable.insert(valve, tunnels);
    });
    let simple_reachable_with_steps = 0;
    println!("{reachable:?}");
    for (key, values) in &reachable {
        println!("{key} : {values:?}");
        values
            .iter()
            .map(|s| {
                let mut a = vec![];
                let mut b = reachable[s].iter().filter(|&a| flow_rate[a] == 0);
            })
            .collect_vec();
    }

    todo!()
    // Grid {
    //     reachable,
    //     flow_rate,
    // }
}

fn main() -> Result<()> {
    let input = input!("d16.txt")?;

    let parsed_input = parse(&input);

    run!("part_one", part_one(&parsed_input)?);
    // run!("part_two", part_two(&parsed_input)?);
    Ok(())
}
