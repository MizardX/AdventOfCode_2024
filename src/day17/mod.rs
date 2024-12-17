use std::{
    fmt::{Display, Write},
    str::FromStr,
};
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 17");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example 1");
    println!(
        "|+-Part 1: {} (expected 4,6,3,5,6,3,5,2,1,0)",
        part_1(&example1)
    );
    //println!("|'-Part 2: {} (expected XXX)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 5,7,3,0)", part_1(&example2));
    // println!("|+-Part 2: {} (expected 117_440)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1,5,0,5,2,0,1,3,5)", part_1(&input));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn part_1(input: &Input) -> String {
    let mut state = input.initial_state;
    let output = state.execute(&input.instructions);
    let mut result = String::new();
    for &value in &output {
        if !result.is_empty() {
            result.push(',');
        }
        write!(&mut result, "{value}").expect("Write value");
    }
    result
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let n = input.instructions.len();
    println!("n={n}");
    part2_solve(input, 0, n).expect("No solution found")
}

fn part2_solve(input: &Input, register_a: u64, position: usize) -> Option<u64> {
    if position == 0 {
        let mut state = State {
            register_a,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
        };
        let result = state.execute(&input.instructions);
        if result == input.instructions {
            return Some(register_a);
        }
        println!("failed at the end: {result:?} vs {:?}", &input.instructions);
        return None;
    }
    for digit in 0..=7 {
        let mut state = State {
            register_a: register_a + (digit << (position * 3)),
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
        };
        let result = state.execute(&input.instructions);
        if result.len() < position {
            // println!(
            //     "[{:#o}] {:?} (too short) vs {:?} | {:?}",
            //     register_a + (digit << (position * 3)),
            //     &result,
            //     &input.instructions[..position-1],
            //     &input.instructions[position-1..]
            // );
            continue;
        }
        // println!(
        //     "[{:#o}] {:?} | {:?} vs {:?} | {:?}",
        //     register_a + (digit << (position * 3)),
        //     &result[..position],
        //     &result[position..],
        //     &input.instructions[..position-1],
        //     &input.instructions[position-1..]
        // );
        if result.is_empty() || result[position..] == input.instructions[position-1..] {
            if let Some(solution) =
                part2_solve(input, register_a + (digit << (position * 3)), position - 1)
            {
                return Some(solution);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(u8),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl TryFrom<u8> for Operand {
    type Error = ExecutionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => Ok(Self::Literal(value)),
            4 => Ok(Self::RegisterA),
            5 => Ok(Self::RegisterB),
            6 => Ok(Self::RegisterC),
            _ => Err(ExecutionError::InvalidOperand(value)),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{value}"),
            Self::RegisterA => write!(f, "A"),
            Self::RegisterB => write!(f, "B"),
            Self::RegisterC => write!(f, "C"),
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
    type Error = ExecutionError;

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
            _ => Err(ExecutionError::InvalidInstruction(value)),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adv => write!(f, "a-divide"),
            Self::Bxl => write!(f, "b-xor-literal"),
            Self::Bst => write!(f, "b-store"),
            Self::Jnz => write!(f, "jump-nonzero"),
            Self::Bxc => write!(f, "b-xor-c"),
            Self::Out => write!(f, "output"),
            Self::Bdv => write!(f, "b-divide"),
            Self::Cdv => write!(f, "c-divide"),
        }
    }
}

#[derive(Debug, Clone, Error)]
#[expect(clippy::enum_variant_names)]
enum ExecutionError {
    #[error("Invalid instruction pointer: {0}")]
    InvalidInstructionPointer(usize),
    #[error("Invalid operand: {0}")]
    InvalidOperand(u8),
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u8),
}

#[derive(Debug, Clone, Copy)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
}

