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
    // println!(
    //     "|'-Part 2: {} (expected bmn,jss,mvb,rds,wss,z08,z18,z23)",
    //     part_2(&input)
    // );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let n = input.gates.len();
    let mut values = vec![None; n];
    let mut waiting_on = vec![vec![]; n];
    let mut pending: VecDeque<_> = (0..n).collect();
    while let Some(index) = pending.pop_front() {
        let gate = &input.gates[index];
        match gate.kind.evaluate(&values) {
            Ok(value) => values[index] = Some(value),
            Err(other) => waiting_on[other].push(index),
        }
        for next in waiting_on[index].drain(..) {
            pending.push_back(next);
        }
    }
    let mut output = 0;
    for (shift, &index) in input.output_gates.iter().enumerate() {
        let value = values[index].expect("Missing output value");
        output |= usize::from(value) << shift;
    }
    output
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    // Inputs: x(0) .. x(n-1), y(0) .. y(n-1)
    // Outputs: z(0) .. z(n)
    // Carry: c(1) .. c(n)
    // A normal graph:
    // - First layer has
    //     x(0) XOR y(0) -> z(0)
    //     x(0) AND y(0) -> c(1)
    // - Middle layers has
    //     x(i) XOR y(i) -> p(i)
    //     x(i) AND y(i) -> q(i)
    //     c(i) XOR p(i) -> z(i)
    //     c(i) AND p(i) -> r(i)
    //     q(i) OR r(i) -> c(i+1)
    // - Last layer has
    //     c(n) -> z(n)
    // 1. Find p(i) and q(i) for all i
    // 3. Find z(i) for all i
    // 2. Find r(i) for all i
    // 4. Find c(i+1) for all i
    // 5. Find any gates that do not match the pattern
    let mut lookup_gate: HashMap<GateKind, usize> = HashMap::new();
    let mut lookup_binary: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut lookup_unary: HashMap<usize, Vec<usize>> = HashMap::new();
    for (index, gate) in input.gates.iter().enumerate() {
        lookup_gate.insert(gate.kind, index);
        if let Some((lhs, rhs)) = gate.kind.as_binary() {
            lookup_binary.entry((lhs, rhs)).or_default().push(index);
            lookup_unary.entry(lhs).or_default().push(index);
            lookup_unary.entry(rhs).or_default().push(index);
        }
        if let Some(flipped) = gate.kind.flip() {
            lookup_gate.insert(flipped, index);
            if let Some((lhs, rhs)) = flipped.as_binary() {
                lookup_binary.entry((lhs, rhs)).or_default().push(index);
            }
        }
    }

    let mut broken_gates = Vec::new();

    let carry_below = input.output_gates.last().copied().unwrap();
    for level in (1..input.output_gates.len() - 1).rev() {
        let x = input.input_gates1[level];
        let y = input.input_gates2[level];
        let z = input.output_gates[level];
        if input.gates[z].kind.is_xor() {
            broken_gates.push(z);
        }
        // The carry-below should be an OR gate of q and r
        // q should be an AND gate of x and y
        // r should be an AND gate of p and carry-above
        // p should be an XOR gate of x and y
        // z should be an XOR gate of p and carry-above
        // Otherwise add the gate to the broken gates.
        // From below we have the carry-below and z. From above we have x and y.
        let p_above = lookup_gate.get(&GateKind::Xor(x, y)).copied();
        let _q_above = lookup_gate.get(&GateKind::And(x, y)).copied();
        let (_r_above, _z_above) =
            if let Some(below_p) = p_above.and_then(|p_above| lookup_unary.get(&p_above)) {
                let r_above = below_p
                    .iter()
                    .find(|&&gate| input.gates[gate].kind.is_and())
                    .copied();
                let z_above = below_p
                    .iter()
                    .find(|&&gate| input.gates[gate].kind.is_xor())
                    .copied();
                (r_above, z_above)
            } else {
                (None, None)
            };
        if !input.gates[carry_below].kind.is_or() {
            broken_gates.push(carry_below);
        }
        if let Some(_p_and_r_below) = input.gates[carry_below].kind.as_binary() {
            todo!()
        }
    }

    0
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
enum GateKind {
    #[default]
    None,
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
    Constant(bool),
}

