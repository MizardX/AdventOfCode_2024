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
    solve::<false>(input)
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    solve::<true>(input)
}

#[must_use]
pub fn solve<const PART2: bool>(input: &Input) -> usize {
    let width = input.plots.width();
    let height = input.plots.height();
    let mut ds = DisjointSet::new(width * height);
    let mut perimiter = vec![4; width * height];
    let stride = width;

    // Top edge -- corners are ignored, since they do not affect anything
    let first_row = input.plots.row(0).unwrap();
    for (c, (left, this)) in first_row.iter().zip(first_row.iter().skip(1)).enumerate() {
        let id = c + 1; // + 0 * stride;
        if left == this {
            // This and left plot are of the same region. So this plots's left fence and left plot's right fence should
            // be removed. this--, left--
            // ..
            // XX
            ds.union(id, id - 1);
            perimiter[id] -= 1;
            perimiter[id - 1] -= 1;

            if PART2 {
                // This plot's above fence countinues, so it does not contribute to the edge count. this--
                // ..
                // XX
                perimiter[id] -= 1;
            }
        }
    }

    // Middle rows will be iterated in pairs, so that we can compare adjacent plots
    for (r, (row1, row2)) in input
        .plots
        .rows()
        .zip(input.plots.rows().skip(1))
        .enumerate()
    {
        // Middle rows, left edge
        let c = 0;
        let id = (r + 1) * stride + c;
        let above = row1[0];
        let this = row2[0];
        if above == this {
            // This and above plot are of the same region. So this plots's above fence and above plot's below fence should
            // be removed. this--, above--
            // .X
            // .X
            ds.union(id, id - stride);
            perimiter[id] -= 1;
            perimiter[id - stride] -= 1;

            if PART2 {
                // This plot's left fence countinues, so it does not contribute to the edge count. this--
                // .X
                // .X
                perimiter[id] -= 1;
            }
        }

        // True middle. We will be comparing 2x2 plots at the time.
        for (c, ((diag, above), (left, this))) in row1
            .iter()
            .zip(row1.iter().skip(1))
            .zip(row2.iter().zip(row2.iter().skip(1)))
            .enumerate()
        {
            let id = (r + 1) * stride + c + 1;
            if left == this {
                // This and left plot are of the same region. So this plots's left fence and left plot's right fence should
                // be removed. this--, left--
                // ..
                // XX
                ds.union(id, id - 1);
                perimiter[id] -= 1;
                perimiter[id - 1] -= 1;

                if PART2 && above != this && diag != left {
                    // This plot's above fence countinues, so it does not contribute to the edge count. this--
                    // ..
                    // XX
                    perimiter[id] -= 1;
                }
            } else if PART2 && diag == left && above != diag {
                // Left plot's right fence countinues, so it does not contribute to the edge count. left--
                // Y.
                // Y.
                perimiter[id - 1] -= 1;
            }
            if above == this {
                // This and above plot are of the same region. So this plots's above fence and above plot's below fence should
                // be removed. this--, above--
                // .X
                // .X
                ds.union(id, id - stride);
                perimiter[id] -= 1;
                perimiter[id - stride] -= 1;

                if PART2 && left != this && diag != this {
                    // This plot's left fence countinues, so it does not contribute to the edge count. this--
                    // .X
                    // .X
                    perimiter[id] -= 1;
                }
            } else if PART2 && diag == above && left != diag {
                // Above plot's below fence countinues, so it does not contribute to the edge count. above--
                // YY
                // ..
                perimiter[id - stride] -= 1;
            }
        }
        
        if PART2 {
            // Middle rows, right edge
            let id = (r + 1) * stride + width;
            let diag = row1[width - 1];
            let left = row2[width - 1];
            if diag == left {
                // Left plot's right fence countinues, so it does not contribute to the edge count. left--
                // Y.
                // Y.
                perimiter[id - 1] -= 1;
            }
        }
    }

    if PART2 {
        // Bottom edge -- corners are ignored, since they do not affect anything
        let last_row = input.plots.row(height - 1).unwrap();
        let r = height;
        for (c, (diag, above)) in last_row.iter().zip(last_row.iter().skip(1)).enumerate() {
            let id = r * stride + c + 1;
            if diag == above {
                // Above plot's below fence countinues, so it does not contribute to the edge count. above--
                // YY
                // ..
                perimiter[id - stride] -= 1;
            }
        }
    }

    // Sum up remaining perimiters multiplied by the size of the surrounding region
    let mut total_cost = 0;
    for r in 0..input.plots.height() {
        for c in 0..input.plots.width() {
            let id = r * stride + c;
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
