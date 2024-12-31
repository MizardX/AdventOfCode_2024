use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 24");

    println!("++Example 1");
    let example1 = EXAMPLE1.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 4)", part_1(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.try_into().expect("Parse example");
    println!("|+-Part 1: {} (expected 2_024)", part_1(&example2));

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
pub fn part_1(input: &Circuit) -> u64 {
    let mut runner = CircuitRunner::new(input);
    runner.run();
    runner.get_output().unwrap()
}

#[must_use]
pub fn part_2(input: &Circuit) -> String {
    let wrongs = input.validate_circuit();

    assert_eq!(
        wrongs.len(),
        8,
        "Expected 8 wrong gates that where missplaced, found {}",
        wrongs.len()
    );

    let mut names: Vec<_> = wrongs
        .into_iter()
        .map(|gate_index| input.gates[gate_index].name)
        .collect();
    names.sort_unstable();
    names.join(",")
}

#[derive(Debug, Clone)]
struct CircuitRunner<'a> {
    circuit: &'a Circuit<'a>,
    values: Vec<Option<bool>>,
}

impl<'a> CircuitRunner<'a> {
    fn new(circuit: &'a Circuit<'a>) -> Self {
        let n = circuit.gates.len();
        let values = vec![None; n];
        Self { circuit, values }
    }

    fn run(&mut self) {
        let n = self.values.len();
        let mut waiting_on = vec![vec![]; n];
        let mut pending: VecDeque<_> = (0..n).collect();
        while let Some(gate_index) = pending.pop_front() {
            if self.values[gate_index].is_none() {
                match self.evaluate_gate(gate_index) {
                    Ok(value) => {
                        self.values[gate_index] = Some(value);
                        for next in waiting_on[gate_index].drain(..) {
                            pending.push_back(next);
                        }
                    }
                    Err(missing) => waiting_on[missing].push(gate_index),
                }
            }
        }
    }

    fn evaluate_gate(&self, gate: usize) -> Result<bool, usize> {
        Ok(match self.circuit.gates[gate].kind {
            GateKind::None => unreachable!(),
            GateKind::Constant(value) => value,
            GateKind::Binary(binop, lhs, rhs) => {
                let lhs_value = self.values[lhs].ok_or(lhs)?;
                let rhs_value = self.values[rhs].ok_or(rhs)?;
                match binop {
                    BinOp::And => lhs_value && rhs_value,
                    BinOp::Or => lhs_value || rhs_value,
                    BinOp::Xor => lhs_value ^ rhs_value,
                }
            }
        })
    }

