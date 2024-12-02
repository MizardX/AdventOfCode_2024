use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 02");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 2)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 486)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut safe_count = 0;
    for report in &input.reports {
        if report.is_safe() {
            safe_count += 1;
        }
    }
    safe_count
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Debug, Clone)]
pub struct Input {
    reports: Vec<Report>,
}

#[derive(Debug, Clone)]
pub struct Report {
    levels: Vec<u8>,
}
impl Report {
    fn is_safe(&self) -> bool {
        self.levels.len() <= 1 || Self::is_safe_range(&self.levels)
    }

    fn is_safe_range(vals: &[u8]) -> bool {
        Self::is_safe_increasing(vals) || Self::is_safe_decreasing(vals)
    }

    fn is_safe_increasing(vals: &[u8]) -> bool {
        let mut prev = vals[0];
        for &val in &vals[1..] {
            if val <= prev || val > prev + 3 {
                return false;
            }
            prev = val;
        }
        true
    }

    fn is_safe_decreasing(vals: &[u8]) -> bool {
        let mut prev = vals[0];
        for &val in &vals[1..] {
            if val >= prev || val + 3 < prev {
                return false;
            }
            prev = val;
        }
        true
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut reports = Vec::new();
        for line in text.lines() {
            let report = line.parse()?;
            reports.push(report);
        }
        Ok(Self { reports })
    }
}

impl FromStr for Report {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut levels = Vec::new();
        for num_str in text.split(' ') {
            let level = num_str.parse()?;
            levels.push(level);
        }
        Ok(Self { levels })
    }
}
