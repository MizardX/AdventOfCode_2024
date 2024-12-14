use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 14");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 12)", part_1(&example, (11, 7)));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!(
        "|+-Part 1: {} (expected 221_142_636)",
        part_1(&input, (101, 103))
    );
    println!("|'-Part 2: {} (expected 7_916)", part_2(&input, (101, 103)));
    println!("')");
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn part_1(input: &Input, room_size: (i32, i32)) -> usize {
    let mut sums = [0; 4];
    let time = 100;
    for robot in &input.robots {
        if let Some(q) = robot.quadrant(time, room_size) {
            sums[q] += 1;
        }
    }
    sums.iter().copied().product()
}

#[must_use]
pub fn part_2(input: &Input, room_size: (i32, i32)) -> i32 {
    let mut sum_var_x = 0;
    let mut sum_var_y = 0;
    for time in 0.. {
        let var_x = input.variance_x(time, room_size);
        let var_y = input.variance_y(time, room_size);
        sum_var_x += var_x;
        sum_var_y += var_y;
        if time > 10 && var_x < sum_var_x/(2*time + 1) && var_y < sum_var_y/(2*time + 1) {
            return time;
        }
    }
    unreachable!()
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn wrap(&self, time: i32, room_size: (i32, i32)) -> (i32, i32) {
        let (px, py) = self.position;
        let (vx, vy) = self.velocity;
        let (w, h) = room_size;
        let new_x = px + vx * time;
        let new_y = py + vy * time;
        let bounded_x = new_x.rem_euclid(w);
        let bounded_y = new_y.rem_euclid(h);
        (bounded_x, bounded_y)
    }

    fn quadrant(&self, time: i32, room_size: (i32, i32)) -> Option<usize> {
        let (px, py) = self.wrap(time, room_size);
        let (w, h) = room_size;
        Some(if px < w / 2 && py < h / 2 {
            0
        } else if px > w / 2 && py < h / 2 {
            1
        } else if px < w / 2 && py > h / 2 {
            2
        } else if px > w / 2 && py > h / 2 {
            3
        } else {
            None?
        })
    }
}

impl FromStr for Robot {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        // p=6,3 v=-1,-3
        let rest = text
            .strip_prefix("p=")
            .ok_or(ParseInputError::InvalidDelimiter)?;
        let (px, rest) = rest
            .split_once(',')
            .ok_or(ParseInputError::InvalidDelimiter)?;
        let (py, rest) = rest
            .split_once(" v=")
            .ok_or(ParseInputError::InvalidDelimiter)?;
        let (vx, vy) = rest
            .split_once(',')
            .ok_or(ParseInputError::InvalidDelimiter)?;
        let position = (px.parse()?, py.parse()?);
        let velocity = (vx.parse()?, vy.parse()?);
        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    robots: Vec<Robot>,
}

impl Input {
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    #[expect(clippy::cast_possible_truncation)]
    pub fn variance_x(&self, time: i32, room_size: (i32, i32)) -> i32 {
        let mut sum = 0;
        let mut sum_squared = 0;
        for robot in &self.robots {
            let (px, _) = robot.wrap(time, room_size);
            sum += px;
            sum_squared += px * px;
        }
        let n = self.robots.len() as i32;
        let mean = sum / n;
        sum_squared / n - mean * mean
    }
    #[must_use]
    #[expect(clippy::cast_possible_wrap)]
    #[expect(clippy::cast_possible_truncation)]
    pub fn variance_y(&self, time: i32, room_size: (i32, i32)) -> i32 {
        let mut sum = 0;
        let mut sum_squared = 0;
        for robot in &self.robots {
            let (_, py) = robot.wrap(time, room_size);
            sum += py;
            sum_squared += py * py;
        }
        let n = self.robots.len() as i32;
        let mean = sum / n;
        sum_squared / n - mean * mean
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    // #[error("Input is empty")]
    // EmptyInput,
    #[error("Invalid delimiter")]
    InvalidDelimiter,
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let robots = text.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { robots })
    }
}
