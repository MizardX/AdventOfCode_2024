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
    let mut s = input.data.as_str();
    let mut enabled = true;
    loop {
        match (s.find("mul("), s.find("do()"), s.find("don't()")) {
            (Some(mul), Some(do_), Some(dont)) => {
                if mul < do_ && mul < dont {
                    if enabled {
                        if let Some(product) = parse_mul(&s[mul+4..]) {
                            sum += product;
                        }
                    }
                    s = &s[(mul + 4)..];
                } else if do_ < dont {
                    enabled = true;
                    s = &s[do_ + 4..];
                } else {
                    enabled = false;
                    s = &s[dont + 7..];
                }
            }
            (Some(mul), Some(do_), None) => {
                if mul < do_ {
                    if enabled {
                        if let Some(product) = parse_mul(&s[mul+4..]) {
                            sum += product;
                        }
                    }
                    s = &s[(mul + 4)..];
                } else {
                    enabled = true;
                    s = &s[do_ + 4..];
                }
            }
            (Some(mul), None, Some(dont)) => {
                if mul < dont {
                    if enabled {
                        if let Some(product) = parse_mul(&s[mul+4..]) {
                            sum += product;
                        }
                    }
                    s = &s[(mul + 4)..];
                } else {
                    enabled = false;
                    s = &s[dont + 7..];
                }
            }
            (Some(mul), None, None) => {
                if enabled {
                    if let Some(product) = parse_mul(&s[mul+4..]) {
                        sum += product;
                    }
                }
                s = &s[(mul + 4)..];
            }
            (None, Some(do_), Some(dont)) => {
                if do_ < dont {
                    enabled = true;
                    s = &s[do_ + 4..];
                } else {
                    enabled = false;
                    s = &s[dont + 7..];
                }
            }
            (None, Some(do_), None) => {
                enabled = true;
                s = &s[do_ + 4..];
            }
            (None, None, Some(dont)) => {
                enabled = false;
                s = &s[dont + 7..];
            }
            (None, None, None) => {
                break;
            }
        }
    }
    sum
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
