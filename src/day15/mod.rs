use std::collections::HashSet;
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
    let mut state = input.initial_state.clone();
    for mv in &input.moves {
        let Some(ahead) = state.robot.move_by(*mv) else {
            // Moved off the board
            continue;
        };
        if let Some(&Tile::Wall) = input.grid.get(ahead.x, ahead.y) {
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
    let mut state = ExpandedState::from_state(&input.initial_state);
    for &mv in &input.moves {
        let Some(ahead) = state.robot.move_by(mv) else {
            continue; // Moved off the board
        };
        if let Some(&Tile::Wall) = input.grid.get(ahead.x / 2, ahead.y) {
            continue; // Hit a wall
        }
        let Some(pushable_boxes) = state.pushable_boxes(mv, input) else {
            continue; // Hit an unpushable box
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

impl Move {
    fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }
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
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Move by a single step in a direction. If this would result in a negative position, return `None`.
    fn move_by(&self, mv: Move) -> Option<Self> {
        Some(match mv {
            Move::Up => Self::new(self.x, self.y.checked_sub(1)?),
            Move::Down => Self::new(self.x, self.y + 1),
            Move::Left => Self::new(self.x.checked_sub(1)?, self.y),
            Move::Right => Self::new(self.x + 1, self.y),
        })
    }

    /// A checksum for the position
    fn gps_coordinate(&self) -> usize {
        self.y * 100 + self.x
    }

    /// Expand the position by doubling the x coordinate
    fn expand(&self) -> Self {
        Self::new(self.x * 2, self.y)
    }
}

/// The parsed input data
#[derive(Debug, Clone)]
pub struct Input {
    grid: Grid<Tile>,
    moves: Vec<Move>,
    initial_state: State,
}

/// The dynamic state of the simulation
#[derive(Debug, Clone)]
pub struct State {
    robot: Position,
    boxes: HashSet<Position>,
}

impl State {
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

struct ExpandedState {
    robot: Position,
    boxes: HashSet<Position>,
}

impl ExpandedState {
    pub fn from_state(state: &State) -> Self {
        Self {
            robot: state.robot.expand(),
            boxes: state.boxes.iter().map(Position::expand).collect(),
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
            if self.boxes.contains(&ahead) {
                // Left half of a box
                boxes.push(ahead);
                if mv.is_horizontal() {
                    pending.push(ahead.move_by(mv)?);
                } else {
                    pending.push(ahead);
                    pending.push(ahead_right);
                }
            } else if self.boxes.contains(&ahead_left) {
                // Right half of a box
                boxes.push(ahead_left);
                if mv.is_horizontal() {
                    pending.push(ahead.move_by(mv)?);
                } else {
                    pending.push(ahead);
                    pending.push(ahead_left);
                }
            }
        }
        Some(boxes)
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
        let initial_state = State {
            robot: robot.ok_or(ParseInputError::MissingRobot)?,
            boxes,
        };
        Ok(Self {
            grid,
            moves,
            initial_state,
        })
    }
}
