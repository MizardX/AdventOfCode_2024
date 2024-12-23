use std::collections::{HashMap, HashSet};
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 23");

    println!("++Example");
    let example = EXAMPLE.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 7)", part_1(&example));
    println!("|'-Part 2: {} (expected co,de,ka,ta)", part_2(&example));

    println!("++Input");
    let input = INPUT.try_into().expect("Parse input");
    println!("|+-Part 1: {} (expected 1151)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    // Look for K3-connected triples i,j,k where i < j < k and at least
    // one of the nodes has a name that starts with 't'
    let mut count_triples = 0;
    for (first_ix, first_node) in input.nodes.iter().enumerate() {
        let first_t = first_node.name.starts_with('t');
        for &second_ix in &first_node.neighbors {
            if second_ix <= first_ix {
                continue;
            }
            let second_node = &input.nodes[second_ix];
            let second_t = second_node.name.starts_with('t');
            for &third_ix in &input.nodes[second_ix].neighbors {
                if third_ix <= second_ix {
                    continue;
                }
                if first_node.neighbors.binary_search(&third_ix).is_ok() {
                    let third_node = &input.nodes[third_ix];
                    let third_t = third_node.name.starts_with('t');
                    if first_t || second_t || third_t {
                        count_triples += 1;
                    }
                }
            }
        }
    }
    count_triples
}

#[must_use]
pub fn part_2(input: &Input) -> String {
    let mut best_selection = Vec::new();
    let mut selected_nodes = Vec::new();
    let mut exclude_nodes = HashSet::new();

    for start_node in 0..input.nodes.len() {
        selected_nodes.clear();
        selected_nodes.push(start_node);

        let candidates = &input.nodes[start_node].neighbors;
        largest_connected_subgraph(
            candidates,
            &mut selected_nodes,
            &exclude_nodes,
            &mut best_selection,
            input,
        );
        exclude_nodes.insert(start_node);
    }

    let mut selected_names = best_selection
        .iter()
        .map(|&node| input.nodes[node].name)
        .collect::<Vec<_>>();
    selected_names.sort_unstable();

    let mut res = String::with_capacity(3 * selected_names.len());
    if !selected_names.is_empty() {
        res.push_str(selected_names[0]);
        for &name in &selected_names[1..] {
            res.push(',');
            res.push_str(name);
        }
    }
    res
}

fn largest_connected_subgraph(
    candidates: &[usize],
    selected_nodes: &mut Vec<usize>,
    exclude_nodes: &HashSet<usize>,
    best_selection: &mut Vec<usize>,
    input: &Input,
) {
    if candidates.is_empty() {
        // Base case: no more candidates to consider
        if selected_nodes.len() > best_selection.len() {
            best_selection.clear();
            best_selection.extend_from_slice(selected_nodes);
        }
        return;
    }

    // Optimization: if we can't possibly beat the current best selection, return early
    if selected_nodes.len() + candidates.len() < best_selection.len() {
        return;
    }

    let candidate_node = candidates[0];
    let remaining_candidates = &candidates[1..];

    // Recurse without the candidate node
    largest_connected_subgraph(
        remaining_candidates,
        selected_nodes,
        exclude_nodes,
        best_selection,
        input,
    );

    // Optimization: If the candidate node has been processed before, skip it
    if exclude_nodes.contains(&candidate_node) {
        return;
    }

    // If the candidate node is not connected to all selected nodes, skip it
    if !selected_nodes.iter().all(|&selected_node| {
        input.nodes[selected_node]
            .neighbors
            .binary_search(&candidate_node)
            .is_ok()
    }) {
        return;
    }

    // Recurse with the candidate node
    selected_nodes.push(candidate_node);
    largest_connected_subgraph(
        remaining_candidates,
        selected_nodes,
        exclude_nodes,
        best_selection,
        input,
    );
    selected_nodes.pop();
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    neighbors: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Input<'a> {
    nodes: Vec<Node<'a>>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
    #[error("Missing separator")]
    MissingSeparator,
}

impl<'a> TryFrom<&'a str> for Input<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let mut nodes = Vec::new();
        let mut node_lookup = HashMap::new();
        for line in text.lines() {
            let (left_name, right_name) = line
                .split_once('-')
                .ok_or(ParseInputError::MissingSeparator)?;
            let left_node = *node_lookup.entry(left_name).or_insert_with(|| {
                let new_node = nodes.len();
                nodes.push(Node {
                    name: left_name,
                    neighbors: Vec::new(),
                });
                new_node
            });
            let right_node = *node_lookup.entry(right_name).or_insert_with(|| {
                let new_node = nodes.len();
                nodes.push(Node {
                    name: right_name,
                    neighbors: Vec::new(),
                });
                new_node
            });
            nodes[left_node].neighbors.push(right_node);
            nodes[right_node].neighbors.push(left_node);
        }
        for node in &mut nodes {
            node.neighbors.sort_unstable();
        }
        Ok(Self { nodes })
    }
}
