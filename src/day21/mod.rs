use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::str::FromStr;
use std::vec;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
// const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 21");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 126_384)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    // println!("++Input");
    // let input = INPUT.parse().expect("Parse input");
    // println!("|+-Part 1: {} (expected XXX)", part_1(&input));
    // println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    // println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let mut sum = 0;
    let operator_stack = NumRobot(DirRobot(DirRobot(Human))); //NumRobot(DirRobot(DirRobot(Human)));
    for code in &input.codes {
        let mut prev = Num::A;
        let mut full_path = Vec::new();
        for &num in code {
            let path = operator_stack.shortest_path(prev, num);
            full_path.extend_from_slice(&path);
            prev = num;
        }
        let cost = full_path.len();
        let num = code.iter().fold(0, |acc, &num| {
            acc * 10
                + match num {
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
                    Num::A => return acc,
                }
        });
        println!("code {code:?} cost {cost} num {num} path {full_path:?}");
        sum += cost * num;
    }
    sum
}

#[derive(Debug, Clone)]
struct PathNode<T> {
    cost: usize,
    position: T,
    inner: Dir,
    path: Vec<Dir>,
}
impl<T> PathNode<T> {
    fn new(cost: usize, position: T, inner: Dir) -> Self {
        Self {
            cost,
            position,
            inner,
            path: Vec::new(),
        }
    }

    fn create_next(&self, position: T, more_path: &[Dir]) -> Self {
        let mut path = self.path.clone();
        path.extend_from_slice(more_path);
        let inner = more_path.last().copied().unwrap_or(self.inner);
        Self {
            cost: self.cost + more_path.len(),
            position,
            inner,
            path,
        }
    }
}
impl<T> PartialEq for PathNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<T> Eq for PathNode<T> {}
impl<T> Ord for PathNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl<T> PartialOrd for PathNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait Operator {
    type Item: Copy;

    fn shortest_path(&self, from: Self::Item, to: Self::Item) -> Vec<Dir>;
}

struct Human;
impl Operator for Human {
    type Item = Dir;
    fn shortest_path(&self, _from: Self::Item, to: Self::Item) -> Vec<Dir> {
        vec![to]
    }
}

struct DirRobot<T>(T);
impl<T> Operator for DirRobot<T>
where
    T: Operator<Item = Dir>,
{
    type Item = Dir;
    fn shortest_path(&self, from: Self::Item, to: Self::Item) -> Vec<Dir> {
        let mut shortest_path = Vec::new();
        let mut stack = BinaryHeap::new();
        stack.push(PathNode::new(0, from, Dir::A));
        let mut seen = HashMap::new();
        while let Some(node) = stack.pop() {
            if let Some(&cost) = seen.get(&(node.position, node.inner)) {
                if node.cost >= cost {
                    continue;
                }
            }
            seen.insert((node.position, node.inner), node.cost);
            if !shortest_path.is_empty() && node.path.len() >= shortest_path.len() {
                continue;
            }
            if node.position == to {
                let new_node = node.create_next(to, &self.0.shortest_path(node.inner, Dir::A));
                if shortest_path.is_empty() || new_node.path.len() < shortest_path.len() {
                    shortest_path = new_node.path;
                }
                continue;
            }
            for (dir, next) in node.position.neighbors() {
                let new_node = node.create_next(next, &self.0.shortest_path(node.inner, dir));
                stack.push(new_node);
            }
        }
        assert!(
            !shortest_path.is_empty(),
            "No path found from {from:?} to {to:?}"
        );
        shortest_path
    }
}

struct NumRobot<T>(T);
impl<T> Operator for NumRobot<T>
where
    T: Operator<Item = Dir>,
{
    type Item = Num;
    fn shortest_path(&self, from: Self::Item, to: Self::Item) -> Vec<Dir> {
        let mut shortest_path = Vec::new();
        let mut stack = BinaryHeap::new();
        stack.push(PathNode::new(0, from, Dir::A));
        let mut seen = HashMap::new();
        while let Some(node) = stack.pop() {
            if let Some(&cost) = seen.get(&(node.position, node.inner)) {
                if node.cost >= cost {
                    continue;
                }
            }
            seen.insert((node.position, node.inner), node.cost);
            if !shortest_path.is_empty() && node.path.len() >= shortest_path.len() {
                continue;
            }
            if node.position == to {
                let new_node = node.create_next(to, &self.0.shortest_path(node.inner, Dir::A));
                if shortest_path.is_empty() || new_node.path.len() < shortest_path.len() {
                    shortest_path = new_node.path;
                }
                continue;
            }
            for (dir, next) in node.position.neighbors() {
                match dir {
                    Dir::Up if to.position().1 > node.position.position().1 => {
                        continue;
                    }
                    Dir::Down if to.position().1 < node.position.position().1 => {
                        continue;
                    }
                    Dir::Left if to.position().0 > node.position.position().0 => {
                        continue;
                    }
                    Dir::Right if to.position().0 < node.position.position().0 => {
                        continue;
                    }
                    _ => (),
                }
                let new_node = node.create_next(next, &self.0.shortest_path(node.inner, dir));
                stack.push(new_node);
            }
        }
        assert!(
            !shortest_path.is_empty(),
            "No path found from {from:?} to {to:?}"
        );
        shortest_path
    }
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

trait Keypad: Sized {
    fn all() -> impl IntoIterator<Item = Self>;
    fn move_finger(&self, dir: Dir) -> Option<Self>;
    fn position(self) -> (usize, usize);

    fn neighbors(&self) -> impl IntoIterator<Item = (Dir, Self)> {
        Dir::all()
            .into_iter()
            .filter_map(|dir| Some((dir, self.move_finger(dir)?)))
    }
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

impl Keypad for Num {
    fn all() -> impl IntoIterator<Item = Self> {
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

    fn position(self) -> (usize, usize) {
        match self {
            Self::Zero => (1, 3),
            Self::A => (2, 3),

            Self::One => (0, 2),
            Self::Two => (1, 2),
            Self::Three => (2, 2),

            Self::Four => (0, 1),
            Self::Five => (1, 1),
            Self::Six => (2, 1),

            Self::Seven => (0, 0),
            Self::Eight => (1, 0),
            Self::Nine => (2, 0),
        }
    }

    fn move_finger(&self, dir: Dir) -> Option<Self> {
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

impl Keypad for Dir {
    fn all() -> impl std::iter::IntoIterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right, Self::A]
    }

    fn position(self) -> (usize, usize) {
        match self {
            Self::Up => (1, 0),
            Self::Down => (1, 1),
            Self::Left => (0, 1),
            Self::Right => (2, 1),
            Self::A => (2, 0),
        }
    }

    fn move_finger(&self, dir: Dir) -> Option<Self> {
        Some(match (self, dir) {
            (Self::Down, Dir::Up) | (Self::A, Dir::Left) => Self::Up,
            (Self::Up, Dir::Down) | (Self::Left, Dir::Right) | (Self::Right, Dir::Left) => {
                Self::Down
            }
            (Self::Down, Dir::Left) => Self::Left,
            (Self::Down, Dir::Right) | (Self::A, Dir::Down) => Self::Right,
            (Self::Up, Dir::Right) | (Self::Right, Dir::Up) => Self::A,
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
