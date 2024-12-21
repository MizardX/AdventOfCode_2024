use std::collections::btree_map::Keys;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 21");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 126_384)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    for code in &input.codes {
        let state = State::new(code);
        sum += astar(state).unwrap();
    }
    sum
}

#[expect(unused)]
fn astar(start: State) -> Option<usize> {
    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();
    open.push(start);
    while let Some(current) = open.pop() {
        if current.code_ix == current.code_seq.len() {
            return Some(current.dist);
        }
        if !closed.insert(current) {
            continue;
        }
        for next in current.moves() {
            if closed.contains(&next) {
                continue;
            }
            open.push(next);
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State<'a> {
    dist: usize,
    code_ix: usize,
    keypad: Num,
    directional1: Dir,
    directional2: Dir,
    code_seq: &'a [Num],
}

impl<'a> State<'a> {
    pub fn new(code_seq: &'a [Num]) -> Self {
        Self {
            dist: 0,
            code_ix: 0,
            keypad: Num::A,
            directional1: Dir::A,
            directional2: Dir::A,
            code_seq,
        }
    }

    pub fn moves(self) -> impl Iterator<Item = Self> {
        let Self {
            dist,
            code_ix,
            keypad,
            directional1,
            directional2,
            code_seq
        } = self;
        let next_code = code_seq[code_ix];
        Dir::all().into_iter().filter_map(move |action| {
            match (action, directional1, directional2) {
                (Dir::A, Dir::A, Dir::A) => (keypad == next_code).then_some(Self {
                    dist: dist + 1,
                    code_ix: code_ix + 1,
                    ..self
                }),
                (Dir::A, Dir::A, _) => keypad.move_finger(directional2).map(|new_keypad| Self {
                    dist: dist + 1,
                    keypad: new_keypad,
                    ..self
                }),
                (Dir::A, _, _) => directional1
                    .move_finger(directional1)
                    .map(|new_directional1| Self {
                        dist: dist + 1,
                        directional1: new_directional1,
                        ..self
                    }),
                (_, _, _) => directional2
                    .move_finger(action)
                    .map(|new_directional2| Self {
                        dist: dist + 1,
                        directional2: new_directional2,
                        ..self
                    }),
            }
        })
    }

    fn heuristic(&self) -> usize {
        let Self {
            code_ix,
            keypad,
            directional1,
            directional2,
            code_seq,
            ..
        } = *self;
        let next_code = code_seq.get(code_ix).copied().unwrap_or(Num::A);
        let mut h = code_seq.len() - code_ix;
        h *= 10;
        h += keypad.dist(next_code) as usize;
        h *= 10;
        h += directional1.dist(Dir::A) as usize;
        h *= 10;
        h += directional2.dist(Dir::A) as usize;
        h
    }

    fn score(&self) -> usize {
        self.dist + self.heuristic()
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score().cmp(&self.score())
    }
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Num {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Num {
    pub const fn all() -> [Self; 11] {
        [
            Self::Zero,
            Self::One,
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::A,
        ]
    }

    fn position(self) -> (u8, u8) {
        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+
        match self {
            Self::Zero => (3, 1),
            Self::One => (2, 0),
            Self::Two => (2, 1),
            Self::Three => (2, 2),
            Self::Four => (1, 0),
            Self::Five => (1, 1),
            Self::Six => (1, 2),
            Self::Seven => (0, 0),
            Self::Eight => (0, 1),
            Self::Nine => (0, 2),
            Self::A => (3, 2),
        }
    }

    fn dist(self, other: Self) -> u8 {
        let (r1, c1) = self.position();
        let (r2, c2) = other.position();
        r1.abs_diff(r2) + c1.abs_diff(c2)
    }

    fn move_finger(self, dir: Dir) -> Option<Self> {
        Some(match (self, dir) {
            (Self::Two, Dir::Down) | (Self::A, Dir::Left) => Self::Zero,
            (Self::Two, Dir::Left) | (Self::Four, Dir::Down) => Self::One,
            (Self::Zero, Dir::Up)
            | (Self::One, Dir::Right)
            | (Self::Three, Dir::Left)
            | (Self::Five, Dir::Down) => Self::Two,
            (Self::Two, Dir::Right) | (Self::Six, Dir::Down) | (Self::A, Dir::Up) => Self::Three,
            (Self::One, Dir::Up) | (Self::Five, Dir::Left) | (Self::Seven, Dir::Down) => Self::Four,
            (Self::Two, Dir::Up)
            | (Self::Four, Dir::Right)
            | (Self::Six, Dir::Left)
            | (Self::Eight, Dir::Down) => Self::Five,
            (Self::Three, Dir::Up) | (Self::Five, Dir::Right) | (Self::Nine, Dir::Down) => {
                Self::Six
            }
            (Self::Four, Dir::Up) | (Self::Eight, Dir::Left) => Self::Seven,
            (Self::Five, Dir::Up) | (Self::Seven, Dir::Right) | (Self::Nine, Dir::Left) => {
                Self::Eight
            }
            (Self::Six, Dir::Up) | (Self::Eight, Dir::Right) => Self::Nine,
            (Self::Zero, Dir::Right) | (Self::Three, Dir::Down) => Self::A,
            _ => None?,
        })
    }
}

impl From<Num> for usize {
    fn from(value: Num) -> Self {
        match value {
            Num::Zero => 0,
            Num::One => 1,
            Num::Two => 2,
            Num::Three => 3,
            Num::Four => 4,
            Num::Five => 5,
            Num::Six => 6,
            Num::Seven => 7,
            Num::Eight => 8,
            Num::Nine => 9,
            Num::A => 10,
        }
    }
}

impl TryFrom<u8> for Num {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'0' => Ok(Self::Zero),
            b'1' => Ok(Self::One),
            b'2' => Ok(Self::Two),
            b'3' => Ok(Self::Three),
            b'4' => Ok(Self::Four),
            b'5' => Ok(Self::Five),
            b'6' => Ok(Self::Six),
            b'7' => Ok(Self::Seven),
            b'8' => Ok(Self::Eight),
            b'9' => Ok(Self::Nine),
            b'A' => Ok(Self::A),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Dir {
    pub const fn all() -> [Self; 5] {
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::A]
    }

    pub const fn position(self) -> (u8, u8) {
        //     +---+---+
        //     | ^ | A |
        // +---+---+---+
        // | < | v | > |
        // +---+---+---+
        match self {
            Self::Up => (0, 1),
            Self::Down => (1, 1),
            Self::Left => (1, 0),
            Self::Right => (1, 2),
            Self::A => (0, 2),
        }
    }

    fn dist(self, other: Self) -> u8 {
        let (r1, c1) = self.position();
        let (r2, c2) = other.position();
        r1.abs_diff(r2) + c1.abs_diff(c2)
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        let neighbors: [&[Self]; 5] = [
            &[Self::Down, Self::A],
            &[Self::Up, Self::Left, Self::Right],
            &[Self::Down],
            &[Self::Down, Self::A],
            &[Self::Up, Self::Right],
        ];
        neighbors[usize::from(self)].iter().copied()
    }

    fn move_finger(self, dir: Dir) -> Option<Self> {
        Some(match (self, dir) {
            (Self::Down, Self::Up) | (Self::A, Self::Left) => Self::Up,
            (Self::Up, Self::Down) | (Self::Left, Self::Right) | (Self::Right, Self::Left) => {
                Self::Down
            }
            (Self::Down, Self::Left) => Self::Left,
            (Self::Down, Self::Right) | (Self::A, Self::Down) => Self::Right,
            (Self::Up, Self::Right) | (Self::Right, Self::Up) => Self::A,
            _ => None?,
        })
    }
}

impl From<Dir> for usize {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 3,
            Dir::A => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    codes: Vec<Vec<Num>>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let codes = text
            .lines()
            .map(|line| {
                line.bytes()
                    .map(Num::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { codes })
    }
}
