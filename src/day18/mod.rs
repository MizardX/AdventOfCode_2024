use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 18");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 22)", part_1(&example, 7, 12));
    println!("|'-Part 2: {} (expected 6,1)", part_2(&example, 7));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 318)", part_1(&input, 71, 1024));
    println!("|'-Part 2: {} (expected 56,29)", part_2(&input, 71));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input, size: u8, first: usize) -> usize {
    let blocked: HashSet<_> = input.byte_locations[..first].iter().copied().collect();
    djikstra(&blocked, size).unwrap_or(0)
}

#[must_use]
fn djikstra(blocked: &HashSet<(u8, u8)>, size: u8) -> Option<usize> {
    let start = (0_u8, 0_u8);
    let goal = (size - 1, size - 1);
    let width = size;
    let height = size;
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start)));
    while let Some(Reverse((dist, pos))) = queue.pop() {
        if pos == goal {
            return Some(dist);
        }
        if !visited.insert(pos) {
            continue;
        }
        for neighbor in neighbors(pos, blocked, width, height) {
            queue.push(Reverse((dist + 1, neighbor)));
        }
    }
    None
}

fn neighbors(
    (x, y): (u8, u8),
    blocked: &HashSet<(u8, u8)>,
    width: u8,
    height: u8,
) -> impl Iterator<Item = (u8, u8)> + '_ {
    [
        (x.saturating_sub(1), y, x > 0),
        (x + 1, y, x + 1 < width),
        (x, y.saturating_sub(1), y > 0),
        (x, y + 1, y + 1 < height),
    ]
    .into_iter()
    .filter_map(move |(x, y, valid)| {
        if valid && !blocked.contains(&(x, y)) {
            Some((x, y))
        } else {
            None
        }
    })
}

#[must_use]
pub fn part_2(input: &Input, size: u8) -> String {
    let stride = size as usize;
    let mut ds = DisjointSet::new(stride * stride);
    let mut blocked: HashSet<_> = input.byte_locations.iter().copied().collect();
    for r in 0..size {
        for c in 0..size {
            if blocked.contains(&(r, c)) {
                continue;
            }
            let ix = r as usize * stride + c as usize;
            if r > 0 && !blocked.contains(&(r - 1, c)) {
                ds.union(ix, ix - stride);
            }
            if c > 0 && !blocked.contains(&(r, c - 1)) {
                ds.union(ix, ix - 1);
            }
        }
    }
    let start_ix = 0;
    let end_ix = stride * stride - 1;
    if ds.find(start_ix) == ds.find(end_ix) {
        return "always reachable".to_string();
    }
    for &(r, c) in input.byte_locations.iter().rev() {
        let ix = r as usize * stride + c as usize;
        if r > 0 && !blocked.contains(&(r - 1, c)) {
            ds.union(ix, ix - stride);
        }
        if c > 0 && !blocked.contains(&(r, c - 1)) {
            ds.union(ix, ix - 1);
        }
        if r + 1 < size && !blocked.contains(&(r + 1, c)) {
            ds.union(ix, ix + stride);
        }
        if c + 1 < size && !blocked.contains(&(r, c + 1)) {
            ds.union(ix, ix + 1);
        }
        blocked.remove(&(r, c));
        if ds.find(start_ix) == ds.find(end_ix) {
            return format!("{r},{c}");
        }
    }
    "unreachable".to_string()
}

#[derive(Debug, Clone)]
struct DisjointSet {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    fn find(&mut self, mut node: usize) -> usize {
        let mut parent = self.parents[node];
        let mut grandparent = self.parents[parent];
        while parent != grandparent {
            self.parents[node] = grandparent;
            node = parent;
            parent = self.parents[node];
            grandparent = self.parents[parent];
        }
        parent
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return;
        }
        if self.sizes[a] < self.sizes[b] {
            (a, b) = (b, a);
        }
        self.parents[b] = a;
        self.sizes[a] += self.sizes[b];
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    byte_locations: Vec<(u8, u8)>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let byte_locations = text
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split_once(',')
                    .ok_or(ParseInputError::MissingDelimiter)?;
                Ok((x.parse()?, y.parse()?))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Self { byte_locations })
    }
}