    fn get_output(&self) -> Option<u64> {
        let mut output = 0;
        for (shift, &index) in self.circuit.output_gates.iter().enumerate() {
            output |= u64::from(self.values[index]?) << shift;
        }
        Some(output)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BinOp {
    And,
    Or,
    Xor,
}

impl FromStr for BinOp {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(ParseInputError::SyntaxError),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum GateKind {
    #[default]
    None,
    Binary(BinOp, usize, usize),
    Constant(bool),
}

impl GateKind {
    const fn is_constant(self) -> bool {
        matches!(self, Self::Constant(_))
    }

    const fn has_operand(self, operand: usize) -> bool {
        match self {
            Self::Binary(_, lhs, rhs) => lhs == operand || rhs == operand,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    name: &'a str,
    kind: GateKind,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Circuit<'a> {
    gates: Vec<Gate<'a>>,
    input_gates1: Vec<usize>,
    input_gates2: Vec<usize>,
    output_gates: Vec<usize>,
}

impl Circuit<'_> {
    fn validate_circuit(&self) -> Vec<usize> {
        let mut wrongs = Vec::new();

        // The XOR directly connected to the input. The partial sum.
        let mut input_adds = Vec::new();
        // The AND directly connected to the input. A partial carry.
        let mut input_carries = Vec::new();
        // The XOR that is the output. The full sum.
        let mut full_adds = Vec::new();
        // The AND that is connected to the incoming carry. A partial carry, or "carry-carry".
        let mut inner_carries = Vec::new();
        // The OR that combines the full carries. The final carry.
        let mut full_carries = Vec::new();

        for (index, gate) in self.gates.iter().enumerate() {
            if let GateKind::Binary(bin_op, lhs, rhs) = gate.kind {
                let is_input =
                    self.gates[lhs].kind.is_constant() || self.gates[rhs].kind.is_constant();
                match (bin_op, is_input) {
                    (BinOp::And, true) => input_carries.push(index),
                    (BinOp::And, false) => inner_carries.push(index),
                    (BinOp::Or, true) => {
                        panic!("Inputs should not have been swapped to an OR gate")
                    }
                    (BinOp::Or, false) => full_carries.push(index),
                    (BinOp::Xor, true) => input_adds.push(index),
                    (BinOp::Xor, false) => full_adds.push(index),
                }
            }
        }

        let first_input1 = self.input_gates1.first().copied().unwrap();
        let last_output = self.output_gates.last().copied().unwrap();

        // If the XOR gate either doesn't show up in other XOR operations, or is an output, then it is wrong.
        for &gate in &input_adds {
            let has_first_input_operand = self.gates[gate].kind.has_operand(first_input1);
            let is_not_output_gate = !self.output_gates.contains(&gate);
            let is_not_in_full_adds = full_adds
                .iter()
                .all(|&a| !self.gates[a].kind.has_operand(gate));

            if !has_first_input_operand && is_not_output_gate && is_not_in_full_adds {
                wrongs.push(gate);
                // println!("input add XOR gate {} is wrong, has_first_input_operand={has_first_input_operand}, is_not_output_gate={is_not_output_gate}, is_not_in_full_adds={is_not_in_full_adds}", self.gates[gate].name);
            }
        }

        // If the AND gate either doesn't show up in full carries, or is an output, then it is wrong.
        for &gate in &input_carries {
            let has_first_input_operand = self.gates[gate].kind.has_operand(first_input1);
            let is_output_gate = self.output_gates.contains(&gate);
            let is_not_in_full_carries = full_carries
                .iter()
                .all(|&a| !self.gates[a].kind.has_operand(gate));

            if !has_first_input_operand && (is_output_gate || is_not_in_full_carries) {
                wrongs.push(gate);
                // println!("input carry AND gate {} is wrong, has_first_input_operand={has_first_input_operand}, is_output_gate={is_output_gate}, is_not_in_full_carries={is_not_in_full_carries}", self.gates[gate].name);
            }
        }

        // If the XOR gate isn't an output, then it is wrong
        for &gate in &full_adds {
            let is_output_gate = self.output_gates.contains(&gate);
            if !is_output_gate {
                wrongs.push(gate);
                // println!(
                //     "full add XOR gate {} is wrong, is_output_gate={is_output_gate}",
                //     self.gates[gate].name
                // );
            }
        }

        // If the OR gate is an output, except for the last output, which doubles as a carry, then it is wrong.
        for &gate in &full_carries {
            let is_output_gate = self.output_gates.contains(&gate);
            let is_last_output_gate = gate == last_output;
            if is_output_gate && !is_last_output_gate {
                wrongs.push(gate);
                // println!("full carry OR gate {} is wrong, is_output_gate={is_output_gate}, is_last_output_gate={is_last_output_gate}", self.gates[gate].name);
            }
        }

        // If the AND gate is an output, then it is wrong.
        for &gate in &inner_carries {
            let is_output_gate = self.output_gates.contains(&gate);
            if is_output_gate {
                wrongs.push(gate);
                // println!(
                //     "inner carry AND gate {} is wrong, is_output_gate={is_output_gate}",
                //     self.gates[gate].name
                // );
            }
        }

        wrongs
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Syntax error")]
    SyntaxError,
}

impl<'a> TryFrom<&'a str> for Circuit<'a> {
    type Error = ParseInputError;

    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        let n = text.lines().filter(|line| !line.is_empty()).count();
        let mut gates = Vec::with_capacity(n);
        let mut lookup = HashMap::with_capacity(n);
        let mut lines = text.lines();
        parse_constants(&mut gates, &mut lookup, &mut lines)?;
        parse_gates(&mut gates, &mut lookup, lines)?;
        let (input_gates1, input_gates2, output_gates) = extract_inputs_and_outputs(&gates);

        Ok(Circuit {
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
    for line in lines {
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
        let (expr, output) = line
            .split_once(" -> ")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let (left, rest) = expr.split_once(' ').ok_or(ParseInputError::MissingDelimiter)?;
        let (binop, right) = rest.split_once(' ').ok_or(ParseInputError::MissingDelimiter)?;

        let left_gate = get_or_add_gate(gates, lookup, left);
        let right_gate = get_or_add_gate(gates, lookup, right);
        let output_gate = get_or_add_gate(gates, lookup, output);

        gates[output_gate].kind = GateKind::Binary(binop.parse()?, left_gate, right_gate);
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
