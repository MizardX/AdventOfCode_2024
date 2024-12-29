use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display, Formatter};
use std::iter::FusedIterator;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 08");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 14)", part_1(&example));
    println!("|'-Part 2: {} (expected 34)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 271)", part_1(&input));
    println!("|'-Part 2: {} (expected 994)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut by_freq = HashMap::<u8, Vec<&Antenna>>::new();
    let mut antinodes = HashSet::<(usize, usize)>::new();
    let bounds = (input.width, input.height);
    for antenna in &input.antennas {
        let group = by_freq.entry(antenna.freq).or_default();
        for &prev in group.iter() {
            if let Some((x, y)) = antenna.first_antinode(*prev, bounds) {
                if input.includes(x, y) {
                    antinodes.insert((x, y));
                }
            }
            if let Some((x, y)) = prev.first_antinode(*antenna, bounds) {
                if input.includes(x, y) {
                    antinodes.insert((x, y));
                }
            }
        }
        group.push(antenna);
    }
    antinodes.len()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut by_freq = HashMap::<u8, Vec<&Antenna>>::new();
    let mut found_antinodes = HashSet::<(usize, usize)>::new();
    for antenna in &input.antennas {
        let group = by_freq.entry(antenna.freq).or_default();
        for &prev in group.iter() {
            if let Some(antinodes) = antenna.antinodes(*prev, (input.width, input.height)) {
                for antinode in antinodes {
                    found_antinodes.insert(antinode);
                }
            }
            if let Some(antinodes) = prev.antinodes(*antenna, (input.width, input.height)) {
                for antinode in antinodes {
                    found_antinodes.insert(antinode);
                }
            }
        }
        group.push(antenna);
    }
    found_antinodes.len()
}

#[derive(Debug, Clone, Copy)]
pub struct Antenna {
    pub x: usize,
    pub y: usize,
    pub freq: u8,
}

impl Antenna {
    const fn new(x: usize, y: usize, freq: u8) -> Self {
        Self { x, y, freq }
    }

    fn first_antinode(self, other: Antenna, bounds: (usize, usize)) -> Option<(usize, usize)> {
        self.antinodes(other, bounds)?.nth(1)
    }

    fn antinodes(self, other: Antenna, bounds: (usize, usize)) -> Option<Antinodes> {
        let step = (
            isize::try_from(other.x).ok()? - isize::try_from(self.x).ok()?,
            isize::try_from(other.y).ok()? - isize::try_from(self.y).ok()?,
        );
        Some(Antinodes {
            start: (self.x, self.y),
            step,
            bounds,
            complete: false,
        })
    }
}

impl Display for Antenna {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Antenna { x, y, freq } = *self;
        let freq = freq as char;
        write!(f, "({x}, {y}) freq '{freq}'")
    }
}

#[derive(Debug, Clone)]
struct Antinodes {
    start: (usize, usize),
    step: (isize, isize),
    bounds: (usize, usize),
    complete: bool,
}
impl FusedIterator for Antinodes {}
impl Iterator for Antinodes {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.complete {
            return None;
        }
        if let (Some(new_x), Some(new_y)) = (
            self.start.0.checked_add_signed(self.step.0),
            self.start.1.checked_add_signed(self.step.1),
        ) {
            if new_x >= self.bounds.0 || new_y >= self.bounds.1 {
                self.complete = true;
                None
            } else {
                self.start = (new_x, new_y);
                Some((new_x, new_y))
            }
        } else {
            self.complete = true;
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub width: usize,
    pub height: usize,
    pub antennas: Vec<Antenna>,
}

impl Input {
    #[must_use]
    pub const fn new(width: usize, height: usize, antennas: Vec<Antenna>) -> Self {
        Self {
            width,
            height,
            antennas,
        }
    }
}

impl Input {
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub const fn includes(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    // #[error("Unexpected character: '{0}'")]
    // InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut antennas = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in text.lines().enumerate() {
            height = y + 1;
            width = line.len();
            for (x, freq) in line.bytes().enumerate() {
                if freq != b'.' {
                    antennas.push(Antenna::new(x, y, freq));
                }
            }
        }
        Ok(Self {
            width,
            height,
            antennas,
        })
    }
}
