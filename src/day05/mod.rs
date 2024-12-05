use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 05");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 143)", part_1(&example));
    println!("|'-Part 2: {} (expected 123)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6_384)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u32 {
    let mut mid_sum = 0;
    for update in &input.updates {
        if update.is_correct_order(&input.before) {
            let mid = update.values.len() / 2;
            mid_sum += u32::from(update.values[mid]);
        }
    }
    mid_sum
}

#[must_use]
pub fn part_2(input: &Input) -> u32 {
    let mut mid_sum = 0;
    for update in &input.updates {
        if !update.is_correct_order(&input.before) {
            let sorted = update.topological_sort(&input.before, &input.after);
            let mid = sorted.len() / 2;
            println!("{:?} -> {sorted:?} -> {}", update.values, sorted[mid]);
            mid_sum += u32::from(sorted[mid]);
        }
    }
    mid_sum
}

#[derive(Debug, Clone)]
pub struct Update {
    values: Vec<u8>,
}

impl Update {
    #[must_use]
    pub fn is_correct_order(&self, before: &[Vec<u8>; 100]) -> bool {
        let mut invalid = [false; 100];
        for &value in &self.values {
            if invalid[value as usize] {
                return false;
            }
            invalid[value as usize] = true;
            for &blocked in &before[value as usize] {
                invalid[blocked as usize] = true;
            }
        }
        true
    }

    #[must_use]
    pub fn topological_sort(&self, before: &[Vec<u8>; 100], after: &[Vec<u8>; 100]) -> Vec<u8> {
        let mut candidates = Vec::new();
        for &x in &self.values {
            if before[x as usize]
                .iter()
                .all(|&y| !self.values.contains(&y))
            {
                candidates.push(x);
            }
        }
        assert!(!candidates.is_empty(), "No candidates found");
        let mut sorted = Vec::with_capacity(self.values.len());
        let mut added = [false; 100];
        while let Some(x) = candidates.pop() {
            if added[x as usize] {
                continue;
            }
            added[x as usize] = true;
            sorted.push(x);
            for &y in &after[x as usize] {
                if !added[y as usize] && self.values.contains(&y) {
                    candidates.push(y);
                }
            }
        }
        assert!(sorted.len() == self.values.len(), "Not all values sorted");
        sorted
    }
}

impl FromStr for Update {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let values = text
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        Ok(Self { values })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    before: [Vec<u8>; 100], // Maybe u128 bitmask?
    after: [Vec<u8>; 100],  // Maybe u128 bitmask?
    updates: Vec<Update>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    MissingChar(char),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if text.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        let mut lines = text.lines();
        let mut before = [(); 100].map(|()| Vec::new());
        let mut after = [(); 100].map(|()| Vec::new());
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (left, right) = line
                .split_once('|')
                .ok_or(ParseInputError::MissingChar('|'))?;
            let left = left.parse()?;
            let right = right.parse()?;
            before[right as usize].push(left);
            after[left as usize].push(right);
        }
        if before.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        let mut updates = Vec::new();
        for line in lines {
            updates.push(line.parse()?);
        }
        if updates.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        Ok(Self {
            before,
            after,
            updates,
        })
    }
}
