use std::fmt::{Display, Write};
use std::str::FromStr;
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

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example 2");
    println!("|+-Part 1: {} (expected 5,7,3,0)", part_1(&example2));
    println!("|+-Part 2: {} (expected 117_440)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1,5,0,5,2,0,1,3,5)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected 236_581_108_670_061)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn part_1(input: &Input) -> String {
    let mut state = input.initial_state;
    let output = state.execute_loop(&input.instructions);
    let mut result = String::new();
    for &value in &output.to_vec() {
        if !result.is_empty() {
            result.push(',');
        }
        write!(&mut result, "{value}").expect("Write value");
    }
    result
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    fn inner(
        original_a: u64,
        target: &[u8],
        instructions: &[Instruction],
        index: usize,
    ) -> Option<u64> {
        for digit in 0..=7 {
            let candidate_a = (original_a << 3) | digit;
            if candidate_a == 0 {
                continue;
            }
            let output_digit = State::new(candidate_a, 0, 0).execute_one_cycle(instructions);
            if u64::from(target[index]) == output_digit {
                if index == 0 {
                    return Some(candidate_a);
                }
                if let Some(result) = inner(candidate_a, target, instructions, index - 1) {
                    return Some(result);
                }
            }
        }
        None
    }
    inner(
        0,
        &input.target,
        &input.instructions,
        input.target.len() - 1,
    )
    .expect("no solution found")
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

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{value}"),
            Self::RegisterA => write!(f, "a"),
            Self::RegisterB => write!(f, "b"),
            Self::RegisterC => write!(f, "c"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    /// Shift register A right by argument number of bits
    Adv(Operand),
    /// Xor register B with argument
    Bxl(u8),
    /// Stores the value of arugment in register B, modulo 8
    Bst(Operand),
    /// Jumps to the instruction at the offset of the argument if register A is non-zero
    Jnz(u8),
    /// Xor register B with register C, consuming and ignoring argument
    Bxc(#[allow(unused)] u8),
    /// Outputs the value of argument, modulo 8
    Out(Operand),
    /// Shift register B left by argument number of bits, but stores in register A
    Bdv(Operand),
    /// Shift register C left by argument number of bits, but stores in register A
    Cdv(Operand),
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = ParseInputError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Adv(value.1.try_into()?)),
            1 => Ok(Self::Bxl(value.1)),
            2 => Ok(Self::Bst(value.1.try_into()?)),
            3 => Ok(Self::Jnz(value.1)),
            4 => Ok(Self::Bxc(value.1)),
            5 => Ok(Self::Out(value.1.try_into()?)),
            6 => Ok(Self::Bdv(value.1.try_into()?)),
            7 => Ok(Self::Cdv(value.1.try_into()?)),
            _ => Err(ParseInputError::InvalidInstruction(value.0)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
}

impl State {
    const fn new(register_a: u64, register_b: u64, register_c: u64) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
        }
    }

    fn get_value(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Literal(value) => u64::from(value),
            Operand::RegisterA => self.register_a,
            Operand::RegisterB => self.register_b,
            Operand::RegisterC => self.register_c,
        }
    }

    fn step(&mut self, instruction: Instruction) -> Option<u64> {
        let mut out = None;
        match instruction {
            Instruction::Adv(op) => {
                let value = self.get_value(op);
                self.register_a >>= value;
            }
            Instruction::Bxl(lit) => {
                self.register_b ^= u64::from(lit);
            }
            Instruction::Bst(op) => {
                let value = self.get_value(op);
                self.register_b = value % 8;
            }
            Instruction::Jnz(lit) => {
                if self.register_a != 0 {
                    self.instruction_pointer = lit as usize >> 1;
                    return None;
                }
            }
            Instruction::Bxc(_) => {
                self.register_b ^= self.register_c;
            }
            Instruction::Out(op) => {
                let value = self.get_value(op);
                out = Some(value & 7);
            }
            Instruction::Bdv(op) => {
                let value = self.get_value(op);
                self.register_b = self.register_a >> value;
            }
            Instruction::Cdv(op) => {
                let value = self.get_value(op);
                self.register_c = self.register_a >> value;
            }
        }
        self.instruction_pointer += 1;
        out
    }

    fn execute_one_cycle(&mut self, instructions: &[Instruction]) -> u64 {
        let mut output = 0;
        // Optimization: Last instruction is always JNZ 0, and that is the only
        // jump instruction. So we can run the instuctions in order, ignoring
        // the instruction pointer.
        for &instr in instructions {
            if let Some(out) = self.step(instr) {
                output = out;
            }
        }
        output
    }

    fn execute_loop(&mut self, instructions: &[Instruction]) -> Output {
        let mut output = 0;
        let mut shift = 0;
        // Follow the instruction pointer until it goes out of bounds
        while let Some(&instr) = instructions.get(self.instruction_pointer) {
            if let Some(out) = self.step(instr) {
                output |= out << shift;
                shift += 3;
            }
        }
        Output::new(output, shift)
    }
}

