use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 12");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 480)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    // println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> i64 {
    let mut cost = 0;
    for claw_machine in &input.claw_machines {
        if let Some((x, y)) = dbg!(claw_machine.button_presses()) {
            cost += x * 3 + y;
        }
    }
    cost
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn button_presses(&self) -> Option<(i64, i64)> {
        // Press A X times, then B Y times to reach the prize
        // A * X + B * Y = Prize
        // X = (Prize.y * B.x - Prize.x * B.y) / (A.x * B.y - A.y * B.x)
        // Y = (A.x * Prize.y - A.y * Prize.x) / (A.x * B.y - A.y * B.x)
        let denom = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        if denom == 0 {
            return None;
        }
        let numer_x = self.prize.0 * self.button_b.1 - self.prize.1 * self.button_b.0;
        let numer_y = self.button_a.0 * self.prize.1 - self.button_a.1 * self.prize.0;
        if numer_x / denom < 0 || numer_y / denom < 0 {
            return None;
        }
        let remainder_x = numer_x % denom;
        let remainder_y = numer_y % denom;
        if remainder_x != 0 || remainder_y != 0 {
            return None;
        }
        Some((numer_x / denom, numer_y / denom))
    }
}

impl TryFrom<(&str, &str, &str)> for ClawMachine {
    type Error = ParseInputError;

    fn try_from((line_a, line_b, line_prize): (&str, &str, &str)) -> Result<Self, Self::Error> {
        let (dx, dy) = line_a
            .strip_prefix("Button A: X+")
            .ok_or(ParseInputError::MissingDelimiter)?
            .split_once(", Y+")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let button_a = (dx.parse()?, dy.parse()?);
        let (dx, dy) = line_b
            .strip_prefix("Button B: X+")
            .ok_or(ParseInputError::MissingDelimiter)?
            .split_once(", Y+")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let button_b = (dx.parse()?, dy.parse()?);
        let (x, y) = line_prize
            .strip_prefix("Prize: X=")
            .ok_or(ParseInputError::MissingDelimiter)?
            .split_once(", Y=")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let prize = (x.parse()?, y.parse()?);
        Ok(Self {
            button_a,
            button_b,
            prize,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    claw_machines: Vec<ClawMachine>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid integer: {0}")]
    InvalidInteger(#[from] std::num::ParseIntError),
    #[error("Extra input")]
    ExtraInput,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut claw_machines = Vec::new();
        let mut lines = text.lines();
        if let Some(line_a) = lines.next() {
            let line_b = lines.next().ok_or(ParseInputError::EmptyInput)?;
            let line_prize = lines.next().ok_or(ParseInputError::EmptyInput)?;
            claw_machines.push(ClawMachine::try_from((line_a, line_b, line_prize))?);
        }
        while let Some(empty) = lines.next() {
            if !empty.is_empty() {
                return Err(ParseInputError::ExtraInput);
            }
            let line_a = lines.next().ok_or(ParseInputError::EmptyInput)?;
            let line_b = lines.next().ok_or(ParseInputError::EmptyInput)?;
            let line_prize = lines.next().ok_or(ParseInputError::EmptyInput)?;
            claw_machines.push(ClawMachine::try_from((line_a, line_b, line_prize))?);
        }
        Ok(Self { claw_machines })
    }
}
