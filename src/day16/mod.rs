use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
// const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 16");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 7_036)", part_1(&example1));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 11_048)", part_1(&example2));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example2));

    // println!("++Input");
    // let input = INPUT.parse().expect("Parse input");
    // println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let _ = input;
    todo!()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Tile {
    fn is_passable(&self) -> bool {
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

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Tile>,
    graph: Vec<Vec<Edge>>,
    start_ix: usize,
    end_ixs: [usize; 4],
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

    #[expect(clippy::too_many_lines)]
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Tile> = text.parse()?;
        let mut nodes = Vec::new();
        let mut node_lookup = HashMap::new();
        let mut edges: Vec<Vec<Edge>> = Vec::new();
        let mut start_ix = None;
        let mut end_ixs = None;
        let mut prev_in_row: Option<usize> = None;
        let mut prev_in_col = vec![None; grid.width()];
        for (y, row) in grid.rows().enumerate() {
            prev_in_row = None;
            for (x, &tile) in row.iter().enumerate() {
                let mut is_start = false;
                let mut is_end = false;
                let left = grid.get_signed(x as isize - 1, y as isize);
                let right = grid.get_signed(x as isize + 1, y as isize);
                let up = grid.get_signed(x as isize, y as isize - 1);
                let down = grid.get_signed(x as isize, y as isize + 1);
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
                        continue
                    }
                    _ => (),
                }

                const UP: usize = 0;
                const RIGHT: usize = 1;
                const DOWN: usize = 2;
                const LEFT: usize = 3;
                let base_ix = nodes.len();
                let up_ix = base_ix + UP;
                let right_ix = base_ix + RIGHT;
                let down_ix = base_ix + DOWN;
                let left_ix = base_ix + LEFT;
                if is_start {
                    start_ix = Some(right_ix);
                }
                if is_end {
                    end_ixs = Some([up_ix, right_ix, down_ix, left_ix]);
                }
                node_lookup.insert((x, y, UP), up_ix);
                node_lookup.insert((x, y, RIGHT), right_ix);
                node_lookup.insert((x, y, DOWN), down_ix);
                node_lookup.insert((x, y, LEFT), left_ix);
                nodes.extend_from_slice(&[(x, y, UP), (x, y, RIGHT), (x, y, DOWN), (x, y, LEFT)]);
                edges.extend_from_slice(&[Vec::new(), Vec::new(), Vec::new(), Vec::new()]);
                const TURN_WEIGHT: usize = 1_000;
                for a in [up_ix, right_ix, down_ix, left_ix] {
                    for b in [up_ix, right_ix, down_ix, left_ix] {
                        if a != b {
                            edges[a].push(Edge {
                                dest_ix: b,
                                weight: TURN_WEIGHT,
                            });
                        }
                    }
                }
                if let Some(from_left_ix) = prev_in_row {
                    if let Some(&left) = left {
                        if left.is_passable() {
                            let weight = x - nodes[from_left_ix].0;
                            edges[from_left_ix].push(Edge {
                                dest_ix: right_ix,
                                weight,
                            });
                            edges[left_ix].push(Edge {
                                dest_ix: from_left_ix - RIGHT + LEFT,
                                weight,
                            });
                        }
                    }
                }
                prev_in_row = Some(base_ix);
                if let Some(above_base_ix) = prev_in_col[x] {
                    if let Some(&above) = up {
                        if above.is_passable() {
                            let n: (usize, usize, usize) = nodes[above_base_ix + DOWN];
                            let weight = y - n.1;
                            Vec::push(&mut edges[above_base_ix + DOWN], Edge {
                                dest_ix: down_ix,
                                weight,
                            });
                            Vec::push(&mut edges[up_ix], Edge {
                                dest_ix: above_base_ix + UP,
                                weight,
                            });
                        }
                    }
                }
                prev_in_col[x] = Some(base_ix);
            }
        }
        let start_ix = start_ix.ok_or(ParseInputError::NoStart)?;
        let end_ixs = end_ixs.ok_or(ParseInputError::NoEnd)?;
        Ok(Self {
            grid,
            graph: edges,
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
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_lookup: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, x: usize, y: usize) -> usize {
        let base_ix = self.nodes.len();
        for dir in 0..4 {
            let node = Node { x, y, dir };
            self.node_lookup.insert(node, base_ix + dir);
            self.nodes.push(node);
            self.edges.push(Vec::new());
        }
        for a in base_ix..base_ix + 4 {
            for b in base_ix..base_ix + 4 {
                if a != b {
                    self.edges[a].push(Edge {
                        dest_ix: b,
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
        self.edges[from_ix].push(Edge { dest_ix: to_ix, weight });
    }
}