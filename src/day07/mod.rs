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
    fn check(target_value: i64, sum: i64, operands: &[i64]) -> bool {
        if sum > target_value {
            return false;
        }
        match operands {
            [] => sum == target_value,
            &[x, ref xs @ ..] => {
                check(target_value, sum + x, xs) || check(target_value, sum * x, xs)
            }
        }
    }

    let mut sum = 0;
    for equation in &input.equations {
        if check(
            equation.target_value,
            equation.operands[0],
            &equation.operands[1..],
        ) {
            sum += equation.target_value;
        }
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> i64 {
    fn concat(x: i64, y: i64) -> i64 {
        match y {
            0..=9 => 10*x + y,
            10..=99 => 100*x + y,
            100..=999 => 1_000*x + y,
            1_000..=9_999 => 10_000*x + y,
            10_000..=99_999 => 100_000*x + y,
            100_000..=999_999 => 1_000_000*x + y,
            1_000_000..=9_999_999 => 10_000_000*x + y,
            10_000_000..=99_999_999 => 100_000_000*x + y,
            100_000_000..=999_999_999 => 1_000_000_000*x + y,
            1_000_000_000..=9_999_999_999 => 10_000_000_000*x + y,
            10_000_000_000..=99_999_999_999 => 100_000_000_000*x + y,
            100_000_000_000..=999_999_999_999 => 1_000_000_000_000*x + y,
            1_000_000_000_000..=9_999_999_999_999 => 10_000_000_000_000*x + y,
            10_000_000_000_000..=99_999_999_999_999 => 100_000_000_000_000*x + y,
            100_000_000_000_000..=999_999_999_999_999 => 1_000_000_000_000_000*x + y,
            1_000_000_000_000_000..=9_999_999_999_999_999 => 10_000_000_000_000_000*x + y,
            10_000_000_000_000_000..=99_999_999_999_999_999 => 100_000_000_000_000_000*x + y,
            _ => 1_000_000_000_000_000_000*x + y,
        }
    }

    fn check(target_value: i64, sum: i64, operands: &[i64]) -> bool {
        if sum > target_value {
            return false;
        }
        match operands {
            [] => sum == target_value,
            &[x, ref xs @ ..] => {
                check(target_value, sum + x, xs)
                    || check(target_value, sum * x, xs)
                    || check(target_value, concat(sum, x), xs)
            }
        }
    }

    let mut sum = 0;
    for equation in &input.equations {
        if check(
            equation.target_value,
            equation.operands[0],
            &equation.operands[1..],
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
        Ok(Input { equations })
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
        Ok(Equation {
            target_value,
            operands,
        })
    }
}
