use std::path::Iter;
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
    let mut s = input.data.as_str();
    while let Some((pos, pattern)) = find_any(s, &["mul(", "do()", "don't()"]) {
        match pattern {
            0 => {
                if enabled {
                    if let Some(product) = parse_mul(&s[pos + 4..]) {
                        sum += product;
                    }
                }
                s = &s[pos + 4..];
            }
            1 => {
                enabled = true;
                s = &s[pos + 4..];
            }
            2 => {
                enabled = false;
                s = &s[pos + 7..];
            }
            _ => unreachable!(),
        }
    }
    sum
}

fn find_any<'a>(haystack: &'a str, needles: &[&'a str]) -> Option<(usize, usize)> {
    let mut min_pos = haystack.len();
    let mut min_ix = 0;
    for (ix, needle_str) in needles.iter().enumerate() {
        if let Some(pos) = haystack.find(needle_str) {
            if pos < min_pos {
                min_pos = pos;
                min_ix = ix;
            }
        }
    }
    if min_pos == haystack.len() {
        return None;
    }
    Some((min_pos, min_ix))
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
