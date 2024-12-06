use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 06");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 41)", part_1(&example));
    println!("|'-Part 2: {} (expected 6)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 4776)", part_1(&input));
    println!("|'-Part 2: {} (expected <1587)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut visited = HashMap::<(u8, u8), u8>::new();
    let mut guard = input.guard;
    loop {
        let pos = guard.pos;
        let mask = visited.entry(pos).or_insert(0);
        if *mask & guard.dir.to_bitmask() != 0 {
            break;
        }
        *mask |= guard.dir.to_bitmask();
        match guard.move_forward(input, None) {
            MoveResult::Ok => {}
            MoveResult::Exited => break,
            MoveResult::HitObstacle => {
                guard.turn_right();
            }
        }
    }
    visited.values().filter(|&&mask| mask != 0b0000).count()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut initial_visited = HashMap::<(u8, u8), u8>::new();
    let mut guard = input.guard;
    loop {
        let pos = guard.pos;
        let mask = initial_visited.entry(pos).or_insert(0);
        if *mask & guard.dir.to_bitmask() != 0 {
            break;
        }
        *mask |= guard.dir.to_bitmask();
        match guard.move_forward(input, None) {
            MoveResult::Ok => {}
            MoveResult::Exited => break,
            MoveResult::HitObstacle => {
                guard.turn_right();
            }
        }
    }
    let mut visited = HashMap::<(u8, u8), u8>::new();
    let mut loop_counts = 0;
    for new_obstacle in initial_visited.into_keys() {
        visited.clear();
        guard = input.guard;
        loop {
            let pos = guard.pos;
            let mask = visited.entry(pos).or_insert(0);
            if *mask & guard.dir.to_bitmask() != 0 {
                loop_counts += 1;
                break;
            }
            *mask |= guard.dir.to_bitmask();
            match guard.move_forward(input, Some(new_obstacle)) {
                MoveResult::Ok => {}
                MoveResult::Exited => break,
                MoveResult::HitObstacle => {
                    guard.turn_right();
                }
            }
        }
    }
    loop_counts
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    #[must_use]
    pub fn to_bitmask(self) -> u8 {
        match self {
            Self::Up => 0b0001,
            Self::Down => 0b0010,
            Self::Left => 0b0100,
            Self::Right => 0b1000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Ok,
    Exited,
    HitObstacle,
}

#[derive(Debug, Clone, Copy)]
pub struct Guard {
    pos: (u8, u8),
    dir: Dir,
}

impl Guard {
    #[must_use]
    pub const fn new(pos: (u8, u8), dir: Dir) -> Self {
        Self { pos, dir }
    }

    pub fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }

    #[must_use]
    pub fn ahead(&self, input: &Input) -> Option<(u8, u8)> {
        let (r, c) = self.pos;
        let (r, c) = match self.dir {
            Dir::Up if r > 0 => (r - 1, c),
            Dir::Down => (r + 1, c),
            Dir::Left if c > 0 => (r, c - 1),
            Dir::Right => (r, c + 1),
            _ => return None,
        };
        if r >= input.height || c >= input.width {
            return None;
        }
        Some((r, c))
    }

    pub fn move_forward(&mut self, input: &Input, new_obstacle: Option<(u8, u8)>) -> MoveResult {
        if let Some((r, c)) = self.ahead(input) {
            if input.obstacles.contains(&(r, c)) || new_obstacle == Some((r, c)) {
                return MoveResult::HitObstacle;
            }
            self.pos = (r, c);
            MoveResult::Ok
        } else {
            MoveResult::Exited
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    width: u8,
    height: u8,
    obstacles: HashSet<(u8, u8)>,
    guard: Guard,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Input is empty")]
    EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

#[allow(clippy::cast_possible_truncation)]
impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut obstacles = HashSet::new();
        let mut guard_start = (0, 0);
        let mut guard_direction = Dir::Up;
        let mut width = 0;
        let mut height = 0;
        for (r, row) in text.lines().enumerate() {
            let r = r as u8;
            height = r + 1;
            width = row.len() as u8;
            for (c, cell) in row.chars().enumerate() {
                let c = c as u8;
                match cell {
                    '.' => {}
                    '#' => {
                        obstacles.insert((r, c));
                    }
                    '^' => {
                        guard_start = (r, c);
                        guard_direction = Dir::Up;
                    }
                    '<' => {
                        guard_start = (r, c);
                        guard_direction = Dir::Left;
                    }
                    'v' => {
                        guard_start = (r, c);
                        guard_direction = Dir::Down;
                    }
                    '>' => {
                        guard_start = (r, c);
                        guard_direction = Dir::Right;
                    }
                    _ => return Err(ParseInputError::InvalidChar(cell)),
                }
            }
        }
        Ok(Self {
            width,
            height,
            obstacles,
            guard: Guard::new(guard_start, guard_direction),
        })
    }
}
