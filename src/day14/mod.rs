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
    let (width, height) = room_size;
    let mut stats_x = Stats::new();
    let mut stats_y = Stats::new();
    let mut outlier_x = None;
    let mut outlier_y = None;
    for time in 0..=7_916 {
        if outlier_x.is_none() {
            let x = input.variance_x(time, room_size);
            if stats_x.is_outlier(x, 8) {
                outlier_x = Some(time);
            }
            stats_x.add(x);
        }
        if outlier_y.is_none() {
            let y = input.variance_y(time, room_size);
            if stats_y.is_outlier(y, 8) {
                outlier_y = Some(time);
            }
            stats_y.add(y);
        }
        if let (Some(outlier_x), Some(outlier_y)) = (outlier_x, outlier_y) {
            // projected_time == outlier_x (mod width)
            // projected_time == outlier_y (mod height)

            let outlier_x = i64::from(outlier_x);
            let outlier_y = i64::from(outlier_y);

            let width = i64::from(width);
            let height = i64::from(height);
            let (gcd, inv_width, inv_height) = egcd(width, height);
            assert!(gcd == 1, "No inverse found");

            let projected_time = (width * outlier_y * inv_width + height * outlier_x * inv_height)
                % (width * height);
            let result = i32::try_from(projected_time).expect("Value out of range for i32");
            return result;
        }
    }
    unreachable!()
}

#[expect(clippy::similar_names)]
fn egcd(m: i64, n: i64) -> (i64, i64, i64) {
    let (mut current, mut gcd) = (m, n);
    let (mut coeff_m1, mut coeff_n1) = (1, 0); // such that coeff_m1 * m + coeff_n1 * n = current
    let (mut coeff_m2, mut coeff_n2) = (0, 1); // such that coeff_m2 * m + coeff_n2 * n = gcd

    let (mut quotient, mut remainder) = (current / gcd, current % gcd);
    while remainder != 0 {
        // (coeff_m1, coeff_n1, coeff_m2, coeff_n2, current, gcd) => (coeff_m2, coeff_n2, coeff_m1 - quotient * coeff_m2, coeff_n1 - quotient * coeff_n2, gcd, remainder)
        let mut temp = coeff_m1;
        coeff_m1 = coeff_m2;
        coeff_m2 = temp - quotient * coeff_m2;

        temp = coeff_n1;
        coeff_n1 = coeff_n2;
        coeff_n2 = temp - quotient * coeff_n2;

        current = gcd;
        gcd = remainder;

        quotient = current / gcd;
        remainder = current % gcd;
    }
    (gcd, coeff_m2, coeff_n2)
}

#[derive(Debug, Clone, Copy)]
struct Stats {
    sum: i64,
    sum_squared: i64,
    count: i64,
}

impl Stats {
    fn new() -> Self {
        Self {
            sum: 0,
            sum_squared: 0,
            count: 0,
        }
    }

    fn add(&mut self, value: i32) {
        let value = i64::from(value);
        self.sum += value;
        self.sum_squared += value * value;
        self.count += 1;
    }

    #[expect(clippy::cast_possible_truncation)]
    fn mean(&self) -> i32 {
        (self.sum / self.count) as i32
    }

    #[expect(clippy::cast_possible_truncation)]
    fn variance(&self) -> i32 {
        let mean = self.sum / self.count;
        (self.sum_squared / self.count - mean * mean) as i32
    }

    fn is_outlier(&self, value: i32, sigma: i32) -> bool {
        if self.count < 2 {
            return false;
        }
        let mean = self.mean();
        let variance = self.variance();
        let dist = (value - mean).abs();
        dist * dist > sigma * sigma * variance
    }
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
    pub fn variance_x(&self, time: i32, room_size: (i32, i32)) -> i32 {
        let mut stats = Stats::new();
        for robot in &self.robots {
            let (px, _) = robot.wrap(time, room_size);
            stats.add(px);
        }
        stats.variance()
    }

    #[must_use]
    pub fn variance_y(&self, time: i32, room_size: (i32, i32)) -> i32 {
        let mut stats = Stats::new();
        for robot in &self.robots {
            let (_, py) = robot.wrap(time, room_size);
            stats.add(py);
        }
        stats.variance()
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
