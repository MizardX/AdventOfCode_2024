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
    let mut wrongs = Vec::new();

    // The XOR directly connected to the input. The partial sum.
    let mut input_adds = Vec::new();
    // The AND directly connected to the input. A partial carry.
    let mut input_carries = Vec::new();
    // The XOR that is the output. The full sum.
    let mut full_adds = Vec::new();
    // The AND that is connected to the incoming carry. A partial carry, or "carry-carry".
    let mut inner_carry = Vec::new();
    // The OR that combines the full carries. The final carry.
    let mut full_carries = Vec::new();

    for (index, gate) in input.gates.iter().enumerate() {
        if let GateKind::Binary(bin_op, lhs, rhs) = gate.kind {
            let is_input = input.gates[lhs].kind.is_constant() || input.gates[rhs].kind.is_constant();
            match (bin_op, is_input) {
                (BinOp::And, true) => input_carries.push(index),
                (BinOp::And, false) => inner_carry.push(index),
                (BinOp::Or, true) => panic!("Inputs should not have been swapped to an OR gate"),
                (BinOp::Or, false) => full_carries.push(index),
                (BinOp::Xor, true) => input_adds.push(index),
                (BinOp::Xor, false) => full_adds.push(index),
            }
        }
    }

    let first_input1 = input.input_gates1.first().copied().unwrap();
    let last_output = input.output_gates.last().copied().unwrap();

    // If the XOR gate either doesn't show up in other XOR operations, or is an output, then it is wrong.
    for &gate in &input_adds {
        let has_first_input_operand = input.gates[gate].kind.has_operand(first_input1);
        let is_not_output_gate = !input.output_gates.contains(&gate);
        let is_not_in_full_adds = full_adds.iter().all(|&a| !input.gates[a].kind.has_operand(gate));

        if !has_first_input_operand && is_not_output_gate && is_not_in_full_adds {
            wrongs.push(gate);
        }
    }

    // If the AND gate either doesn't show up in full carries, or is an output, then it is wrong.
    for &gate in &input_carries {
        let has_first_input_operand = input.gates[gate].kind.has_operand(first_input1);
        let is_output_gate = input.output_gates.contains(&gate);
        let is_not_in_full_carries = full_carries
            .iter()
            .all(|&a| !input.gates[a].kind.has_operand(gate));

        if !has_first_input_operand && (is_output_gate || is_not_in_full_carries) {
            wrongs.push(gate);
        }
    }

    // If the XOR gate isn't an output, then it is wrong
    for &gate in &full_adds {
        let is_output_gate = input.output_gates.contains(&gate);
        if !is_output_gate {
            wrongs.push(gate);
        }
    }

    // If the OR gate is an output, except for the last output, which doubles as a carry, then it is wrong.
    for &gate in &full_carries {
        let is_output_gate = input.output_gates.contains(&gate);
        let is_last_output_gate = gate == last_output;
        if is_output_gate && !is_last_output_gate {
            wrongs.push(gate);
        }
    }

    // If the AND gate is an output, then it is wrong.
    for &gate in &inner_carry {
        let is_output_gate = input.output_gates.contains(&gate);
        if is_output_gate {
            wrongs.push(gate);
        }
    }

    assert_eq!(wrongs.len(), 8, "Expected 8 wrong gates that where missplaced, found {}", wrongs.len());

    let mut names: Vec<_> = wrongs.into_iter().map(|s| input.gates[s].name).collect();
    names.sort_unstable();
    names.join(",")
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
#[allow(unused)]
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
