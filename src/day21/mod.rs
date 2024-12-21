use std::hash::Hash;
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
    let operator_stack = NumRobot(DirRobot(DirRobot(Human)));
    for code in &input.codes {
        let (cost, path) = operator_stack.cost_of_full_code(code);
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
        println!("code {code:?} cost {cost} num {num} path {path:?}");
        sum += cost * num;
    }
    sum
}

trait Operator {
    type Item;
    fn cost_of_move(&self, from: &Self::Item, to: &Self::Item, path: &mut Vec<Dir>) -> usize;

    fn cost_of_full_code(&self, code: &[Self::Item]) -> (usize, Vec<Dir>) {
        let mut path = Vec::new();
        let total_cost = code
            .windows(2)
            .map(|pair| self.cost_of_move(&pair[0], &pair[1], &mut path))
            .sum();
        (total_cost, path)
    }
}

struct Human;
impl Operator for Human {
    type Item = Dir;
    fn cost_of_move(&self, _from: &Dir, &to: &Dir, path: &mut Vec<Dir>) -> usize {
        path.push(to);
        1
    }
}

struct DirRobot<T>(T);
impl<T> Operator for DirRobot<T>
where
    T: Operator<Item = Dir>,
{
    type Item = Dir;
    fn cost_of_move(&self, &from: &Dir, &to: &Dir, path: &mut Vec<Dir>) -> usize {
        let mut stack = Vec::new();
        let mut distance = [(); 5].map(|()| (usize::MAX, Dir::A, vec![]));
        stack.push((0, 0, from, Dir::A, vec![]));
        while let Some((full_dist, dist, key, inner, path)) = stack.pop() {
            if full_dist >= distance[key as usize].0 {
                continue;
            }
            distance[key as usize].0 = full_dist;
            distance[key as usize].1 = inner;
            for (dir, next) in key.neighbors() {
                let mut new_path = path.clone();
                let move_cost = self.0.cost_of_move(&inner, &dir, &mut new_path);
                let activate_cost = self.0.cost_of_move(&dir, &Dir::A, &mut vec![]);
                stack.push((
                    dist + move_cost + activate_cost,
                    dist + move_cost,
                    next,
                    dir,
                    new_path,
                ));
            }
            distance[key as usize].2 = path;
        }
        let &(total_cost, last_inner, ref new_path) = &distance[usize::from(to)];
        path.extend_from_slice(new_path);
        self.0.cost_of_move(&last_inner, &Dir::A, path);
        total_cost
    }
}

struct NumRobot<T>(T);
impl<T> Operator for NumRobot<T>
where
    T: Operator<Item = Dir>,
{
    type Item = Num;
    fn cost_of_move(&self, &from: &Num, &to: &Num, path: &mut Vec<Dir>) -> usize {
        let mut stack = Vec::new();
        let mut distance = [(); 11].map(|()| (usize::MAX, Dir::A, vec![]));
        stack.push((0, 0, from, Dir::A, vec![]));
        while let Some((full_dist, dist, key, inner, path)) = stack.pop() {
            if full_dist >= distance[key as usize].0 {
                continue;
            }
            distance[key as usize].0 = full_dist;
            distance[key as usize].1 = inner;
            for (dir, next) in key.neighbors() {
                let mut new_path = path.clone();
                let move_cost = self.0.cost_of_move(&inner, &dir, &mut new_path);
                let activate_cost = self.0.cost_of_move(&dir, &Dir::A, &mut vec![]);
                stack.push((
                    dist + move_cost + activate_cost,
                    dist + move_cost,
                    next,
                    dir,
                    new_path,
                ));
            }
            distance[key as usize].2 = path;
        }
        let &(total_cost, last_inner, ref new_path) = &distance[usize::from(to)];
        path.extend_from_slice(new_path);
        self.0.cost_of_move(&last_inner, &Dir::A, path);
        total_cost
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
