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
    println!("|+-Part 1: {} (expected 4_776)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_586)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let (visited, _) = do_walk(input, None);
    visited.values().filter(|&&mask| mask != 0b0000).count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum WalkResult {
    Loop,
    Exit,
}

fn do_walk(input: &Input, new_obstacle: Option<(u8, u8)>) -> (HashMap<(u8, u8), u8>, WalkResult) {
    let mut visited = HashMap::<(u8, u8), u8>::new();
    let mut guard = input.guard;
    loop {
        let pos = guard.pos;
        let mask = visited.entry(pos).or_insert(0);
        if *mask & guard.dir.to_bitmask() != 0 {
            return (visited, WalkResult::Loop);
        }
        *mask |= guard.dir.to_bitmask();
        match guard.move_forward(input, new_obstacle) {
            MoveResult::Ok => {}
            MoveResult::Exited => return (visited, WalkResult::Exit),
            MoveResult::HitObstacle => {
                guard.turn_right();
            }
        }
    }
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let (initial_visited, _) = do_walk(input, None);
    let mut loop_counts = 0;
    for new_obstacle in initial_visited.into_keys() {
        if new_obstacle == input.guard.pos || input.obstacles.contains(&new_obstacle) {
            continue;
        }
        if do_fast_walk(input, Some(new_obstacle)) == WalkResult::Loop {
            loop_counts += 1;
        }
    }
    loop_counts
}

fn do_fast_walk(input: &Input, new_obstacle: Option<(u8, u8)>) -> WalkResult {
    let mut visited = HashMap::<(u8, u8), u8>::new();
    let mut guard = input.guard;
    loop {
        let pos = guard.pos;
        let mask = visited.entry(pos).or_insert(0);
        if *mask & guard.dir.to_bitmask() != 0 {
            return WalkResult::Loop;
        }
        *mask |= guard.dir.to_bitmask();
        match guard.move_forward_fast(input, new_obstacle) {
            MoveResult::Ok => {}
            MoveResult::Exited => return WalkResult::Exit,
            MoveResult::HitObstacle => {
                guard.turn_right();
            }
        }
    }
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
    pub const fn to_bitmask(self) -> u8 {
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
    pub const fn ahead(&self, input: &Input) -> Option<(u8, u8)> {
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

    pub fn move_forward_fast(
        &mut self,
        input: &Input,
        new_obstacle: Option<(u8, u8)>,
    ) -> MoveResult {
        match self.dir {
            Dir::Up | Dir::Down => {
                let obstacles_this_col = &input.obstacles_per_col[self.pos.1 as usize];
                let extra_obstacle_row = match (new_obstacle, self.dir) {
                    (Some((r, c)), Dir::Up) if c == self.pos.1 && r < self.pos.0 => Some(r),
                    (Some((r, c)), Dir::Down) if c == self.pos.1 && r > self.pos.0 => Some(r),
                    _ => None,
                };
                let obstacle_row = match (obstacles_this_col.binary_search(&self.pos.0), self.dir) {
                    (Err(0), Dir::Up) => None,
                    (Err(next_ix), Dir::Down) if next_ix == obstacles_this_col.len() => None,
                    (Err(next_ix), Dir::Up) => Some(obstacles_this_col[next_ix - 1]),
                    (Err(next_ix), Dir::Down) => Some(obstacles_this_col[next_ix]),
                    _ => unreachable!("We should not start inside an obstacle"),
                };
                let obstacle_row = match (obstacle_row, extra_obstacle_row, self.dir) {
                    (None, Some(c), _) => Some(c),
                    (Some(l), None, _) => Some(l),
                    (Some(l), Some(r), Dir::Up) => Some(l.max(r)),
                    (Some(l), Some(r), Dir::Down) => Some(l.min(r)),
                    _ => None,
                };
                match (obstacle_row, self.dir) {
                    (Some(r), Dir::Up) => {
                        self.pos = (r + 1, self.pos.1);
                        MoveResult::HitObstacle
                    }
                    (Some(r), Dir::Down) => {
                        self.pos = (r - 1, self.pos.1);
                        MoveResult::HitObstacle
                    }
                    (None, _) => MoveResult::Exited,
                    _ => unreachable!(),
                }
            }
            Dir::Left | Dir::Right => {
                let obstacles_this_row = &input.obstacles_per_row[self.pos.0 as usize];
                let extra_obstacle_col = match (new_obstacle, self.dir) {
                    (Some((r, c)), Dir::Left) if r == self.pos.0 && c < self.pos.1 => Some(c),
                    (Some((r, c)), Dir::Right) if r == self.pos.0 && c > self.pos.1 => Some(c),
                    _ => None,
                };
                let obstacle_col = match (obstacles_this_row.binary_search(&self.pos.1), self.dir) {
                    (Err(next_ix), Dir::Left) if next_ix > 0 => {
                        Some(obstacles_this_row[next_ix - 1])
                    }
                    (Err(next_ix), Dir::Right) if next_ix < obstacles_this_row.len() => {
                        Some(obstacles_this_row[next_ix])
                    }
                    _ => None,
                };
                let obstacle_col = match (obstacle_col, extra_obstacle_col, self.dir) {
                    (None, Some(r), _) => Some(r),
                    (Some(l), None, _) => Some(l),
                    (Some(l), Some(r), Dir::Left) => Some(l.max(r)),
                    (Some(l), Some(r), Dir::Right) => Some(l.min(r)),
                    _ => None,
                };
                match (obstacle_col, self.dir) {
                    (Some(c), Dir::Left) => {
                        self.pos = (self.pos.0, c + 1);
                        MoveResult::HitObstacle
                    }
                    (Some(c), Dir::Right) => {
                        self.pos = (self.pos.0, c - 1);
                        MoveResult::HitObstacle
                    }
                    (None, _) => MoveResult::Exited,
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    width: u8,
    height: u8,
    obstacles: HashSet<(u8, u8)>,
    obstacles_per_row: Vec<Vec<u8>>,
    obstacles_per_col: Vec<Vec<u8>>,
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
        let mut obstacles_per_row = Vec::new();
        let mut obstacles_per_col = Vec::new();
        let mut guard_start = (0, 0);
        let mut guard_direction = Dir::Up;
        let mut width = 0;
        let mut height = 0;
        for (r, row) in text.lines().enumerate() {
            let r = r as u8;
            height = r + 1;
            width = row.len() as u8;
            if obstacles_per_col.len() < width as usize {
                obstacles_per_col.resize(width as usize, Vec::new());
            }
            let mut obstacles_this_row = Vec::new();
            for (c, cell) in row.chars().enumerate() {
                let c = c as u8;
                match cell {
                    '.' => {}
                    '#' => {
                        obstacles.insert((r, c));
                        obstacles_this_row.push(c);
                        obstacles_per_col[c as usize].push(r);
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
            obstacles_per_row.push(obstacles_this_row);
        }
        Ok(Self {
            width,
            height,
            obstacles,
            obstacles_per_row,
            obstacles_per_col,
            guard: Guard::new(guard_start, guard_direction),
        })
    }
}
