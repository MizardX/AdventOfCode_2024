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

/* Recursive solution with backtracking
#[must_use]
pub fn part_2(input: &Input) -> String {
    let mut best_selection = Vec::new();
    let mut selected_nodes = Vec::new();

    for start_node in 0..input.nodes.len() {
        selected_nodes.clear();
        selected_nodes.push(start_node);

        let candidates = &input.nodes[start_node].neighbors;
        largest_connected_subgraph(candidates, &mut selected_nodes, &mut best_selection, input);
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
    largest_connected_subgraph(remaining_candidates, selected_nodes, best_selection, input);

    // If we can add the candidate node
    if selected_nodes.iter().all(|&selected_node| {
        input.nodes[selected_node]
            .neighbors
            .binary_search(&candidate_node)
            .is_ok()
    }) {
        // Recurse with the candidate node
        selected_nodes.push(candidate_node);
        largest_connected_subgraph(remaining_candidates, selected_nodes, best_selection, input);
        selected_nodes.pop();
    }
}
*/

#[must_use]
pub fn part_2(input: &Input) -> String {
    fn bron_kerbosch1(
        clique: &mut Vec<usize>,
        mut candidates: &[usize],
        mut excluded: HashSet<usize>,
        max_clique: &mut HashSet<usize>,
        input: &Input,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            if clique.len() > max_clique.len() {
                max_clique.clear();
                max_clique.extend(clique.iter().copied());
            }
            return;
        }

        for &v in candidates {
            let new_candidates = candidates
                .iter()
                .filter(|&&i| input.nodes[v].neighbors.contains(&i))
                .copied()
                .collect::<Vec<_>>();
            let new_excluded = excluded
                .iter()
                .filter(|&&i| input.nodes[v].neighbors.contains(&i))
                .copied()
                .collect();
            clique.push(v);
            bron_kerbosch1(clique, &new_candidates, new_excluded, max_clique, input);
            clique.pop();
            candidates = &candidates[1..];
            excluded.insert(v);
        }
    }
    let mut largest_clique = HashSet::new();
    let candidates = (0..input.nodes.len()).collect::<Vec<_>>();
    let excluded = HashSet::new();
    bron_kerbosch1(
        &mut Vec::new(),
        &candidates,
        excluded,
        &mut largest_clique,
        input,
    );
    let mut sorted_clique = largest_clique
        .into_iter()
        .map(|i| input.nodes[i].name)
        .collect::<Vec<_>>();
    sorted_clique.sort_unstable();
    sorted_clique.join(",")
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
            let left_node = *index_lookup.entry(left).or_insert_with(|| {
                let new_ix = nodes.len();
                nodes.push(Node {
                    name: left,
                    neighbors: Vec::new(),
                });
                new_ix
            });
            let right_node = *index_lookup.entry(right).or_insert_with(|| {
                let new_ix = nodes.len();
                nodes.push(Node {
                    name: right,
                    neighbors: Vec::new(),
                });
                new_ix
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