impl State {
    fn read_instruction(
        &mut self,
        instructions: &[u8],
    ) -> Result<(Instruction, Operand), ExecutionError> {
        let instruction: Instruction = instructions
            .get(self.instruction_pointer)
            .copied()
            .ok_or(ExecutionError::InvalidInstructionPointer(
                self.instruction_pointer,
            ))?
            .try_into()?;
        let operand_byte = instructions
            .get(self.instruction_pointer + 1)
            .copied()
            .ok_or(ExecutionError::InvalidInstructionPointer(
                self.instruction_pointer + 1,
            ))?;
        let operand = match instruction {
            Instruction::Bxl | Instruction::Jnz => Operand::Literal(operand_byte),
            _ => operand_byte.try_into()?,
        };
        self.instruction_pointer += 2;
        Ok((instruction, operand))
    }

    fn get_value(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Literal(value) => u64::from(value),
            Operand::RegisterA => self.register_a,
            Operand::RegisterB => self.register_b,
            Operand::RegisterC => self.register_c,
        }
    }

    fn execute(&mut self, instructions: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        loop {
            // let ip = self.instruction_pointer;
            // let a = self.register_a;
            // let b = self.register_b;
            // let c = self.register_c;
            // let in_byte = instructions.get(ip).copied().unwrap_or(0);
            // let op_byte = instructions.get(ip + 1).copied().unwrap_or(0);
            match self.read_instruction(instructions) {
                Err(ExecutionError::InvalidInstructionPointer(_ip)) => {
                    // println!("End of program at instruction pointer: {ip}");
                    break;
                }
                Err(error) => panic!("Error: {error}"),
                Ok((instruction, operand)) => {
                    let value = self.get_value(operand);
                    // println!("[{ip:3o} A:{a:<9o} B:{b:<9o} C:{c:<9o}] {in_byte} {op_byte} --> {instruction} {operand} ({value:o})");
                    match instruction {
                        Instruction::Adv => {
                            self.register_a >>= value;
                        }
                        Instruction::Bxl => {
                            self.register_b ^= value;
                        }
                        Instruction::Bst => {
                            self.register_b = value % 8;
                        }
                        Instruction::Jnz =>
                        {
                            #[allow(clippy::cast_possible_truncation)]
                            if self.register_a != 0 {
                                self.instruction_pointer = value as usize;
                            }
                        }
                        Instruction::Bxc => {
                            self.register_b ^= self.register_c;
                        }
                        Instruction::Out => {
                            output.push((value % 8) as u8);
                        }
                        Instruction::Bdv => {
                            self.register_b = self.register_a >> value;
                        }
                        Instruction::Cdv => {
                            self.register_c = self.register_a >> value;
                        }
                    }
                }
            }
        }
        output
    }
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
    #[error("Invalid value: {0}")]
    InvalidValue(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let register_a = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Register A: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .parse()?;
        let register_b = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Register B: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .parse()?;
        let register_c = lines
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let input = EXAMPLE1.parse().expect("Parse example");
        assert_eq!(part_1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part_1_example_2() {
        let input = EXAMPLE2.parse().expect("Parse example");
        assert_eq!(part_1(&input), "0,3,5,4,3,0");
    }

    #[test]
    fn example_1() {
        let mut state = State {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
        };
        let instructions = &[2, 6];
        let output = state.execute(instructions);
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 1);
    }

    #[test]
    fn example_2() {
        let mut state = State {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
        };
        let instructions = &[5, 0, 5, 1, 5, 4];
        let output = state.execute(instructions);
        assert_eq!(output, &[0, 1, 2]);
    }

    #[test]
    fn example_3() {
        let mut state = State {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
        };
        let instructions = &[0, 1, 5, 4, 3, 0];
        let output = state.execute(instructions);
        assert_eq!(output, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.register_a, 0);
    }

    #[test]
    fn example_4() {
        let mut state = State {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
        };
        let instructions = &[1, 7];
        let output = state.execute(instructions);
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 26);
    }

    #[test]
    fn example_5() {
        let mut state = State {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
        };
        let instructions = &[4, 0];
        let output = state.execute(instructions);
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 44354);
    }
}
