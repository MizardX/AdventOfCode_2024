use std::collections::hash_map::Entry;
use std::collections::HashMap;
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
    println!("|'-Part 2: {} (expected ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys)", part_2(&input));
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
                if first_node.neighbors.contains(&third_ix) {
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
    if let Some(selection) =
        recursive_connected_subgraph(0, &mut vec![false; input.nodes.len()], input)
    {
        let mut names = Vec::new();
        for (ix, &include) in selection.iter().enumerate() {
            if include {
                names.push(input.nodes[ix].name);
            }
        }
        names.sort_unstable();
        return names.join(",");
    }
    String::new()
}

fn recursive_connected_subgraph(
    node_ix: usize,
    included_ixs: &mut [bool],
    input: &Input,
) -> Option<Vec<bool>> {
    // Base case
    if node_ix == included_ixs.len() {
        return Some(included_ixs.to_vec());
    }
    // If all included nodes are connected to this one, try including it
    let included = input
        .nodes
        .iter()
        .enumerate()
        .all(|(ix, node)| !included_ixs[ix] || node.neighbors.contains(&node_ix))
        .then(|| {
            included_ixs[node_ix] = true;
            let res = recursive_connected_subgraph(node_ix + 1, included_ixs, input);
            included_ixs[node_ix] = false;
            res
        })
        .flatten();
    // Also try excluding this node
    let excluded = recursive_connected_subgraph(node_ix + 1, included_ixs, input);
    // Pick the best result
    match (included, excluded) {
        (Some(included), Some(excluded)) => {
            let included_count = included.iter().filter(|&&b| b).count();
            let excluded_count = excluded.iter().filter(|&&b| b).count();
            if included_count >= excluded_count {
                Some(included)
            } else {
                Some(excluded)
            }
        }
        (Some(included), None) => Some(included),
        (None, Some(excluded)) => Some(excluded),
        (None, None) => None,
    }
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
        let mut index_lookup = HashMap::new();
        for line in text.lines() {
            let (left, right) = line
                .split_once('-')
                .ok_or(ParseInputError::MissingSeparator)?;
            let left_ix = match index_lookup.entry(left) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    let new_ix = nodes.len();
                    nodes.push(Node {
                        name: left,
                        neighbors: Vec::new(),
                    });
                    *entry.insert(new_ix)
                }
            };
            let right_ix = match index_lookup.entry(right) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    let new_ix = nodes.len();
                    nodes.push(Node {
                        name: right,
                        neighbors: Vec::new(),
                    });
                    *entry.insert(new_ix)
                }
            };
            nodes[left_ix].neighbors.push(right_ix);
            nodes[right_ix].neighbors.push(left_ix);
        }
        Ok(Self { nodes })
    }
}
