extern crate petgraph;
extern crate regex;

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Read};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Instruction {
    name: String,
    parent: String,
}

impl Instruction {
    fn new(name: &str, parent: &str) -> Instruction {
        Instruction { name: name.to_string(), parent: parent.to_string() }
    }
}

fn parse_instruction(s: &str) -> Result<Instruction, Box<Error>> {
    let re = Regex::new(r"Step (.+) must be finished before step (.+) can begin.$").unwrap();

    let caps = match re.captures(s) {
        Some(c) => c,
        None => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                format!("Unrecognized record \"{}\"", s),
            )
            .into())
        }
    };

    Ok(Instruction::new(caps.get(2).unwrap().as_str(), caps.get(1).unwrap().as_str()))
}

fn build_graph(instructions: &Vec<Instruction>) -> Graph<&str, ()> {
    let mut node_map = HashMap::new();

    let mut graph = Graph::<&str, ()>::new();
    for i in instructions {
        if !node_map.contains_key(&i.name) {
            node_map.insert(&i.name, graph.add_node(i.name.as_str()));
        }
        if !node_map.contains_key(&i.parent) {
            node_map.insert(&i.parent, graph.add_node(i.parent.as_str()));
        }
    }

    for i in instructions {
        let parent = node_map.get(&i.parent).unwrap();
        let child = node_map.get(&i.name).unwrap();

        graph.add_edge(*parent, *child, ());
    }

    graph
}

fn node_name<'a>(graph: &'a Graph<&str, ()>, node: NodeIndex) -> &'a str {
    *graph.node_weight(node).unwrap()
}

fn walk_graph(graph: &Graph<&str, ()>) -> String {
    let mut output = String::new();
    let mut available_nodes = BTreeMap::new();
    let mut visited_nodes = HashSet::new();

    for ni in graph.node_indices() {
        let num_deps = graph
            .neighbors_directed(ni, petgraph::Incoming)
            .map(|i| *graph.node_weight(i).unwrap())
            .count();
        if num_deps == 0 {
            available_nodes.insert(node_name(&graph, ni), ni);
        }
    }

    while available_nodes.len() > 0 {
        // Borrow checker hates me!!!
        let (name, ni) = {
            let (a, b) = available_nodes.iter().next().unwrap();
            (a.clone(), b.clone())
        };
        available_nodes.remove(name);
        visited_nodes.insert(name);
        output += name;

        for edge in graph.edges_directed(ni, petgraph::Outgoing) {
            let child = edge.target();
            let mut avail = true;

            // Well this escalated quickly!

            for p_edge in graph.edges_directed(child, petgraph::Incoming) {
                let p_name = p_edge.source();
                if !visited_nodes.contains(node_name(&graph, p_name)) {
                    avail = false;
                    break;
                }
            }

            if avail {
                available_nodes.insert(node_name(&graph, child), child);
            }
        }
    }

    output
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Worker {
    Idle,
    Working { node: NodeIndex, time: i64 },
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Worker) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Worker) -> Ordering {
        use Worker::*;
        match (self, other) {
            (Idle, Idle) => Ordering::Less,
            (Idle, Working { node: _, time: _ }) => Ordering::Less,
            (Working { node: _, time: _ }, Idle) => Ordering::Greater,
            (Working { node: _, time: a }, Working { node: _, time: b }) => a.cmp(&b),
        }
    }
}

fn is_worker_idle(worker: &Worker) -> bool {
    match worker {
        Worker::Idle => true,
        Worker::Working { node: _, time: _ } => false,
    }
}

fn workers_idle(workers: &Vec<Worker>) -> bool {
    for worker in workers {
        if is_worker_idle(&worker) {
            return true;
        }
    }
    false
}

fn workers_working(workers: &Vec<Worker>) -> bool {
    for worker in workers {
        if !is_worker_idle(&worker) {
            return true;
        }
    }
    false
}

fn calc_cost(name: &str, fixed_cost: i64) -> i64 {
    let mut b = name.bytes().next().unwrap();
    b -= b'A';
    b as i64 + 1 + fixed_cost
}

