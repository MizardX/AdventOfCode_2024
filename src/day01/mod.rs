use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 01");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 11)", part_1(&example));
    println!("|'-Part 2: {} (expected 31)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1,646,452)", part_1(&input));
    println!("|'-Part 2: {} (expected 23,609,874)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u32 {
    let mut lefts: Vec<_> = input.lefts.clone();
    lefts.sort_unstable();
    let mut rights: Vec<_> = input.rights.clone();
    rights.sort_unstable();
    let mut sum = 0;
    for (&a, &b) in lefts.iter().zip(rights.iter()) {
        sum += a.abs_diff(b);
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> u32 {
    const MIN: u32 = 0;
    const MAX: u32 = 99_999;
    let mut freq_left = vec![0; (MAX - MIN + 1) as usize];
    let mut freq_right = vec![0; (MAX - MIN + 1) as usize];
    for &left in &input.lefts {
        freq_left[(left - MIN) as usize] += 1;
    }
    for &right in &input.rights {
        freq_right[(right - MIN) as usize] += 1;
    }
    let mut sum = 0;
    for (i, (&left, &right)) in freq_left.iter().zip(freq_right.iter()).enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        let x = i as u32 + MIN;
        sum += x * left * right;
    }
    sum
}

#[derive(Debug, Clone)]
pub struct Input {
    lefts: Vec<u32>,
    rights: Vec<u32>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected line format")]
    InvalidFormat,
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        let mut lefts = Vec::new();
        let mut rights = Vec::new();
        for line in text.lines() {
            let (left_str, right_str) =
                line.split_once(' ').ok_or(ParseInputError::InvalidFormat)?;
            let left = left_str.parse()?;
            let right = right_str.trim_start().parse()?;
            lefts.push(left);
            rights.push(right);
        }
        Ok(Self { lefts, rights })
    }
}