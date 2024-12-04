use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 04");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 18)", part_1(&example));
    println!("|'-Part 2: {} (expected 9)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 2_583)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_978)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut count = 0;
    for (r, row) in input.grid.rows().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if c >= 3 {
                match (row[c - 3], row[c - 2], row[c - 1], cell) {
                    (Cell::X, Cell::M, Cell::A, Cell::S) | (Cell::S, Cell::A, Cell::M, Cell::X) => {
                        count += 1;
                    }
                    _ => (),
                }
            }
            if r >= 3 {
                match (
                    input.grid.get(c, r - 3).unwrap(),
                    input.grid.get(c, r - 2).unwrap(),
                    input.grid.get(c, r - 1).unwrap(),
                    cell,
                ) {
                    (Cell::X, Cell::M, Cell::A, Cell::S) | (Cell::S, Cell::A, Cell::M, Cell::X) => {
                        count += 1;
                    }
                    _ => (),
                }
                if c >= 3 {
                    match (
                        input.grid.get(c - 3, r - 3).unwrap(),
                        input.grid.get(c - 2, r - 2).unwrap(),
                        input.grid.get(c - 1, r - 1).unwrap(),
                        cell,
                    ) {
                        (Cell::X, Cell::M, Cell::A, Cell::S)
                        | (Cell::S, Cell::A, Cell::M, Cell::X) => {
                            count += 1;
                        }
                        _ => (),
                    }
                }
                if c + 3 < input.grid.width() {
                    match (
                        input.grid.get(c + 3, r - 3).unwrap(),
                        input.grid.get(c + 2, r - 2).unwrap(),
                        input.grid.get(c + 1, r - 1).unwrap(),
                        cell,
                    ) {
                        (Cell::X, Cell::M, Cell::A, Cell::S)
                        | (Cell::S, Cell::A, Cell::M, Cell::X) => {
                            count += 1;
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    count
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let width = input.grid.width();
    let rows = input.grid.height();
    let mut count = 0;
    for (r, row) in input.grid.rows().enumerate().skip(1).take(rows - 2) {
        for (c, &cell) in row.iter().enumerate().skip(1).take(width - 2) {
            if !matches!(cell, Cell::A) {
                continue;
            }
            match (
                input.grid.get(c - 1, r - 1).unwrap(),
                input.grid.get(c + 1, r + 1).unwrap(),
                input.grid.get(c + 1, r - 1).unwrap(),
                input.grid.get(c - 1, r + 1).unwrap(),
            ) {
                (Cell::M, Cell::S, Cell::M, Cell::S)
                | (Cell::M, Cell::S, Cell::S, Cell::M)
                | (Cell::S, Cell::M, Cell::M, Cell::S)
                | (Cell::S, Cell::M, Cell::S, Cell::M) => {
                    count += 1;
                }
                _ => (),
            }
        }
    }
    count
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    X,
    M,
    A,
    S,
}

impl TryFrom<u8> for Cell {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Self::X),
            b'M' => Ok(Self::M),
            b'A' => Ok(Self::A),
            b'S' => Ok(Self::S),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => 'X',
                Self::M => 'M',
                Self::A => 'A',
                Self::S => 'S',
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Cell>,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            grid: text.parse()?,
        })
    }
}
