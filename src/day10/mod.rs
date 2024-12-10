use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 10");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 36)", part_1(&example));
    println!("|'-Part 2: {} (expected 81)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 786)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_722)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    fn collect_paths(elevation: &Grid<Elevation>, expected: Elevation, r: usize, c: usize, goals_reached: &mut HashSet<(usize, usize)>) {
        if elevation.get(c, r) != Some(&expected) {
            return;
        }
        let Some(next) = expected.next() else {
            goals_reached.insert((r, c));
            return;
        };
        for &(dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let (Some(r), Some(c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                if r < elevation.height() && c < elevation.width() {
                    collect_paths(elevation, next, r, c, goals_reached);
                }
            }
        }
    }
    let mut score = 0;
    let mut goals_reached = HashSet::new();
    for (r, row) in input.elevations.rows().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != Elevation::H0 {
                continue;
            }
            goals_reached.clear();
            collect_paths(&input.elevations, Elevation::H0, r, c, &mut goals_reached);
            score += goals_reached.len();
        }
    }
    score
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    fn collect_paths(elevation: &Grid<Elevation>, expected: Elevation, r: usize, c: usize) -> usize {
        if elevation.get(c, r) != Some(&expected) {
            return 0;
        }
        let Some(next) = expected.next() else {
            return 1;
        };
        let mut score = 0;
        for &(dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let (Some(r), Some(c)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                if r < elevation.height() && c < elevation.width() {
                    score += collect_paths(elevation, next, r, c);
                }
            }
        }
        score
    }
    let mut score = 0;
    for (r, row) in input.elevations.rows().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != Elevation::H0 {
                continue;
            }
            score += collect_paths(&input.elevations, Elevation::H0, r, c);
        }
    }
    score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Elevation {
    H0,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
    H9,
}

impl Elevation {
    pub fn next(self) -> Option<Self> {
        Some(match self {
            Self::H0 => Self::H1,
            Self::H1 => Self::H2,
            Self::H2 => Self::H3,
            Self::H3 => Self::H4,
            Self::H4 => Self::H5,
            Self::H5 => Self::H6,
            Self::H6 => Self::H7,
            Self::H7 => Self::H8,
            Self::H8 => Self::H9,
            Self::H9 => None?,
        })
    }
}

impl TryFrom<u8> for Elevation {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(Self::H0),
            b'1' => Ok(Self::H1),
            b'2' => Ok(Self::H2),
            b'3' => Ok(Self::H3),
            b'4' => Ok(Self::H4),
            b'5' => Ok(Self::H5),
            b'6' => Ok(Self::H6),
            b'7' => Ok(Self::H7),
            b'8' => Ok(Self::H8),
            b'9' => Ok(Self::H9),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    elevations: Grid<Elevation>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let elevations = text.parse()?;
        Ok(Self { elevations })
    }
}
