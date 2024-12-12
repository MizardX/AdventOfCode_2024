use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const EXAMPLE3: &str = include_str!("example3.txt");
const EXAMPLE4: &str = include_str!("example4.txt");
const EXAMPLE5: &str = include_str!("example5.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 12");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!("|+-Part 1: {} (expected 140)", part_1(&example1));
    println!("|'-Part 2: {} (expected 80)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 772)", part_1(&example2));
    println!("|'-Part 2: {} (expected 436)", part_2(&example2));

    println!("++Example 3");
    let example3 = EXAMPLE3.parse().expect("Parse example 3");
    println!("|+-Part 1: {} (expected 1_930)", part_1(&example3));
    println!("|'-Part 2: {} (expected 1_206)", part_2(&example3));

    println!("++Example 4");
    let example4 = EXAMPLE4.parse().expect("Parse example 4");
    println!("|+-Part 1: {} (expected 692)", part_1(&example4));
    println!("|+-Part 2: {} (expected 236)", part_2(&example4));

    println!("++Example 5");
    let example5 = EXAMPLE5.parse().expect("Parse example 5");
    println!("|+-Part 1: {} (expected 1_184)", part_1(&example5));
    println!("|+-Part 2: {} (expected 368)", part_2(&example5));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1_573_474)", part_1(&input));
    println!("|'-Part 2: {} (expected 966_476)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut ds = DisjointSet::new(input.plots.height() * input.plots.width());
    let stride = input.plots.width();
    let mut perimiter = vec![4; input.plots.height() * input.plots.width()];
    for (r, row) in input.plots.rows().enumerate() {
        for (c, &plot) in row.iter().enumerate() {
            let id = r * input.plots.width() + c;
            if c > 0 && row[c - 1] == plot {
                ds.union(id, id - 1);
                perimiter[id] -= 1;
                perimiter[id - 1] -= 1;
            }
            if r > 0 && input.plots.get(c, r - 1) == Some(&plot) {
                ds.union(id, id - stride);
                perimiter[id] -= 1;
                perimiter[id - stride] -= 1;
            }
        }
    }
    let mut total_cost = 0;
    for r in 0..input.plots.height() {
        for c in 0..input.plots.width() {
            let id = r * input.plots.width() + c;
            let root = ds.find(id);
            if let Some(size) = ds.size(root) {
                total_cost += size * perimiter[id];
            }
        }
    }
    total_cost
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let width = input.plots.width();
    let height = input.plots.height();
    let mut ds = DisjointSet::new(width * height);
    let mut perimiter = vec![4; width * height];
    let stride = width;
    for r in 0..=height {
        for c in 0..=width {
            let id = r * stride + c;
            let y = isize::try_from(r).expect("usize to isize conversion failed");
            let x = isize::try_from(c).expect("usize to isize conversion failed");
            let diag = input.plots.get_signed(x - 1, y - 1);
            let left = input.plots.get_signed(x - 1, y);
            let above = input.plots.get_signed(x, y - 1);
            let this = input.plots.get_signed(x, y);
            if left.is_some() && left == this {
                // same as left, so left fence does not exist, and right fence of left plot does not exist. this--, left--
                ds.union(id, id - 1);
                perimiter[id] -= 1;
                perimiter[id - 1] -= 1;
            }
            if above.is_some() && above == this {
                // same as above, so above fence does not exist, and below fence of above plot does not exist. this--, above--
                ds.union(id, id - stride);
                perimiter[id] -= 1;
                perimiter[id - stride] -= 1;
            }
            if this.is_some() && above == this && left != this && diag != this {
                // left fence countinues. this--
                // .X
                // .X
                perimiter[id] -= 1;
            }
            if left.is_some() && diag == left && this != left && above != diag {
                // right fence of left region countinues. left--
                // Y.
                // Y.
                perimiter[id - 1] -= 1;
            }
            if this.is_some() && left == this && above != this && diag != left {
                // above fence countinues. this--
                // ..
                // XX
                perimiter[id] -= 1;
            }
            if above.is_some() && diag == above && left != diag && this != above {
                // below fence of above region countinues. above--
                // YY
                // ..
                perimiter[id - stride] -= 1;
            }
        }
    }
    let mut total_cost = 0;
    for r in 0..input.plots.height() {
        for c in 0..input.plots.width() {
            let id = r * input.plots.width() + c;
            let root = ds.find(id);
            if let Some(size) = ds.size(root) {
                total_cost += size * perimiter[id];
            }
        }
    }
    total_cost
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

    fn size(&self, root: usize) -> Option<usize> {
        (root == self.parents[root]).then_some(self.sizes[root])
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Plot(u8);

impl TryFrom<u8> for Plot {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    plots: Grid<Plot>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let plots = text.parse()?;
        Ok(Self { plots })
    }
}
