use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 19");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 6)", part_1(&example));
    println!("|'-Part 2: {} (expected 16)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 293)", part_1(&input));
    println!("|'-Part 2: {} (expected 623_924_810_770_264)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut count = 0;
    for pattern in &input.target_patterns {
        // if pattern can be built from pieces, increment count
        let n = pattern.colors.len();
        let mut dp = vec![false; n + 1];
        dp[n] = true;
        for i in (0..n).rev() {
            for piece in &input.pieces {
                if pattern.colors[i..].starts_with(&piece.colors) && dp[i + piece.colors.len()] {
                    dp[i] = true;
                    break; // for piece
                }
            }
        }
        if dp[0] {
            count += 1;
        }
    }
    count
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut sum_counts = 0;
    for pattern in &input.target_patterns {
        // if pattern can be built from pieces, increment count
        let n = pattern.colors.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        for i in (0..n).rev() {
            for piece in &input.pieces {
                if pattern.colors[i..].starts_with(&piece.colors) {
                    dp[i] += dp[i + piece.colors.len()];
                }
            }
        }
        sum_counts += dp[0];
    }
    sum_counts
}

// white (w), blue (u), black (b), red (r), or green (g)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<u8> for Color {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'w' => Ok(Self::White),
            b'u' => Ok(Self::Blue),
            b'b' => Ok(Self::Black),
            b'r' => Ok(Self::Red),
            b'g' => Ok(Self::Green),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    colors: Vec<Color>,
}

impl FromStr for Pattern {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s
            .trim()
            .bytes()
            .map(Color::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { colors })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pieces: Vec<Pattern>,
    target_patterns: Vec<Pattern>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Incomplete input")]
    IncompleteInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let pieces = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if lines.next() != Some("") {
            return Err(ParseInputError::IncompleteInput);
        }
        let target_patterns = lines
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            pieces,
            target_patterns,
        })
    }
}
