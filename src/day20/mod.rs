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
    let (part1, part2) = part_1_and_2(&example, 1);
    println!("|+-Part 1: {part1} (expected 44)");
    println!("|'-Part 2: {part2} (expected 3_081)");

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    let (part1, part2) = part_1_and_2(&input, 100);
    println!("|+-Part 1: {part1} (expected 1_395)");
    println!("|'-Part 2: {part2} (expected 993_178)");
    println!("')");
}

#[must_use]
pub fn part_1_and_2(input: &Input, least_saved: usize) -> (usize, usize) {
    let normal_path = walk_path(
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
    );
    let mut cheat_count_short = 0;
    let mut cheat_count_long = 0;
    for (i, &(x1, y1)) in normal_path.iter().enumerate() {
        for (skip_length, &(x2, y2)) in normal_path[i..].iter().enumerate() {
            let jump_dist = x1.abs_diff(x2) + y1.abs_diff(y2);
            let saved = skip_length.saturating_sub(jump_dist);
            if saved >= least_saved && jump_dist <= 20 {
                cheat_count_long += 1;
                if jump_dist <= 2 {
                    cheat_count_short += 1;
                }
            }
        }
    }
    (cheat_count_short, cheat_count_long)
}

fn walk_path<P, N, NI, G>(start: P, neighbors: N, mut goal: G) -> Vec<P>
where
    P: Copy + Eq,
    N: Fn(P) -> NI,
    NI: IntoIterator<Item = P>,
    G: FnMut(P) -> bool,
{
    // There is only a single path from start to goal
    let mut path = vec![];
    let mut node = start;
    let mut prev = None;
    while !goal(node) {
        path.push(node);
        (prev, node) = (Some(node), neighbors(node)
            .into_iter()
            .find(|&n| Some(n) != prev)
            .unwrap());
    }
    path.push(node);
    path
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
