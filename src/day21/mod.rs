use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;
use std::vec;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 21");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 126_384)", part_1(&example));
    println!(
        "|'-Part 2: {} (expected 154_115_708_116_294)",
        part_2(&example)
    );

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 132_532)", part_1(&input));
    println!(
        "|'-Part 2: {} (expected 165_644_591_859_332)",
        part_2(&input)
    );
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> usize {
    let operator_stack = NumRobot::new(DirRobot::new(DirRobot::new(Human))); //NumRobot(DirRobot(DirRobot(Human)));
    solve(input, operator_stack)
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let operator_stack = NumRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(
        DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(
            DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(
                DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(
                    DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(DirRobot::new(
                        DirRobot::new(Human),
                    ))))),
                ))))),
            ))))),
        ))))),
    )))));
    solve(input, operator_stack)
}

fn solve(input: &Input, mut operator_stack: impl Operator<Item = Num>) -> usize {
    let mut sum = 0;
    for code in &input.codes {
        let mut prev = Num::START;
        let mut full_cost = 0;
        for &num in code {
            let shortest_code = operator_stack.shortest_code(prev, num).unwrap();
            full_cost += shortest_code;
            prev = num;
        }
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
        sum += full_cost * num;
    }
    sum
}

#[derive(Debug, Clone)]
struct Path(Vec<Dir>);
impl Path {
    const fn new() -> Self {
        Self(Vec::new())
    }
    fn push(&mut self, dir: Dir) {
        self.0.push(dir);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for dir in &self.0 {
            match dir {
                Dir::Up => write!(f, "^")?,
                Dir::Down => write!(f, "v")?,
                Dir::Left => write!(f, "<")?,
                Dir::Right => write!(f, ">")?,
                Dir::A => write!(f, "A")?,
            }
        }
        Ok(())
    }
}

trait Operator {
    type Item: Copy + Keypad;

    fn shortest_code(&mut self, from: Self::Item, to: Self::Item) -> Option<usize>;
}

#[derive(Debug, Clone)]
struct Human;
impl Operator for Human {
    type Item = Dir;
    fn shortest_code(&mut self, _from: Self::Item, _to: Self::Item) -> Option<usize> {
        Some(1)
    }
}

type DirRobot<T> = Robot<Dir, T>;
type NumRobot<T> = Robot<Num, T>;

#[derive(Clone)]
struct Robot<K, T> {
    inner: T,
    cache: HashMap<(K, K), Option<usize>>,
}
impl<K, T> Robot<K, T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            cache: HashMap::new(),
        }
    }
}
impl<K: Keypad, T: Debug> Debug for Robot<K, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Robot<{}>({:?})", K::NAME, self.inner)
    }
}
impl<K, T> Operator for Robot<K, T>
where
    T: Operator<Item = Dir> + Debug,
    K: Keypad + Copy + Eq + Hash + Debug,
{
    type Item = K;
    fn shortest_code(&mut self, from: Self::Item, to: Self::Item) -> Option<usize> {
        if let Some(&shortest_code) = self.cache.get(&(from, to)) {
            return shortest_code;
        }
        let paths = K::all_paths(from, to);
        let mut stack = paths
            .iter()
            .map(|path| (Dir::START, path.0.as_slice(), 0))
            .collect::<Vec<_>>();
        let mut shortest_code = None;
        while let Some((prev, path, current_len)) = stack.pop() {
            match path {
                [] => {
                    if shortest_code.is_none_or(|len| len > current_len) {
                        shortest_code = Some(current_len);
                    }
                }
                &[next, ref rest @ ..] => {
                    if let Some(step_len) = self.inner.shortest_code(prev, next) {
                        stack.push((next, rest, current_len + step_len));
                    }
                }
            }
        }
        self.cache.insert((from, to), shortest_code);
        shortest_code
    }
}

trait Keypad: Sized + Copy + Eq + Debug {
    const START: Self;
    const NAME: &'static str;

    fn all() -> impl IntoIterator<Item = Self>;
    fn move_finger(self, dir: Dir) -> Option<Self>;
    fn position(self) -> (usize, usize);

    fn neighbors(&self) -> impl IntoIterator<Item = (Dir, Self)> {
        Dir::all()
            .into_iter()
            .filter_map(|dir| Some((dir, self.move_finger(dir)?)))
    }

    fn all_paths(self, to: Self) -> Vec<Path> {
        let mut all_paths = Vec::<Path>::new();
        let mut stack = vec![(self, Path::new())];
        while let Some((pos, mut path)) = stack.pop() {
            if all_paths.first().is_some_and(|p| p.len() <= path.len()) {
                continue;
            }
            if pos == to {
                path.push(Dir::START);
                if all_paths.first().is_some_and(|f| f.len() > path.len()) {
                    all_paths.clear();
                }
                all_paths.push(path);
                continue;
            }
            for (dir, next) in pos.neighbors() {
                match dir {
                    Dir::Up if to.position().1 >= pos.position().1 => {
                        continue;
                    }
                    Dir::Down if to.position().1 <= pos.position().1 => {
                        continue;
                    }
                    Dir::Left if to.position().0 >= pos.position().0 => {
                        continue;
                    }
                    Dir::Right if to.position().0 <= pos.position().0 => {
                        continue;
                    }
                    _ => (),
                }
                assert!(path.len() <= 5, "Path too long");
                let mut new_path = path.clone();
                new_path.push(dir);
                stack.push((next, new_path));
            }
        }
        all_paths
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
    const START: Self = Self::A;
    const NAME: &'static str = "Num";

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

impl Keypad for Dir {
    const START: Self = Self::A;
    const NAME: &'static str = "Dir";

    fn all() -> impl IntoIterator<Item = Self> {
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

    #[expect(clippy::use_self, reason = "`dir` parameter is fixed type [Dir], and does not vary between impls of the [Keypad] trait")]
    fn move_finger(self, dir: Dir) -> Option<Self> {
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
