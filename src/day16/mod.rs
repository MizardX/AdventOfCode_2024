use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 16");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 7_036)", part_1(&example1));
    println!("|'-Part 2: {} (expected 45)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 11_048)", part_1(&example2));
    println!("|'-Part 2: {} (expected 64)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 88_468)", part_1(&input));
    println!("|'-Part 2: {} (expected 616)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    input.find_shortest_path().expect("No path found")
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let all_nodes = input.find_all_paths();
    let mut visited_locations = HashSet::new();
    for &a in &all_nodes {
        for &b in &all_nodes {
            if a != b && input.graph.is_connected(a, b) {
                let node_a = input.graph.get_node(a);
                let node_b = input.graph.get_node(b);
                for y in node_a.y.min(node_b.y)..=node_a.y.max(node_b.y) {
                    for x in node_a.x.min(node_b.x)..=node_a.x.max(node_b.x) {
                        visited_locations.insert((x, y));
                    }
                }
            }
        }
    }
    visited_locations.len()
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Tile {
    fn is_passable(self) -> bool {
        match self {
            Self::Empty | Self::Start | Self::End => true,
            Self::Wall => false,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Empty),
            b'#' => Ok(Self::Wall),
            b'S' => Ok(Self::Start),
            b'E' => Ok(Self::End),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;

#[derive(Debug, Clone)]
pub struct Input {
    graph: Graph,
    start_ix: usize,
    end_ixs: [usize; 4],
}

impl Input {
    fn find_shortest_path(&self) -> Option<usize> {
        let mut visited_nodes = HashSet::new();
        let mut pending = BinaryHeap::new();
        pending.push(Reverse((0, self.start_ix)));
        while let Some(Reverse((dist, ix))) = pending.pop() {
            if !visited_nodes.insert(ix) {
                continue;
            }
            if self.end_ixs.contains(&ix) {
                return Some(dist);
            }
            for edge in &self.graph.edges[ix] {
                pending.push(Reverse((dist + edge.weight, edge.dest_ix)));
            }
        }
        None
    }

    fn find_all_paths(&self) -> Vec<usize> {
        let node_count = self.graph.nodes.len();
        let mut visited_nodes = vec![false; node_count];
        let mut dist_to_node = vec![usize::MAX; node_count];
        let mut pending = BinaryHeap::new();
        pending.push(Reverse((0, self.start_ix)));
        while let Some(Reverse((dist, ix))) = pending.pop() {
            if visited_nodes[ix] {
                continue;
            }
            visited_nodes[ix] = true;
            dist_to_node[ix] = dist;
            for edge in &self.graph.edges[ix] {
                pending.push(Reverse((dist + edge.weight, edge.dest_ix)));
            }
        }
        for visited in &mut visited_nodes {
            *visited = false;
        }
        let mut dist_from_node = vec![usize::MAX; node_count];
        pending.clear();
        pending.extend(self.end_ixs.iter().map(|&ix| Reverse((0, ix))));
        while let Some(Reverse((dist, ix))) = pending.pop() {
            if visited_nodes[ix] {
                continue;
            }
            visited_nodes[ix] = true;
            dist_from_node[ix] = dist;
            for edge in &self.graph.rev_edges[ix] {
                pending.push(Reverse((dist + edge.weight, edge.dest_ix)));
            }
        }
        let dist_start_to_goal = dist_from_node[self.start_ix];
        (0..node_count)
            .filter(|&ix| dist_to_node[ix] + dist_from_node[ix] == dist_start_to_goal)
            .collect()
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("No start node")]
    NoStart,
    #[error("No end node")]
    NoEnd,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Tile> = text.parse()?;
        let mut graph = Graph::new();
        let mut start_ix = None;
        let mut end_ixs = None;
        let mut prev_in_col = vec![None; grid.width()];
        for (y, row) in grid.rows().enumerate() {
            let mut prev_in_row = None;
            for (x, &tile) in row.iter().enumerate() {
                let mut is_start = false;
                let mut is_end = false;
                let left = x.checked_sub(1).and_then(|x1| grid.get(x1, y));
                let right = grid.get(x + 1, y);
                let up = y.checked_sub(1).and_then(|y1| grid.get(x, y1));
                let down = grid.get(x, y + 1);
                match tile {
                    Tile::Wall => continue,
                    Tile::Start => {
                        is_start = true;
                    }
                    Tile::End => {
                        is_end = true;
                    }
                    Tile::Empty => (),
                };
                match (left, right, up, down) {
                    (None, _, _, _)
                    | (_, None, _, _)
                    | (_, _, None, _)
                    | (_, _, _, None)
                    | (Some(Tile::Wall), Some(Tile::Wall), Some(Tile::Empty), Some(Tile::Empty))
                    | (Some(Tile::Empty), Some(Tile::Empty), Some(Tile::Wall), Some(Tile::Wall))
                        if !is_start && !is_end =>
                    {
                        continue; // Touching an edge, or a straight corridor
                    }
                    _ => (),
                }

                let base_ix = graph.add_node(x, y);
                if is_start {
                    start_ix = Some(base_ix + RIGHT);
                }
                if is_end {
                    end_ixs = Some([
                        base_ix + UP,
                        base_ix + RIGHT,
                        base_ix + DOWN,
                        base_ix + LEFT,
                    ]);
                }
                if let (Some(from_left_ix), Some(&left)) = (prev_in_row, left) {
                    if left.is_passable() {
                        graph.add_edge(from_left_ix + RIGHT, base_ix + RIGHT);
                        graph.add_edge(base_ix + LEFT, from_left_ix + LEFT);
                    }
                }
                prev_in_row = Some(base_ix);
                if let (Some(above_base_ix), Some(&above)) = (prev_in_col[x], up) {
                    if above.is_passable() {
                        graph.add_edge(above_base_ix + DOWN, base_ix + DOWN);
                        graph.add_edge(base_ix + UP, above_base_ix + UP);
                    }
                }
                prev_in_col[x] = Some(base_ix);
            }
        }
        let start_ix = start_ix.ok_or(ParseInputError::NoStart)?;
        let end_ixs = end_ixs.ok_or(ParseInputError::NoEnd)?;
        Ok(Self {
            graph,
            start_ix,
            end_ixs,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    dir: usize,
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    dest_ix: usize,
    weight: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<Node>,
    node_lookup: HashMap<Node, usize>,
    edges: Vec<Vec<Edge>>,
    rev_edges: Vec<Vec<Edge>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
            edges: Vec::new(),
            rev_edges: Vec::new(),
        }
    }

    fn add_node(&mut self, x: usize, y: usize) -> usize {
        let base_ix = self.nodes.len();
        for dir in [UP, RIGHT, DOWN, LEFT] {
            let node = Node { x, y, dir };
            self.node_lookup.insert(node, base_ix + dir);
            self.nodes.push(node);
            self.edges.push(Vec::new());
            self.rev_edges.push(Vec::new());
        }
        for a in base_ix..base_ix + 4 {
            for b in base_ix..base_ix + 4 {
                if a != b {
                    self.edges[a].push(Edge {
                        dest_ix: b,
                        weight: 1_000,
                    });
                    self.rev_edges[b].push(Edge {
                        dest_ix: a,
                        weight: 1_000,
                    });
                }
            }
        }
        base_ix
    }

    fn add_edge(&mut self, from_ix: usize, to_ix: usize) {
        let weight_y = self.nodes[to_ix].y.abs_diff(self.nodes[from_ix].y);
        let weight_x = self.nodes[to_ix].x.abs_diff(self.nodes[from_ix].x);
        let weight = weight_x + weight_y;
        self.edges[from_ix].push(Edge {
            dest_ix: to_ix,
            weight,
        });
        self.rev_edges[to_ix].push(Edge {
            dest_ix: from_ix,
            weight,
        });
    }

    fn is_connected(&self, from_ix: usize, to_ix: usize) -> bool {
        self.edges[from_ix].iter().any(|edge| edge.dest_ix == to_ix)
    }

    fn get_node(&self, ix: usize) -> &Node {
        &self.nodes[ix]
    }

    // fn heuristic(&self, from_ix: usize, to_ix: usize) -> usize {
    //     let from = self.nodes[from_ix];
    //     let to = self.nodes[to_ix];
    //     let dist_y = to.y.abs_diff(from.y);
    //     let dist_x = to.x.abs_diff(from.x);
    //     dist_x + dist_y
    // }
}
