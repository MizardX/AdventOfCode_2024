use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

use crate::aoclib::Grid;

const EXAMPLE1: &str = include_str!("example1.txt");
const EXAMPLE2: &str = include_str!("example2.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 15");

    println!("++Example 1");
    let example1 = EXAMPLE1.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 10_092)", part_1(&example1));
    println!("|'-Part 2: {} (expected 9_021)", part_2(&example1));

    println!("++Example 2");
    let example2 = EXAMPLE2.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 2_028)", part_1(&example2));
    println!("|'-Part 2: {} (expected 1_751)", part_2(&example2));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 1_515_788)", part_1(&input));
    println!("|'-Part 2: {} (expected 1_516_544)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut state = State::from_input(input);
    for mv in &input.moves {
        let Some(ahead) = state.robot.move_by(*mv) else {
            // Moved off the board
            continue;
        };
        if matches!(input.grid.get(ahead.x, ahead.y), Some(&Tile::Wall)) {
            // Hit a wall
            continue;
        }
        if state.boxes.contains(&ahead) {
            let Some(empty) = state.next_empty(*mv, input) else {
                // Hit an unpushable box
                continue;
            };
            // Remove the first box, and insert a new one at the end, simulating pushing
            state.boxes.remove(&ahead);
            state.boxes.insert(empty);
        }
        state.robot = ahead;
    }
    state.boxes.iter().map(Position::gps_coordinate).sum()
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let mut state = ExpandedState::from_input(input);
    for &mv in &input.moves {
        let Some(ahead) = state.robot.move_by(mv) else {
            // Moved off the board
            continue;
        };
        if matches!(input.grid.get(ahead.x / 2, ahead.y), Some(&Tile::Wall)) {
            // Hit a wall
            continue;
        }
        let Some(pushable_boxes) = state.pushable_boxes(mv, input) else {
            // Hit an unpushable box
            continue;
        };
        // Remove old positions, so they do not overlap with new positions in the set
        for &pushable_box in &pushable_boxes {
            state.boxes.remove(&pushable_box);
        }
        // Add new positions
        for &pushable_box in &pushable_boxes {
            let new_position = pushable_box.move_by(mv).unwrap();
            state.boxes.insert(new_position);
        }
        state.robot = ahead;
    }
    state.boxes.iter().map(Position::gps_coordinate).sum()
}

/// A static tile of the map
#[derive(Debug, Clone, Copy, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

impl TryFrom<u8> for Tile {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Empty),
            b'#' => Ok(Self::Wall),
            ch => Err(ParseInputError::InvalidChar(ch as char)),
        }
    }
}

/// A move instruction
#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<u8> for Move {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'^' => Ok(Self::Up),
            b'v' => Ok(Self::Down),
            b'<' => Ok(Self::Left),
            b'>' => Ok(Self::Right),
            ch => Err(ParseInputError::InvalidChar(ch as char)),
        }
    }
}

/// A position of an entity on the map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Move by a single step in a direction. If this would result in a negative position, return `None`.
    const fn move_by(&self, mv: Move) -> Option<Self> {
        Some(match mv {
            Move::Up if self.y == 0 => return None,
            Move::Up => Self::new(self.x, self.y - 1),
            Move::Down => Self::new(self.x, self.y + 1),
            Move::Left if self.x == 0 => return None,
            Move::Left => Self::new(self.x - 1, self.y),
            Move::Right => Self::new(self.x + 1, self.y),
        })
    }

    /// A checksum for the position
    const fn gps_coordinate(&self) -> usize {
        self.y * 100 + self.x
    }

    /// Expand the position by doubling the x coordinate
    const fn expand(&self) -> Self {
        Self::new(self.x * 2, self.y)
    }
}

/// The parsed input data
#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Tile>,
    moves: Vec<Move>,
    robot: Position,
    boxes: HashSet<Position>,
}

/// The dynamic state of the simulation
#[derive(Debug, Clone)]
pub struct State<'a> {
    input: &'a Input,
    robot: Position,
    boxes: HashSet<Position>,
}

impl<'a> State<'a> {
    fn from_input(input: &'a Input) -> Self {
        Self {
            input,
            robot: input.robot,
            boxes: input.boxes.clone(),
        }
    }

