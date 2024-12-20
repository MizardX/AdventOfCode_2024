use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 20");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 44)", part_1(&example, 1));
    println!("|'-Part 2: {} (expected 285)", part_2(&example, 50));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1_395)", part_1(&input, 100));
    println!("|'-Part 2: {} (expected 993_178)", part_2(&input, 100));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input, least_saved: usize) -> usize {
    count_cheats(input, 2, least_saved)
}

#[must_use]
pub fn part_2(input: &Input, least_saved: usize) -> usize {
    count_cheats(input, 20, least_saved)
}

#[must_use]
pub fn count_cheats(input: &Input, max_skip: usize, least_saved: usize) -> usize {
    let normal_path = dfs(
        input.start,
        |pos| {
            neighbors(pos, input).filter_map(|(x, y)| {
                input
                    .grid
                    .get(x, y)
                    .unwrap()
                    .is_passable()
                    .then_some((x, y))
            })
        },
        |pos| pos == input.end,
    )
    .unwrap();
    let mut cheat_count = 0;
    for (i, &(x1, y1)) in normal_path.iter().enumerate() {
        for (skip, &(x2, y2)) in normal_path[i..].iter().enumerate() {
            let dist = x1.abs_diff(x2) + y1.abs_diff(y2);
            let saved = skip.saturating_sub(dist);
            if dist <= max_skip && saved >= least_saved {
                cheat_count += 1;
            }
        }
    }
    cheat_count
}

fn dfs<P, N, NI, G>(start: P, neighbors: N, mut goal: G) -> Option<Vec<P>>
where
    P: Copy + Debug + Eq + std::hash::Hash,
    N: Fn(P) -> NI,
    NI: Iterator<Item = P>,
    G: FnMut(P) -> bool,
{
    let mut visited = HashSet::new();
    let mut stack = vec![(start, false)];
    let mut path = Vec::new();
    while let Some((pos, backtracking)) = stack.pop() {
        if backtracking {
            visited.remove(&pos);
            path.pop();
            continue;
        }
        if !visited.insert(pos) {
            continue;
        }
        path.push(pos);
        if goal(pos) {
            return Some(path);
        }
        stack.push((pos, true));
        for pos in neighbors(pos) {
            stack.push((pos, false));
        }
    }
    None
}

fn neighbors((x, y): (usize, usize), input: &Input) -> impl Iterator<Item = (usize, usize)> + '_ {
    [
        (x + 1, y, x + 1 < input.grid.width()),
        (x, y + 1, y + 1 < input.grid.height()),
        (x.saturating_sub(1), y, x > 0),
        (x, y.saturating_sub(1), y > 0),
    ]
    .into_iter()
    .filter_map(|(x, y, valid)| valid.then_some((x, y)))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl Tile {
    const fn is_passable(self) -> bool {
        matches!(self, Self::Empty | Self::Start | Self::End)
    }
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            b'.' => Self::Empty,
            b'#' => Self::Wall,
            b'S' => Self::Start,
            b'E' => Self::End,
            _ => Err(ParseInputError::InvalidChar(c as char))?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Tile>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Missing start tile")]
    MissingStart,
    #[error("Missing end tile")]
    MissingEnd,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let grid: Grid<Tile> = text.parse()?;
        let mut start = None;
        let mut end = None;
        for (y, row) in grid.rows().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Start => {
                        start = Some((x, y));
                    }
                    Tile::End => {
                        end = Some((x, y));
                    }
                    _ => {}
                }
            }
        }
        let Some(start) = start else {
            return Err(ParseInputError::MissingStart);
        };
        let Some(end) = end else {
            return Err(ParseInputError::MissingEnd);
        };
        Ok(Self { grid, start, end })
    }
}
