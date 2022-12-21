// use lib::prelude::*;
// use petgraph::{
//     adj::NodeIndex, algo::floyd_warshall, dot::Dot, graph::UnGraph, prelude::UnGraphMap,
//     stable_graph, visit::GetAdjacencyMatrix,
// };
// use std::collections::HashMap;

// // #[derive(Debug, Clone)]
// // struct State {
// //     location: String,
// //     current_time: usize,
// //     current_flow: usize,
// //     oppened: HashSet<String>,
// // }
// #[derive(Clone, Debug)]
// pub struct Grid {
//     reachable: HashMap<String, Vec<String>>,
//     flow_rate: HashMap<String, i32>,
// }

// // fn foo(grid: Grid) -> i32 {
// //     0
// // }

// // pub fn part_one(grid: &Grid) -> Result<usize> {
// //     Ok(0)
// // }

// fn parse(input: &str) -> stable_graph::StableGraph<(&str, i32), i32, petgraph::Undirected> {
//     let mut flow_rate = HashMap::new();
//     let mut reachable = HashMap::new();
//     input.lines().for_each(|s| {
//         let (_, s) = s.split_once(' ').unwrap();
//         let (valve, s) = s.split_once(' ').unwrap();
//         let (_, s) = s.split_once('=').unwrap();
//         let (flow, s) = s.split_once(';').unwrap();
//         let tunnels = s.trim_start_matches(|s: char| s.is_lowercase() || s.is_whitespace());
//         let flow = flow.parse::<i32>().unwrap();
//         let tunnels = tunnels.split(", ").collect_vec();

//         flow_rate.insert(valve, flow);
//         reachable.insert(valve, tunnels);
//     });
//     // ---------------------------------------------------------------------------------------------

//     let mut graph = UnGraph::new_undirected();

//     let mut nodes = HashMap::new();
//     reachable.keys().for_each(|&k| {
//         nodes.insert(k, graph.add_node((k, flow_rate[k])));
//     });

//     for (a, b) in reachable {
//         let a = nodes[a];
//         b.iter().for_each(|&b| {
//             let b = nodes[b];
//             graph.add_edge(a, b, 1);
//         })
//     }

//     let shortest_pairs = floyd_warshall(&graph, |e| *e.weight()).unwrap();

//     println!("{shortest_pairs:#?}");

//     graph.clear_edges();
//     let mut graph = stable_graph::StableUnGraph::from(graph);
//     let starting_index = nodes["AA"];

//     for ((from, to), steps) in shortest_pairs {
//         if !graph.contains_node(from) || !graph.contains_node(to) {
//             continue;
//         }
//         let (from_name, from_flow) = graph[from];
//         let (to_name, to_flow) = graph[to];
//         if !graph.contains_edge(from, to) && steps > 0 {
//             graph.add_edge(from, to, steps);
//         }
//         // if graph.
//         if from_flow == 0 && from_name != "AA" {
//             graph.remove_node(from);
//         }
//         if to_flow == 0 && to_name != "AA" {
//             graph.remove_node(to);
//         }
//     }

//     println!("{:?}", Dot::new(&graph));
//     let curr = starting_index;
//     loop {}

//     todo!();
//     graph
// }

// fn main() -> Result<()> {
//     let input = input!("d16.txt")?;

//     let parsed_input = parse(&input);

//     // run!("part_one", part_one(&parsed_input)?);
//     // run!("part_two", part_two(&parsed_input)?);
//     Ok(())
// }
fn main() {}
