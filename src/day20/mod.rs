use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 20");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 44)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let shortest_path = pathfinding::directed::dfs::dfs(
        &input.start,
        |&pos| neighbors(pos, input).filter(|&pos| input.grid.get(pos.0, pos.1).unwrap().is_passable()).copied(),
        |&pos| pos == &input.end,
    );
    todo!()
}

fn neighbors((x, y): (usize, usize), input: &Input) -> impl Iterator<Item = (usize, usize)> {
    [(x + 1, y, x+1 < input.grid.width()),
    (x, y + 1, y+1 < input.grid.height()), (x.saturating_sub(1), y, x > 0), (x, y.saturating_sub(1), y > 0)]
        .into_iter()
        .filter_map(|(x, y, valid)| if valid { Some((x, y)) } else { None })
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
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
    fn is_passable(&self) -> bool {
        match self {
            Self::Empty | Self::Start | Self::End => true,
            _ => false,
        }
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
        let Some(start) = start else { return Err(ParseInputError::MissingStart) };
        let Some(end) = end else { return Err(ParseInputError::MissingEnd) };
        Ok(Self { grid, start, end })
    }
}
