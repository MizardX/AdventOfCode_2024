use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 11");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 55_312)", part_1(&example));
    println!("|'-Part 2: {} (expected 65_601_038_650_482)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 218_079)", part_1(&input));
    println!("|'-Part 2: {} (expected 259_755_538_429_618)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input.stones.iter().map(|&val| count_after_split(val, 25, &mut cache)).sum()
}

fn count_after_split(
    val: u64,
    times: usize,
    cache: &mut HashMap<(u64, usize), usize>,
) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(&count) = cache.get(&(val, times)) {
        return count;
    }
    let mut count = 0;
    if val == 0 {
        count += count_after_split(1, times - 1, cache);
    } else if let Some((left, right)) = split_in_half(val) {
        count += count_after_split(left, times - 1, cache);
        count += count_after_split(right, times - 1, cache);
    } else {
        count += count_after_split(val * 2024, times - 1, cache);
    }
    cache.insert((val, times), count);
    count
}

const fn num_digits(val: u64) -> usize {
    let mut digits = 0;
    let mut val = val;
    while val > 0 {
        digits += 1;
        val /= 10;
    }
    digits
}

fn split_in_half(val: u64) -> Option<(u64, u64)> {
    let digits = num_digits(val);
    if digits % 2 != 0 {
        return None;
    }
    let half = digits / 2;
    let mut scale = 1;
    for _ in 0..half {
        scale *= 10;
    }
    let left = val / scale;
    let right = val % scale;
    Some((left, right))
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input.stones.iter().map(|&val| count_after_split(val, 75, &mut cache)).sum()
}

#[derive(Debug, Clone)]
pub struct Input {
    stones: Vec<u64>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut stones = Vec::new();
        for word in text.split_whitespace() {
            stones.push(word.parse()?);
        }
        Ok(Self { stones })
    }
}
