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
    println!("|+-Part 1: {} (expected 221_142_636)", part_1(&input, (101, 103)));
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
    for time in 0.. {
        let picture = input.draw_picture(time, room_size);
        let has_5x5_box = picture.windows(5).any(|five_rows| {
            five_rows[0].windows(5).enumerate().any(|(y, row1_five_items)| {
                row1_five_items == b"#####"
                    && &five_rows[1][y..y + 5] == b"#####"
                    && &five_rows[2][y..y + 5] == b"#####"
                    && &five_rows[3][y..y + 5] == b"#####"
                    && &five_rows[4][y..y + 5] == b"#####"
            })
        });
        if has_5x5_box {
            return time;
        }
    }
    0
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
    #[allow(clippy::cast_sign_loss)]
    pub fn draw_picture(&self, time: i32, room_size: (i32, i32)) -> Vec<Vec<u8>> {
        let mut picture = vec![vec![b'.'; room_size.0 as usize]; room_size.1 as usize];
        for robot in &self.robots {
            let (px, py) = robot.wrap(time, room_size);
            picture[py as usize][px as usize] = b'#';
        }
        picture
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_warp() {
        let robot = Robot {
            position: (2, 4),
            velocity: (2, -3),
        };
        let room_size = (11, 7);
        assert_eq!(robot.wrap(0, room_size), (2, 4));
        assert_eq!(robot.wrap(1, room_size), (4, 1));
        assert_eq!(robot.wrap(2, room_size), (6, 5));
        assert_eq!(robot.wrap(3, room_size), (8, 2));
        assert_eq!(robot.wrap(4, room_size), (10, 6));
        assert_eq!(robot.wrap(5, room_size), (1, 3));
    }

    #[test]
    fn test_robots_example_wrap_after_100_seconds() {
        let input: Input = Input {
            robots: vec![
                "p=6,3 v=-1,-3".parse().unwrap(), // (6+(-1)*100, 3+(-3)*100) = (-94, -297) % (11, 7) = (5, 4)
                "p=3,4 v=1,3".parse().unwrap(), // (3+(1)*100, 4+(3)*100) = (103, 304) % (11, 7) = (4, 3)
                "p=4,3 v=0,-1".parse().unwrap(), // (4+(0)*100, 3+(-1)*100) = (4, -97) % (11, 7) = (4, 1)
                "p=5,4 v=1,3".parse().unwrap(), // (5+(1)*100, 4+(3)*100) = (105, 304) % (11, 7) = (6, 3)
                "p=6,3 v=0,-1".parse().unwrap(), // (6+(0)*100, 3+(-1)*100) = (6, -97) % (11, 7) = (6, 1)
                "p=7,4 v=1,3".parse().unwrap(), // (7+(1)*100, 4+(3)*100) = (107, 304) % (11, 7) = (8, 3)
                "p=8,3 v=0,-1".parse().unwrap(), // (8+(0)*100, 3+(-1)*100) = (8, -97) % (11, 7) = (8, 1)
                "p=9,4 v=1,3".parse().unwrap(), // (9+(1)*100, 4+(3)*100) = (109, 304) % (11, 7) = (10, 3)
                "p=10,3 v=0,-1".parse().unwrap(), // (10+(0)*100, 3+(-1)*100) = (10, -97) % (11, 7) = (10, 1)
                "p=1,4 v=1,3".parse().unwrap(), // (1+(1)*100, 4+(3)*100) = (101, 304) % (11, 7) = (2, 3)
                "p=2,3 v=1,-3".parse().unwrap(), // (2+(1)*100, 3+(-3)*100) = (102, -297) % (11, 7) = (3, 4)
            ],
        };
        let room_size = (11, 7);
        let time = 100;
        let expected = vec![
            (5, 4),
            (4, 3),
            (4, 1),
            (6, 3),
            (6, 1),
            (8, 3),
            (8, 1),
            (10, 3),
            (10, 1),
            (2, 3),
            (3, 4),
        ];
        for (robot, (px, py)) in input.robots.iter().zip(expected) {
            assert_eq!(robot.wrap(time, room_size), (px, py));
        }
    }

    #[test]
    fn test_example_quadrants_after_100_seconds() {
        let input: Input = Input {
            // quadrants:
            //   0 => 0..5, 0..3
            //   1 => 6..11, 0..3
            //   2 => 0..5, 4..7
            //   3 => 6..11, 4..7
            //   x = 5 or y = 3 => no quadrant
            robots: vec![
                "p=6,3 v=-1,-3".parse().unwrap(), // (6+(-1)*100, 3+(-3)*100) = (-94, -297) % (11, 7) = (5, 4) => no quadrant
                "p=3,4 v=1,3".parse().unwrap(), // (3+(1)*100, 4+(3)*100) = (103, 304) % (11, 7) = (4, 3) => no quadrant
                "p=4,3 v=0,-1".parse().unwrap(), // (4+(0)*100, 3+(-1)*100) = (4, -97) % (11, 7) = (4, 1) => quadrant 0
                "p=5,4 v=1,3".parse().unwrap(), // (5+(1)*100, 4+(3)*100) = (105, 304) % (11, 7) = (6, 3) => no quadrant
                "p=6,3 v=0,-1".parse().unwrap(), // (6+(0)*100, 3+(-1)*100) = (6, -97) % (11, 7) = (6, 1) => quadrant 1
                "p=7,4 v=1,3".parse().unwrap(), // (7+(1)*100, 4+(3)*100) = (107, 304) % (11, 7) = (8, 3) => no quadrant
                "p=8,3 v=0,-1".parse().unwrap(), // (8+(0)*100, 3+(-1)*100) = (8, -97) % (11, 7) = (8, 1) => quadrant 1
                "p=9,4 v=1,3".parse().unwrap(), // (9+(1)*100, 4+(3)*100) = (109, 304) % (11, 7) = (10, 3) => no quadrant
                "p=10,3 v=0,-1".parse().unwrap(), // (10+(0)*100, 3+(-1)*100) = (10, -97) % (11, 7) = (10, 1) => quadrant 1
                "p=1,4 v=1,3".parse().unwrap(), // (1+(1)*100, 4+(3)*100) = (101, 304) % (11, 7) = (2, 3) => no quadrant
                "p=2,3 v=1,-3".parse().unwrap(), // (2+(1)*100, 3+(-3)*100) = (102, -297) % (11, 7) = (3, 4) => quadrant 2
            ],
        };
        let room_size = (11, 7);
        let time = 100;
        let expected = vec![
            None,
            None,
            Some(0),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(1),
            None,
            Some(2),
        ];
        for (robot, q) in input.robots.iter().zip(expected) {
            assert_eq!(robot.quadrant(time, room_size), q, "{robot:?}");
        }
    }
}
