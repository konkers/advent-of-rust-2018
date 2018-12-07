extern crate petgraph;
extern crate regex;

use petgraph::graph::{EdgeReference, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
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
            ).into())
        }
    };

    Ok(Instruction {
        name: caps.get(1).unwrap().as_str().to_string(),
        parent: caps.get(2).unwrap().as_str().to_string(),
    })
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

fn find_first_node(graph: &Graph<&str, ()>) -> Option<NodeIndex> {
    for ni in graph.node_indices() {
        let parents = graph.edges_directed(ni, petgraph::Incoming).count();
        if parents == 0 {
            return Some(ni);
        }
    }
    None
}

fn node_name<'a>(graph: &'a Graph<&str, ()>, node: NodeIndex) -> &'a str {
    *graph.node_weight(node).unwrap()
}

fn walk_graph(graph: &Graph<&str, ()>) -> String {
    let mut output = String::new();
    let first_node = find_first_node(&graph).unwrap();
    let mut available_nodes = BTreeMap::new();

    available_nodes.insert(node_name(&graph, first_node), first_node);

    while available_nodes.len() > 0 {
        // Borrow checker hates me!!!
        let (name, ni) = {
            let (a, b) = available_nodes.iter().next().unwrap();
            (a.clone(), b.clone())
        };
        available_nodes.remove(name);
        output += name;

        for edge in graph.edges_directed(ni, petgraph::Outgoing) {
            let child = edge.target();
           available_nodes.insert(node_name(&graph, child), child);
        }

    }

    output
}

fn read<R: Read>(io: R) -> Result<Vec<()>, Box<Error>> {
    let br = BufReader::new(io);
    let mut points = Vec::new();
    for line in br.lines() {}

    Ok(points)
}

fn main() -> Result<(), Box<Error>> {
    let input = read(File::open("input.txt")?)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_instructions() -> Vec<Instruction> {
        let mut i = vec![
            Instruction::new("A", "C"), // Step C must be finished before step A can begin.
            Instruction::new("F", "C"), // Step C must be finished before step F can begin.
            Instruction::new("B", "A"), // Step A must be finished before step B can begin.
            Instruction::new("D", "A"), // Step A must be finished before step D can begin.
            Instruction::new("E", "B"), // Step B must be finished before step E can begin.
            Instruction::new("E", "D"), // Step D must be finished before step E can begin.
            Instruction::new("E", "F"), // Step F must be finished before step E can begin.
        ];
        i.sort_by(|a, b| {
            (b.clone().name + &b.parent).partial_cmp(&(a.clone().name + &a.parent)).unwrap()
        });
        //println!("{:?}", i);
        i
    }

    #[test]
    fn parse_instruction_test() {
        assert_eq!(
            Instruction::new("C", "A"),
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
    fn find_first_node_test() {
        let instructions = get_instructions();
        let graph = build_graph(&instructions);
        let root = find_first_node(&graph).unwrap();
        let name = graph.node_weight(root).unwrap();

        assert_eq!(&"C", name);

        let mut bfs = petgraph::visit::Bfs::new(&graph, root);
        while let Some(nx) = bfs.next(&graph) {
            let wname = graph.node_weight(nx).unwrap();
            // we can access `graph` mutably here still
        }
    }

    #[test]
    fn walk_node_test() {
        let instructions = get_instructions();
        let graph = build_graph(&instructions);
        let root = find_first_node(&graph).unwrap();
        println!("{}", walk_graph(&graph));
    }
}
