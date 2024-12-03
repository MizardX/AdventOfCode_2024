use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 03");

    println!("++Example");
    let example1 = EXAMPLE1.parse().expect("Parse example1");
    println!("|+-Part 1: {} (expected 161)", part_1(&example1));
    let example2 = EXAMPLE2.parse().expect("Parse example2");
    println!("|'-Part 2: {} (expected 48)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 166_357_705)", part_1(&input));
    println!("|'-Part 2: {} (expected 88_811_886)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    for part in input.data.split("mul(").skip(1) {
        if let Some(product) = parse_mul(part) {
            sum += product;
        }
    }
    sum
}

fn parse_mul(part: &str) -> Option<usize> {
    fn first_4(s: &str) -> &str {
        &s[..s.len().min(4)]
    }
    let comma = first_4(part).find(',')?;
    let paren = first_4(&part[comma + 1..]).find(')')? + comma + 1;
    let a: usize = part[..comma].parse().ok()?;
    let b: usize = part[comma + 1..paren].parse().ok()?;
    Some(a * b)
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut sum = 0;
    let mut enabled = true;
    let data = input.data.as_str();
    for (pos, pattern) in FindAny::new(data, &["mul(", "do()", "don't()"]) {
        match pattern {
            0 if enabled => {
                if let Some(product) = parse_mul(&data[pos + 4..]) {
                    sum += product;
                }
            }
            1 => enabled = true,
            2 => enabled = false,
            _ => (),
        }
    }
    sum
}

struct FindAny<'a> {
    haystack: &'a str,
    patterns: BinaryHeap<(Reverse<usize>, usize, &'a str)>,
}

impl<'a> FindAny<'a> {
    fn new(haystack: &'a str, patterns: &[&'a str]) -> Self {
        let patterns = patterns
            .iter()
            .enumerate()
            .filter_map(|(ix, &pattern)| Some((Reverse(haystack.find(pattern)?), ix, pattern)))
            .collect();
        Self { haystack, patterns }
    }
}

impl Iterator for FindAny<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (Reverse(pos), ix, pattern) = self.patterns.pop()?;
        let len = pattern.len();
        if let Some(next) = self.haystack[pos + len..].find(pattern) {
            self.patterns.push((Reverse(pos + len + next), ix, pattern));
        }
        Some((pos, ix))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    data: String,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            data: text.to_string(),
        })
    }
}
