use std::collections::{HashMap, VecDeque};
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 24");

    println!("++Example 1");
    let example1 = EXAMPLE1.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 4)", part_1(&example1));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 2_024)", part_1(&example2));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&example2));

    println!("++Input");
    let input = INPUT.try_into().expect("Parse input");
    println!(
        "|+-Part 1: {} (expected 41_324_968_993_486)",
        part_1(&input)
    );
    println!(
        "|'-Part 2: {} (expected bmn,jss,mvb,rds,wss,z08,z18,z23)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    let n = input.gates.len();
    let mut values = vec![None; n];
    run_circuit(&input.gates, &mut values);
    extract_output(input, &values).unwrap()
}

fn run_circuit(gates: &[Gate<'_>], values: &mut [Option<bool>]) {
    let n = gates.len();
    let mut waiting_on = vec![vec![]; n];
    let mut pending: VecDeque<_> = (0..n).collect();
    while let Some(index) = pending.pop_front() {
        if values[index].is_none() {
            let gate = &gates[index];
            match gate.kind.evaluate(values) {
                Ok(value) => {
                    values[index] = Some(value);
                    for next in waiting_on[index].drain(..) {
                        pending.push_back(next);
                    }
                }
                Err(other) => waiting_on[other].push(index),
            }
        }
    }
}

fn extract_output(input: &Input<'_>, values: &[Option<bool>]) -> Option<u64> {
    let mut output = 0;
    for (shift, &index) in input.output_gates.iter().enumerate() {
        let value: bool = values[index]?;
        output |= u64::from(value) << shift;
    }
    Some(output)
}

#[must_use]
pub fn part_2(input: &Input) -> String {
    let input_size = input.input_gates1.len();
    let output_size = input.output_gates.len();
    assert_eq!(input_size, input.input_gates2.len());
    assert_eq!(output_size, input_size + 1);
    let num_or_gates = input.gates.iter().filter(|gate| gate.kind.is_or()).count();
    assert_eq!(num_or_gates, input_size - 1);
    assert!(input.gates[*input.output_gates.last().unwrap()]
        .kind
        .is_or());
    let mut gates = input.gates.clone();
    let mut all_swaps = Vec::new();
    println!("Brute forcing! Will take ~90 minutes...");
    for _ in 0..4 {
        let mut best_score = u32::MAX;
        let mut best_swap = None;
        for i in 0..input.gates.len() {
            for j in i + 1..input.gates.len() {
                gates.swap(i, j);
                let score = evaluate_circuit(input, &gates);
                if score < best_score {
                    best_score = score;
                    best_swap = Some((i, j));
                }
                gates.swap(i, j);
            }
        }
        gates.swap(best_swap.unwrap().0, best_swap.unwrap().1);
        all_swaps.push(best_swap.unwrap());
    }
    let mut names = Vec::new();
    for &(i, j) in all_swaps.iter().rev() {
        names.push(input.gates[i].name);
        names.push(input.gates[j].name);
        gates.swap(i, j);
    }
    names.sort_unstable();
    names.join(",")
}

fn evaluate_circuit(input: &Input, gates: &[Gate<'_>]) -> u32 {
    let mut score = 0;
    let mut values = vec![None; input.gates.len()];
    for bit in 0..input.input_gates1.len() {
        // x = 0, y = 1, carry = 0
        for &(x, y, carry) in &[
            (false, true, false),
            (true, false, false),
            (true, true, false),
            (false, false, true),
            (true, false, true),
            (false, true, true),
            (true, true, true),
        ] {
            if bit == 0 && carry {
                continue;
            }
            let Some(s) = evaluate_case(input, gates, &mut values, bit, x, y, carry) else {
                return u32::MAX;
            };
            score += s;
        }
    }
    score
}

fn evaluate_case(
    input: &Input<'_>,
    gates: &[Gate<'_>],
    values: &mut [Option<bool>],
    bit: usize,
    x: bool,
    y: bool,
    carry: bool,
) -> Option<u32> {
    for v in values.iter_mut() {
        *v = None;
    }
    for i in 0..input.input_gates1.len() {
        values[input.input_gates1[i]] = Some(x && (bit == i) || carry && (bit - 1 == i));
        values[input.input_gates2[i]] = Some(y && (bit == i) || carry && (bit - 1 == i));
    }
    run_circuit(gates, values);
    let output = extract_output(input, values)?;
    let carry = u64::from(carry) << bit;
    let x = u64::from(x) << bit;
    let y = u64::from(y) << bit;
    Some((output ^ (x + y + carry)).count_ones())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BinOp {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum GateKind {
    #[default]
    None,
    Binary(BinOp, usize, usize),
    Constant(bool),
}

#[expect(dead_code)]
impl GateKind {
    fn is_or(self) -> bool {
        matches!(self, GateKind::Binary(BinOp::Or, _, _))
    }
    fn is_and(self) -> bool {
        matches!(self, GateKind::Binary(BinOp::And, _, _))
    }
    fn is_xor(self) -> bool {
        matches!(self, GateKind::Binary(BinOp::Xor, _, _))
    }
    fn is_constant(self) -> bool {
        matches!(self, GateKind::Constant(_))
    }

    fn is_binary(self) -> bool {
        matches!(self, GateKind::Binary(_, _, _))
    }

    fn has_operand(self, operand: usize) -> bool {
        match self {
            GateKind::Binary(_, lhs, rhs) => lhs == operand || rhs == operand,
            _ => false,
        }
    }

    fn other_operand(self, first_operand: usize) -> Option<usize> {
        let (lhs, rhs) = self.as_binary()?;
        Some(if lhs == first_operand {
            rhs
        } else if rhs == first_operand {
            lhs
        } else {
            None?
        })
    }

    fn children(self) -> Option<[usize; 2]> {
        self.as_binary().map(|(a, b)| [a, b])
    }

    fn as_binary(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Binary(_, lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_and(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Binary(BinOp::And, lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_or(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Binary(BinOp::Or, lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_xor(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Binary(BinOp::Xor, lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }

    fn flip(self) -> Option<Self> {
        Some(match self {
            GateKind::Binary(BinOp::And, lhs, rhs) => GateKind::Binary(BinOp::Or, lhs, rhs),
            GateKind::Binary(BinOp::Or, lhs, rhs) => GateKind::Binary(BinOp::And, lhs, rhs),
            GateKind::Binary(BinOp::Xor, lhs, rhs) => GateKind::Binary(BinOp::Xor, lhs, rhs),
            _ => None?,
        })
    }

    fn evaluate(self, inputs: &[Option<bool>]) -> Result<bool, usize> {
        Ok(match self {
            GateKind::None => unreachable!(),
            GateKind::Constant(value) => value,
            GateKind::Binary(BinOp::And, lhs, rhs) => {
                let lhs_value = inputs[lhs].ok_or(lhs)?;
                let rhs_value = inputs[rhs].ok_or(rhs)?;
                lhs_value && rhs_value
            }
            GateKind::Binary(BinOp::Or, lhs, rhs) => {
                let lhs_value = inputs[lhs].ok_or(lhs)?;
                let rhs_value = inputs[rhs].ok_or(rhs)?;
                lhs_value || rhs_value
            }
            GateKind::Binary(BinOp::Xor, lhs, rhs) => {
                let lhs_value = inputs[lhs].ok_or(lhs)?;
                let rhs_value = inputs[rhs].ok_or(rhs)?;
                lhs_value ^ rhs_value
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    name: &'a str,
    kind: GateKind,
}

#[derive(Debug, Clone)]
pub struct Input<'a> {
    gates: Vec<Gate<'a>>,
    input_gates1: Vec<usize>,
    input_gates2: Vec<usize>,
    output_gates: Vec<usize>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Syntax error")]
    SyntaxError,
}

impl<'a> TryFrom<&'a str> for Input<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let n = text.lines().filter(|line| !line.is_empty()).count();
        let mut gates = Vec::with_capacity(n);
        let mut lookup = HashMap::with_capacity(n);
        let mut lines = text.lines();
        parse_constants(&mut gates, &mut lookup, &mut lines)?;
        parse_gates(&mut gates, &mut lookup, lines)?;
        let (input_gates1, input_gates2, output_gates) = extract_inputs_and_outputs(&gates);

        Ok(Input {
            gates,
            input_gates1,
            input_gates2,
            output_gates,
        })
    }
}

fn parse_constants<'a>(
    gates: &mut Vec<Gate<'a>>,
    lookup: &mut HashMap<&'a str, usize>,
    lines: &mut std::str::Lines<'a>,
) -> Result<(), ParseInputError> {
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, value) = line
            .split_once(": ")
            .ok_or(ParseInputError::MissingDelimiter)?;
        lookup.insert(name, gates.len());
        gates.push(Gate {
            name,
            kind: GateKind::Constant(value == "1"),
        });
    }
    Ok(())
}

fn parse_gates<'a>(
    gates: &mut Vec<Gate<'a>>,
    lookup: &mut HashMap<&'a str, usize>,
    lines: std::str::Lines<'a>,
) -> Result<(), ParseInputError> {
    for line in lines {
        let (lhs, rhs) = line
            .split_once(" -> ")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let kind = if let Some((left, right)) = lhs.split_once(" AND ") {
            let left_gate = get_or_add_gate(gates, lookup, left);
            let right_gate = get_or_add_gate(gates, lookup, right);
            GateKind::Binary(BinOp::And, left_gate, right_gate)
        } else if let Some((left, right)) = lhs.split_once(" OR ") {
            let left_gate = get_or_add_gate(gates, lookup, left);
            let right_gate = get_or_add_gate(gates, lookup, right);
            GateKind::Binary(BinOp::Or, left_gate, right_gate)
        } else if let Some((left, right)) = lhs.split_once(" XOR ") {
            let left_gate = get_or_add_gate(gates, lookup, left);
            let right_gate = get_or_add_gate(gates, lookup, right);
            GateKind::Binary(BinOp::Xor, left_gate, right_gate)
        } else {
            return Err(ParseInputError::SyntaxError);
        };
        let gate = get_or_add_gate(gates, lookup, rhs);
        gates[gate].kind = kind;
    }
    Ok(())
}

fn get_or_add_gate<'a>(
    gates: &mut Vec<Gate<'a>>,
    lookup: &mut HashMap<&'a str, usize>,
    name: &'a str,
) -> usize {
    *lookup.entry(name).or_insert_with(|| {
        gates.push(Gate {
            name,
            kind: GateKind::None,
        });
        gates.len() - 1
    })
}

fn extract_inputs_and_outputs(gates: &[Gate<'_>]) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut input_gates1 = Vec::new();
    let mut input_gates2 = Vec::new();
    let mut output_gates = Vec::new();
    for (index, gate) in gates.iter().enumerate() {
        if gate.name.starts_with('x') {
            input_gates1.push(index);
        } else if gate.name.starts_with('y') {
            input_gates2.push(index);
        } else if gate.name.starts_with('z') {
            output_gates.push(index);
        }
    }
    output_gates.sort_unstable_by_key(|&index| gates[index].name);
    input_gates1.sort_unstable_by_key(|&index| gates[index].name);
    input_gates2.sort_unstable_by_key(|&index| gates[index].name);
    (input_gates1, input_gates2, output_gates)
}
