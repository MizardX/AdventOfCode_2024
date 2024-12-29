use std::collections::VecDeque;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 22");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 37_327_623)", part_1(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example");
    println!("|+-Part 2: {} (expected 23)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 12_664_695_565)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_444)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    let mut sum = 0;
    for &secret_number in &input.secret_numbers {
        sum += PseduoRandom::new(secret_number).nth(1_999).unwrap_or(0);
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let mut price_per_sequence = vec![(0, usize::MAX); 19 * 19 * 19 * 19].into_boxed_slice();
    for (monkey, &secret_number) in input.secret_numbers.iter().enumerate() {
        collect_price_fluctuations(monkey, secret_number, &mut price_per_sequence);
    }
    price_per_sequence
        .iter()
        .map(|&(v, _)| v)
        .max()
        .unwrap_or(0)
}

#[expect(clippy::cast_sign_loss, reason = "All values are between -9 and 9")]
const fn to_index(d1: i8, d2: i8, d3: i8, d4: i8) -> usize {
    let d1 = (d1 + 9) as usize;
    let d2 = (d2 + 9) as usize;
    let d3 = (d3 + 9) as usize;
    let d4 = (d4 + 9) as usize;
    d1 * 19 * 19 * 19 + d2 * 19 * 19 + d3 * 19 + d4
}

fn collect_price_fluctuations(
    monkey: usize,
    secret_number: u64,
    price_per_sequence: &mut [(u64, usize)],
) {
    let mut diffs = VecDeque::new();
    let mut prev_price = (secret_number % 10) as u8;

    for value in PseduoRandom::new(secret_number).take(2_000) {
        let price = (value % 10) as u8;
        #[expect(clippy::cast_possible_wrap)]
        let diff = price.wrapping_sub(prev_price) as i8;

        diffs.push_back(diff);

        if diffs.len() == 4 {
            let key = to_index(diffs[0], diffs[1], diffs[2], diffs[3]);
            let (total, last_index) = &mut price_per_sequence[key];
            if *last_index != monkey {
                *last_index = monkey;
                *total += u64::from(price);
            }
            diffs.pop_front();
        }

        prev_price = price;
    }
}

struct PseduoRandom {
    value: u64,
}

impl PseduoRandom {
    const fn new(secret_number: u64) -> Self {
        Self {
            value: secret_number,
        }
    }
}

impl Iterator for PseduoRandom {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // mix(a,b) = a ^ b
        // prune(x) = x % 16777216
        // x = prune(mix(x, x * 32))
        // x = prune(mix(x, x / 16))
        // x = prune(mix(x, x * 2048))
        let mut value = self.value;
        value = ((value << 6) ^ value) & 0xFF_FFFF;
        value = ((value >> 5) ^ value) & 0xFF_FFFF;
        value = ((value << 11) ^ value) & 0xFF_FFFF;
        self.value = value;
        Some(self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    secret_numbers: Vec<u64>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let secret_numbers = text.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { secret_numbers })
    }
}