#[expect(dead_code)]
impl GateKind {
    fn is_or(self) -> bool {
        matches!(self, GateKind::Or(_, _))
    }
    fn is_and(self) -> bool {
        matches!(self, GateKind::And(_, _))
    }
    fn is_xor(self) -> bool {
        matches!(self, GateKind::Xor(_, _))
    }
    fn is_constant(self) -> bool {
        matches!(self, GateKind::Constant(_))
    }

    fn is_binary(self) -> bool {
        matches!(
            self,
            GateKind::And(_, _) | GateKind::Or(_, _) | GateKind::Xor(_, _)
        )
    }
    fn has_operand(self, operand: usize) -> bool {
        match self {
            GateKind::And(lhs, rhs) | GateKind::Or(lhs, rhs) | GateKind::Xor(lhs, rhs) => {
                lhs == operand || rhs == operand
            }
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
    fn first(self) -> Option<usize> {
        Some(self.as_binary()?.0)
    }
    fn second(self) -> Option<usize> {
        Some(self.as_binary()?.1)
    }
    fn as_binary(self) -> Option<(usize, usize)> {
        match self {
            GateKind::And(lhs, rhs) | GateKind::Or(lhs, rhs) | GateKind::Xor(lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_and(self) -> Option<(usize, usize)> {
        match self {
            GateKind::And(lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_or(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Or(lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }
    fn as_xor(self) -> Option<(usize, usize)> {
        match self {
            GateKind::Xor(lhs, rhs) => Some((lhs, rhs)),
            _ => None,
        }
    }

    fn flip(self) -> Option<Self> {
        Some(match self {
            GateKind::And(lhs, rhs) => GateKind::Or(lhs, rhs),
            GateKind::Or(lhs, rhs) => GateKind::And(lhs, rhs),
            GateKind::Xor(lhs, rhs) => GateKind::Xor(lhs, rhs),
            _ => None?,
        })
    }

    fn evaluate(self, inputs: &[Option<bool>]) -> Result<bool, usize> {
        Ok(match self {
            GateKind::None => unreachable!(),
            GateKind::Constant(value) => value,
            GateKind::And(lhs, rhs) => {
                let lhs_value = inputs[lhs].ok_or(lhs)?;
                let rhs_value = inputs[rhs].ok_or(rhs)?;
                lhs_value && rhs_value
            }
            GateKind::Or(lhs, rhs) => {
                let lhs_value = inputs[lhs].ok_or(lhs)?;
                let rhs_value = inputs[rhs].ok_or(rhs)?;
                lhs_value || rhs_value
            }
            GateKind::Xor(lhs, rhs) => {
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
        parse_gates(&mut gates, lookup, lines)?;
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
    mut lookup: HashMap<&'a str, usize>,
    lines: std::str::Lines<'a>,
) -> Result<(), ParseInputError> {
    for line in lines {
        let (lhs, rhs) = line
            .split_once(" -> ")
            .ok_or(ParseInputError::MissingDelimiter)?;
        let kind = if let Some((left, right)) = lhs.split_once(" AND ") {
            let left_gate = *lookup.entry(left).or_insert_with(|| {
                gates.push(Gate {
                    name: left,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            let right_gate = *lookup.entry(right).or_insert_with(|| {
                gates.push(Gate {
                    name: right,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            GateKind::And(left_gate, right_gate)
        } else if let Some((left, right)) = lhs.split_once(" OR ") {
            let left_gate = *lookup.entry(left).or_insert_with(|| {
                gates.push(Gate {
                    name: left,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            let right_gate = *lookup.entry(right).or_insert_with(|| {
                gates.push(Gate {
                    name: right,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            GateKind::Or(left_gate, right_gate)
        } else if let Some((left, right)) = lhs.split_once(" XOR ") {
            let left_gate = *lookup.entry(left).or_insert_with(|| {
                gates.push(Gate {
                    name: left,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            let right_gate = *lookup.entry(right).or_insert_with(|| {
                gates.push(Gate {
                    name: right,
                    kind: GateKind::None,
                });
                gates.len() - 1
            });
            GateKind::Xor(left_gate, right_gate)
        } else {
            return Err(ParseInputError::SyntaxError);
        };
        let gate = *lookup.entry(rhs).or_insert_with(|| {
            gates.push(Gate {
                name: rhs,
                kind: GateKind::None,
            });
            gates.len() - 1
        });
        gates[gate].kind = kind;
    }
    Ok(())
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
