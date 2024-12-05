use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 05");

    println!("++Example");
    let mut example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 143)", part_1(&example));
    println!("|'-Part 2: {} (expected 123)", part_2(&mut example));

    println!("++Input");
    let mut input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6_384)", part_1(&input));
    println!("|'-Part 2: {} (expected 5_353)", part_2(&mut input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u32 {
    let mut mid_sum = 0;
    for update in &input.updates {
        if update.is_correct_order(&input.rules) {
            let mid = update.values.len() / 2;
            mid_sum += u32::from(update.values[mid]);
        }
    }
    mid_sum
}

#[must_use]
pub fn part_2(input: &mut Input) -> u32 {
    let mut mid_sum = 0;
    for update in &mut input.updates {
        if !update.is_correct_order(&input.rules) {
            let sorted_middle = update.sorted_middle(&input.rules);
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
    pub fn is_correct_order(&self, rules: &HashSet<(u8, u8)>) -> bool {
        self.values.is_sorted_by(|&a, &b| rules.contains(&(a, b)))
    }

    pub fn sort(&mut self, rules: &HashSet<(u8, u8)>) {
        self.values.sort_unstable_by(|&a, &b| {
            if rules.contains(&(a, b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    #[must_use]
    pub fn sorted_middle(&mut self, rules: &HashSet<(u8, u8)>) -> u8 {
        self.sort(rules);
        let mid = self.values.len() / 2;
        self.values[mid]
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
    rules: HashSet<(u8, u8)>, // Maybe u128 bitmask?
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
        let mut rules = HashSet::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (left, right) = line
                .split_once('|')
                .ok_or(ParseInputError::MissingChar('|'))?;
            let left: u8 = left.parse()?;
            let right: u8 = right.parse()?;
            rules.insert((left, right));
        }
        if rules.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }

        let mut updates = Vec::new();
        for line in lines {
            updates.push(line.parse()?);
        }
        if updates.is_empty() {
            return Err(ParseInputError::EmptyInput);
        }
        Ok(Self { rules, updates })
    }
}