#[derive(Debug, Clone, Copy)]
struct Output {
    value: u64,
    shift: u64,
}

impl Output {
    const fn new(value: u64, shift: u64) -> Self {
        Self { value, shift }
    }

    fn to_vec(self) -> Vec<u8> {
        let mut result = Vec::new();
        let mut value = self.value;
        let mut shift = self.shift;
        while shift > 0 {
            result.push((value & 7) as u8);
            value >>= 3;
            shift -= 3;
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    target: Vec<u8>,
    instructions: Vec<Instruction>,
    initial_state: State,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is incomplete")]
    IncompleteInput,
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid value: {0}")]
    InvalidValue(#[from] std::num::ParseIntError),
    #[error("Invalid operand: {0}")]
    InvalidOperand(u8),
    #[error("Invalid instruction: {0}")]
    InvalidInstruction(u8),
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
        let target = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .strip_prefix("Program: ")
            .ok_or(ParseInputError::MissingDelimiter)?
            .as_bytes()
            .chunks(2)
            .map(|instr| instr[0] - b'0')
            .collect::<Vec<_>>();
        let instructions = target
            .chunks_exact(2)
            .map(|xs| (xs[0], xs[1]).try_into())
            .collect::<Result<Vec<_>, _>>()?;
        let initial_state = State {
            register_a,
            register_b,
            register_c,
            instruction_pointer: 0,
        };
        Ok(Self {
            target,
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
        assert_eq!(part_1(&input), "5,7,3,0");
    }

    #[test]
    fn example_1() {
        let input: Input = "Register A: 0\nRegister B: 0\nRegister C: 9\n\nProgram: 2 6"
            .parse()
            .expect("Parse example");
        let mut state = input.initial_state;
        let output = state.execute_loop(&input.instructions).to_vec();
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 1);
    }

    #[test]
    fn example_2() {
        let input: Input = "Register A: 10\nRegister B: 0\nRegister C: 0\n\nProgram: 5 0 5 1 5 4"
            .parse()
            .expect("Parse example");
        let mut state = input.initial_state;
        let output = state.execute_loop(&input.instructions).to_vec();
        assert_eq!(output, &[0, 1, 2]);
    }

    #[test]
    fn example_3() {
        let input: Input = "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0 1 5 4 3 0"
            .parse()
            .expect("Parse example");
        let mut state = input.initial_state;
        let output = state.execute_loop(&input.instructions).to_vec();
        assert_eq!(output, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.register_a, 0);
    }

    #[test]
    fn example_4() {
        let input: Input = "Register A: 0\nRegister B: 29\nRegister C: 0\n\nProgram: 1 7"
            .parse()
            .expect("Parse example");
        let mut state = input.initial_state;
        let output = state.execute_loop(&input.instructions).to_vec();
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 26);
    }

    #[test]
    fn example_5() {
        let input: Input = "Register A: 0\nRegister B: 2024\nRegister C: 43690\n\nProgram: 4 0"
            .parse()
            .expect("Parse example");
        let mut state = input.initial_state;
        let output = state.execute_loop(&input.instructions).to_vec();
        assert_eq!(output, &[]);
        assert_eq!(state.register_b, 44354);
    }
}
