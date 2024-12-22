use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 22");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 37_327_623)", part_1(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example");
    println!("|+-Part 2: {} (expected 23)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 12_664_695_565)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_444)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    let mut sum = 0;
    for &secret_number in &input.secret_numbers {
        sum += generate(secret_number).nth(1_999).unwrap_or(0);
    }
    sum
}

fn step(mut value: u64) -> u64 {
    // mix(a,b) = a ^ b
    // prune(x) = x & 0xFFFFFF
    // x = prune(mix(x, x<<6))
    // x = prune(mix(x, x>>5))
    // x = prune(mix(x, x<<11))
    value = ((value << 6) ^ value) & 0xFF_FFFF;
    value = ((value >> 5) ^ value) & 0xFF_FFFF;
    value = ((value << 11) ^ value) & 0xFF_FFFF;
    value
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let mut scores = HashMap::<(i8, i8, i8, i8), u64>::new();
    for &secret_number in &input.secret_numbers {
        let new_scores = process_secret_number(secret_number);
        update_scores(&mut scores, &new_scores);
    }
    scores.values().max().copied().unwrap_or(0)
}

fn process_secret_number(secret_number: u64) -> HashMap<(i8, i8, i8, i8), u64> {
    let mut diffs = VecDeque::new();
    let mut new_scores = HashMap::<(i8, i8, i8, i8), u64>::new();
    let mut prev_price = (secret_number % 10) as u8;

    for value in PseduoRandom::new(secret_number).take(2_000) {
        let price = (value % 10) as u8;
        let price_i8 = i8::try_from(price).expect("price is always < 10");
        let prev_price_i8 = i8::try_from(prev_price).expect("prev_price is always < 10");
        let diff = price_i8 - prev_price_i8;

        diffs.push_back(diff);

        if diffs.len() == 4 {
            let key = (diffs[0], diffs[1], diffs[2], diffs[3]);
            diffs.pop_front();
            new_scores.entry(key).or_insert(u64::from(price));
        }

        prev_price = price;
    }

    new_scores
}

fn update_scores(
    scores: &mut HashMap<(i8, i8, i8, i8), u64>,
    new_scores: &HashMap<(i8, i8, i8, i8), u64>,
) {
    for (&key, &score) in new_scores {
        *scores.entry(key).or_insert(0) += score;
    }
}

struct PseduoRandom {
    value: u64,
}

impl PseduoRandom {
    fn new(secret_number: u64) -> Self {
        Self {
            value: secret_number,
        }
    }
}

impl Iterator for PseduoRandom {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // mix(a,b) = a ^ b
        // prune(x) = x % 16777216
        // x = prune(mix(x, x * 32))
        // x = prune(mix(x, x / 16))
        // x = prune(mix(x, x * 2048))
        let mut value = self.value;
        value = ((value << 6) ^ value) & 0xFF_FFFF;
        value = ((value >> 5) ^ value) & 0xFF_FFFF;
        value = ((value << 11) ^ value) & 0xFF_FFFF;
        self.value = value;
        Some(self.value)
    }
}

fn generate(secret_number: u64) -> PseduoRandom {
    PseduoRandom::new(secret_number)
}

#[derive(Debug, Clone)]
pub struct Input {
    secret_numbers: Vec<u64>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let secret_numbers = text.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { secret_numbers })
    }
}
