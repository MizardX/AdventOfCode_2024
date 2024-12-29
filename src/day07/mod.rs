use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 07");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 3_749)", part_1(&example));
    println!("|'-Part 2: {} (expected 11_387)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 5_540_634_308_362)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected 472_290_821_152_397)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> i64 {
    fn check(target_value: i64, operands: &[i64]) -> bool {
        match operands {
            [] => target_value == 0,
            &[x] => x == target_value,
            &[ref xs @ .., x] => {
                if target_value % x == 0 && check(target_value / x, xs) {
                    return true;
                }
                if x <= target_value && check(target_value - x, xs) {
                    return true;
                }
                false
            }
        }
    }

    let mut sum = 0;
    for equation in &input.equations {
        if check(
            equation.target_value,
            &equation.operands,
        ) {
            sum += equation.target_value;
        }
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> i64 {
    fn ends_with(x: i64, y: i64) -> Option<i64> {
        match y {
            0..=9 => (x % 10 == y).then_some(x / 10),
            10..=99 => (x % 100 == y).then_some(x / 100),
            100..=999 => (x % 1_000 == y).then_some(x / 1_000),
            1_000..=9_999 => (x % 10_000 == y).then_some(x / 10_000),
            10_000..=99_999 => (x % 100_000 == y).then_some(x / 100_000),
            100_000..=999_999 => (x % 1_000_000 == y).then_some(x / 1_000_000),
            1_000_000..=9_999_999 => (x % 10_000_000 == y).then_some(x / 10_000_000),
            10_000_000..=99_999_999 => (x % 100_000_000 == y).then_some(x / 100_000_000),
            100_000_000..=999_999_999 => (x % 1_000_000_000 == y).then_some(x / 1_000_000_000),
            1_000_000_000..=9_999_999_999 => (x % 10_000_000_000 == y).then_some(x / 10_000_000_000),
            10_000_000_000..=99_999_999_999 => (x % 100_000_000_000 == y).then_some(x / 100_000_000_000),
            100_000_000_000..=999_999_999_999 => (x % 1_000_000_000_000 == y).then_some(x / 1_000_000_000_000),
            1_000_000_000_000..=9_999_999_999_999 => (x % 10_000_000_000_000 == y).then_some(x / 10_000_000_000_000),
            10_000_000_000_000..=99_999_999_999_999 => (x % 100_000_000_000_000 == y).then_some(x / 100_000_000_000_000),
            100_000_000_000_000..=999_999_999_999_999 => (x % 1_000_000_000_000_000 == y).then_some(x / 1_000_000_000_000_000),
            1_000_000_000_000_000..=9_999_999_999_999_999 => (x % 10_000_000_000_000_000 == y).then_some(x / 10_000_000_000_000_000),
            10_000_000_000_000_000..=99_999_999_999_999_999 => (x % 100_000_000_000_000_000 == y).then_some(x / 100_000_000_000_000_000),
            _ => None,
        }
    }

    fn check(target_value: i64, operands: &[i64]) -> bool {
        match operands {
            [] => target_value == 0,
            &[x] => x == target_value,
            &[ref xs @ .., x] => {
                if target_value % x == 0 && check(target_value / x, xs) {
                    return true;
                }
                if let Some(rest) = ends_with(target_value, x) {
                    if check(rest, xs) {
                        return true;
                    }
                }
                if x <= target_value && check(target_value - x, xs) {
                    return true;
                }
                false
            }
        }
    }

    let mut sum = 0;
    for equation in &input.equations {
        if check(
            equation.target_value,
            &equation.operands,
        ) {
            sum += equation.target_value;
        }
    }
    sum
}

#[derive(Debug, Clone)]
pub struct Equation {
    target_value: i64,
    operands: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct Input {
    equations: Vec<Equation>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.target_value)?;
        for &operand in &self.operands {
            write!(f, " {operand}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Empty equation")]
    EmptyEquation,
    #[error("Missing character: '{0}'")]
    MissingChar(char),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let equations = text.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { equations })
    }
}

impl FromStr for Equation {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            return Err(ParseInputError::EmptyEquation);
        }
        let (target_str, operands_str) = text
            .split_once(": ")
            .ok_or(ParseInputError::MissingChar(':'))?;
        let target_value = target_str.parse()?;
        let operands = operands_str
            .split(' ')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            target_value,
            operands,
        })
    }
}