fn walk_graph2(graph: &Graph<&str, ()>, num_workers: usize, fixed_cost: i64) -> (i64, String) {
    let mut output = String::new();
    let mut available_nodes = BTreeMap::new();
    let mut visited_nodes = HashSet::new();
    let mut workers = vec![Worker::Idle; num_workers];
    let mut global_time = 0;

    for ni in graph.node_indices() {
        let num_deps = graph
            .neighbors_directed(ni, petgraph::Incoming)
            .map(|i| *graph.node_weight(i).unwrap())
            .count();
        if num_deps == 0 {
            available_nodes.insert(node_name(&graph, ni), ni);
        }
    }

    while available_nodes.len() > 0 || workers_working(&workers) {
        while available_nodes.len() > 0 && workers_idle(&workers) {
            let worker = workers.iter_mut().min().unwrap();
            if let Worker::Idle = worker {
                let (name, ni) = {
                    let (a, b) = available_nodes.iter().next().unwrap();
                    (a.clone(), b.clone())
                };
                println!("{}: available nodes: {:?}", global_time, available_nodes.keys());
                println!("{}: assigning {} to worker.", global_time, name);
                *worker = Worker::Working { node: ni, time: calc_cost(name, fixed_cost) };
                available_nodes.remove(name); // Worker owns the node now.
            }
        }

        println!("{}: workers: {:?}.", global_time, workers);
        let advance_time = {
            let worker = workers.iter().filter(|w| !is_worker_idle(&w)).min();
            match worker {
                Some(Worker::Working { node: _, time: time_remaining }) => *time_remaining,
                _ => 0,
            }
        };

        println!(
            "{}: advance time {}, idle {}, working {}",
            global_time,
            advance_time,
            workers_idle(&workers),
            workers_working(&workers)
        );
        if advance_time == 0 {
            continue;
        }

        global_time += advance_time;

        for w in workers.iter_mut() {
            let idle = {
                if let Worker::Working { node: ref ni, time: ref mut t } = w {
                    *t -= advance_time;
                    if *t < 0 {
                        panic!("negative time!");
                    }
                    if *t == 0 {
                        let name = node_name(&graph, *ni);
                        output += name;
                        println!("{}: {} done. {}", global_time, name, output);
                        visited_nodes.insert(name.clone());
                        for edge in graph.edges_directed(*ni, petgraph::Outgoing) {
                            let child = edge.target();
                            let mut avail = true;

                            // Well this escalated quickly!

                            for p_edge in graph.edges_directed(child, petgraph::Incoming) {
                                let p_name = p_edge.source();
                                if !visited_nodes.contains(node_name(&graph, p_name)) {
                                    avail = false;
                                    break;
                                }
                            }

                            if avail {
                                available_nodes.insert(node_name(&graph, child), child);
                            }
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if idle {
                *w = Worker::Idle;
            }
        }
    }

    (global_time, output)
}

fn read<R: Read>(io: R) -> Result<Vec<Instruction>, Box<Error>> {
    let br = BufReader::new(io);
    let mut insts = Vec::new();
    for line in br.lines() {
        insts.push(parse_instruction(&line?)?);
    }

    Ok(insts)
}

fn main() -> Result<(), Box<Error>> {
    let input = read(File::open("input.txt")?)?;
    let graph = build_graph(&input);
    for ni in graph.node_indices() {
        let n = graph.node_weight(ni).unwrap();
        let children: Vec<&str> = graph
            .neighbors_directed(ni, petgraph::Outgoing)
            .map(|i| *graph.node_weight(i).unwrap())
            .collect();
        let parents: Vec<&str> = graph
            .neighbors_directed(ni, petgraph::Incoming)
            .map(|i| *graph.node_weight(i).unwrap())
            .collect();
        println!("{:?}: {:?} {:?}", n, parents, children);
    }

    let part2 = walk_graph2(&graph, 4, 60);
    println!("{:?}", graph.node_count());
    println!("Pt 1 answer: {:?}", walk_graph(&graph));
    println!("Pt 2 answer: {:?}", part2.0);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_instructions() -> Vec<Instruction> {
        vec![
            Instruction::new("A", "C"), // Step C must be finished before step A can begin.
            Instruction::new("F", "C"), // Step C must be finished before step F can begin.
            Instruction::new("B", "A"), // Step A must be finished before step B can begin.
            Instruction::new("D", "A"), // Step A must be finished before step D can begin.
            Instruction::new("E", "B"), // Step B must be finished before step E can begin.
            Instruction::new("E", "D"), // Step D must be finished before step E can begin.
            Instruction::new("E", "F"), // Step F must be finished before step E can begin.
        ]
    }

    #[test]
    fn parse_instruction_test() {
        assert_eq!(
            Instruction::new("A", "C"),
            parse_instruction("Step C must be finished before step A can begin.").unwrap()
        );
    }

    #[test]
    fn build_graph_test() {
        let instructions = get_instructions();
        let graph = build_graph(&instructions);

        for ni in graph.node_indices() {
            let n = graph.node_weight(ni).unwrap();
            let neighbors: Vec<&str> =
                graph.neighbors(ni).map(|i| *graph.node_weight(i).unwrap()).collect();

            // println!("{:?}: {:?}", n, neighbors);
            match n {
                &"A" => assert_eq!(2, neighbors.len()),
                &"B" => assert_eq!(1, neighbors.len()),
                &"C" => assert_eq!(2, neighbors.len()),
                &"D" => assert_eq!(1, neighbors.len()),
                &"E" => assert_eq!(0, neighbors.len()),
                &"F" => assert_eq!(1, neighbors.len()),
                _ => panic!("unexpected node"),
            }
        }
    }

    #[test]
    fn walk_node_test() {
        let instructions = get_instructions();
        let graph = build_graph(&instructions);
        assert_eq!("CABDFE", walk_graph(&graph));
    }

    #[test]
    fn calc_cost_test() {
        assert_eq!(61, calc_cost("A", 60));
        assert_eq!(86, calc_cost("Z", 60));
    }

    #[test]
    fn walk_node2_test() {
        let instructions = get_instructions();
        let graph = build_graph(&instructions);
        assert_eq!((15, "CABFDE".to_string()), walk_graph2(&graph, 2, 0));
    }
}
