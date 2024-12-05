use std::{collections::HashMap, str::FromStr};
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
    println!("|'-Part 2: {} (expected 5_353)", part_2(&input));
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
            let sorted_middle = update.sorted_middle(&input.before);
            mid_sum += u32::from(sorted_middle);
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
    pub fn is_correct_order(&self, before: &HashMap<u8, Vec<u8>>) -> bool {
        let mut invalid = [false; 100];
        for &value in &self.values {
            if invalid[value as usize] {
                return false;
            }
            invalid[value as usize] = true;
            for &blocked in before.get(&value).map(Vec::as_slice).unwrap_or_default() {
                invalid[blocked as usize] = true;
            }
        }
        true
    }

    #[must_use]
    pub fn sorted_middle(&self, before: &HashMap<u8, Vec<u8>>) -> u8 {
        let mut sorted = Vec::new();
        let mut visited = [false; 100];
        for &value in &self.values {
            if !visited[value as usize] {
                self.topological_sort(value, before, &mut visited, &mut sorted);
            }
        }
        assert!(sorted.len() == self.values.len(), "Topological sort failed. {:?} -> {sorted:?}", self.values);
        let mid = sorted.len() / 2;
        sorted[mid]
    }

    fn topological_sort(
        &self,
        value: u8,
        before: &HashMap<u8, Vec<u8>>,
        visited: &mut [bool; 100],
        sorted: &mut Vec<u8>,
    ) {
        visited[value as usize] = true;
        for &blocked in before.get(&value).map(Vec::as_slice).unwrap_or_default() {
            if self.values.contains(&blocked) && !visited[blocked as usize] {
                self.topological_sort(blocked, before, visited, sorted);
            }
        }
        sorted.push(value);
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
    before: HashMap<u8, Vec<u8>>, // Maybe u128 bitmask?
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
        let mut before = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (left, right) = line
                .split_once('|')
                .ok_or(ParseInputError::MissingChar('|'))?;
            let left: u8 = left.parse()?;
            let right: u8 = right.parse()?;
            before.entry(right).or_insert_with(Vec::new).push(left);
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
            updates,
        })
    }
}
