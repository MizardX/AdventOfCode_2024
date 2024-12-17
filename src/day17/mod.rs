use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 17");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected XXX)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let _ = input;
    todo!()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl TryFrom<u8> for Operand {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => Ok(Self::Literal(value)),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::RegisterC),
            _ => Err(ParseInputError::InvalidOperand(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    /// Shift register A right by argument number of bits
    Adv,
    /// Xor register B with argument
    Bxl,
    /// Stores the value of arugment in register B, modulo 8
    Bst,
    /// Jumps to the instruction at the offset of the argument if register A is non-zero
    Jnz,
    /// Xor register B with register C, consuming and ignoring argument
    Bxc,
    /// Outputs the value of argument, modulo 8
    Out,
    /// Shift register B left by argument number of bits, but stores in register A
    Bdv,
    /// Shift register C left by argument number of bits, but stores in register A
    Cdv,
}

impl TryFrom<u8> for Instruction {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Adv),
            1 => Ok(Self::Bxl),
            2 => Ok(Self::Bst),
            3 => Ok(Self::Jnz),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out),
            6 => Ok(Self::Bdv),
            7 => Ok(Self::Cdv),
            _ => Err(ParseInputError::InvalidInstruction(value)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    register_a: u8,
    register_b: u8,
    register_c: u8,
    instruction_pointer: usize,
}

#[derive(Debug, Clone)]
pub struct Input {
    instructions: Vec<u8>,
    initial_state: State,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is incomplete")]
    IncompleteInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid operand: {0}")]
    InvalidOperand(u8),
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u8),
    #[error("Invalid value: {0}")]
    InvalidValue(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let register_a: u8 = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Register A: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .parse()?;
        let register_b: u8 = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Register B: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .parse()?;
        let register_c: u8 = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Register C: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .parse()?;
        if !lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .is_empty()
        {
            return Err(ParseInputError::MissingDelimiter);
        }
        let instructions = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Program: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let initial_state = State {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
        };
        Ok(Self {
            instructions,
            initial_state,
        })
    }
}
