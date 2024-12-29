use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 25");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 3)", part_1(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 3065)", part_1(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut count = 0;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for &schematic in &input.schematics {
        match schematic {
            Schematic::Lock(lock) => {
                locks.push(lock);
                for &key in &keys {
                    if (key & lock) == 0 {
                        count += 1;
                    }
                }
            }
            Schematic::Key(key) => {
                keys.push(key);
                for &lock in &locks {
                    if (key & lock) == 0 {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[derive(Debug, Clone, Copy)]
enum Schematic {
    Lock(u32),
    Key(u32),
}

impl TryFrom<&[&str]> for Schematic {
    type Error = ParseInputError;

    #[expect(clippy::cast_possible_truncation)]
    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        if value.len() != 7 || value.iter().map(|row| row.len()).max() != Some(5) {
            return Err(ParseInputError::InvalidSize(
                value.len() as u8,
                value[0].len() as u8,
            ));
        }
        if value[0].bytes().all(|c| c == b'#') {
            // Lock, has downwards pointing pins
            let mut lock = 0;
            for (row, &line) in (0..).zip(&value[1..6]) {
                for (col, ch) in line.bytes().enumerate() {
                    match ch {
                        b'#' => lock |= 1 << (row * 5 + col),
                        b'.' => (),
                        _ => return Err(ParseInputError::InvalidChar(ch as char)),
                    }
                }
            }
            Ok(Self::Lock(lock))
        } else {
            // Key, has upwards pointing pins
            let mut key = 0;
            for (row, &line) in (0..).zip(&value[1..6]) {
                for (col, ch) in line.bytes().enumerate() {
                    match ch {
                        b'#' => key |= 1 << (row * 5 + col),
                        b'.' => (),
                        _ => return Err(ParseInputError::InvalidChar(ch as char)),
                    }
                }
            }
            Ok(Self::Key(key))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    schematics: Vec<Schematic>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Invalid schematic size: {0}x{1}")]
    InvalidSize(u8, u8),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut schematics = Vec::new();
        let mut lines = Vec::new();
        for line in text.lines() {
            if line.is_empty() {
                if !lines.is_empty() {
                    schematics.push(Schematic::try_from(lines.as_slice())?); // Just &lines doesn't work
                    lines.clear();
                }
            } else {
                lines.push(line);
            }
        }
        if !lines.is_empty() {
            schematics.push(Schematic::try_from(lines.as_slice())?);
        }
        Ok(Self { schematics })
    }
}