    /// Find the next empty position in a direction
    fn next_empty(&self, mv: Move, input: &Input) -> Option<Position> {
        let mut pos = self.robot;
        loop {
            pos = pos.move_by(mv)?;
            match input.grid.get(pos.x, pos.y)? {
                Tile::Wall => return None,
                Tile::Empty if !self.boxes.contains(&pos) => return Some(pos),
                Tile::Empty => {}
            }
        }
    }
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.input.grid.height() {
            for x in 0..self.input.grid.width() {
                let pos = Position::new(x, y);
                if pos == self.robot {
                    write!(f, "@")?;
                } else if self.boxes.contains(&pos) {
                    write!(f, "O")?;
                } else if let Some(&tile) = self.input.grid.get(x, y) {
                    match tile {
                        Tile::Empty => write!(f, ".")?,
                        Tile::Wall => write!(f, "#")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ExpandedState<'a> {
    input: &'a Input,
    robot: Position,
    boxes: HashSet<Position>,
}

#[derive(Debug, Clone, Copy)]
enum HalfBox {
    Left,
    Right,
}

impl<'a> ExpandedState<'a> {
    pub fn from_input(input: &'a Input) -> Self {
        Self {
            input,
            robot: input.robot.expand(),
            boxes: input.boxes.iter().map(Position::expand).collect(),
        }
    }

    /// Find all boxes that must be moved in a direction, assuming expanded coordinates
    fn pushable_boxes(&self, mv: Move, input: &Input) -> Option<Vec<Position>> {
        let mut boxes = Vec::new();
        let mut pending = vec![self.robot];
        while let Some(pos) = pending.pop() {
            let ahead = pos.move_by(mv)?;
            let ahead_left = ahead.move_by(Move::Left)?;
            let ahead_right = ahead.move_by(Move::Right)?;
            if matches!(input.grid.get(ahead.x / 2, ahead.y)?, Tile::Wall) {
                return None;
            }
            match (self.get_box(ahead), mv) {
                (Some(HalfBox::Left), Move::Right) => {
                    boxes.push(ahead);
                    pending.push(ahead_right);
                }
                (Some(HalfBox::Right), Move::Left) => {
                    boxes.push(ahead_left);
                    pending.push(ahead_left);
                }
                (Some(HalfBox::Left), Move::Up | Move::Down) => {
                    boxes.push(ahead);
                    pending.push(ahead);
                    pending.push(ahead_right);
                }
                (Some(HalfBox::Right), Move::Up | Move::Down) => {
                    boxes.push(ahead_left);
                    pending.push(ahead_left);
                    pending.push(ahead);
                }
                _ => (),
            }
        }
        Some(boxes)
    }

    fn get_box(&self, pos: Position) -> Option<HalfBox> {
        if self.boxes.contains(&pos) {
            Some(HalfBox::Left)
        } else if pos.move_by(Move::Left).is_none() {
            None
        } else if self.boxes.contains(&pos.move_by(Move::Left)?) {
            Some(HalfBox::Right)
        } else {
            None
        }
    }
}

impl Display for ExpandedState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.input.grid.height() {
            for x in 0..self.input.grid.width() * 2 {
                let pos = Position::new(x, y);
                if pos == self.robot {
                    write!(f, "@")?;
                } else if let Some(half) = self.get_box(pos) {
                    match half {
                        HalfBox::Left => write!(f, "[")?,
                        HalfBox::Right => write!(f, "]")?,
                    }
                } else if let Some(&tile) = self.input.grid.get(x / 2, y) {
                    match tile {
                        Tile::Empty => write!(f, ".")?,
                        Tile::Wall => write!(f, "#")?,
                    }
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
    #[error("Missing robot")]
    MissingRobot,
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let width = text.lines().next().map_or(0, str::len);
        let height = text
            .lines()
            .map(str::len)
            .take_while(|&len| len > 0)
            .count();
        let mut grid = Grid::new(width, height);
        let mut boxes = HashSet::new();
        let mut robot = None;
        for (y, line) in text.lines().enumerate().take(height) {
            for (x, ch) in line.bytes().enumerate().take(width) {
                match ch {
                    b'@' => robot = Some(Position::new(x, y)),
                    b'O' => {
                        boxes.insert(Position::new(x, y));
                    }
                    ch => grid.set(x, y, ch.try_into()?),
                }
            }
        }
        let moves = text
            .lines()
            .skip(height + 1)
            .flat_map(str::bytes)
            .map(u8::try_into)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            grid,
            moves,
            robot: robot.ok_or(ParseInputError::MissingRobot)?,
            boxes,
        })
    }
}
