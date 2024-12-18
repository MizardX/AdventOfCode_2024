use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 18");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 22)", part_1(&example, 7, 12));
    println!("|'-Part 2: {} (expected 6,1)", part_2(&example, 7));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 318)", part_1(&input, 71, 1024));
    println!("|'-Part 2: {} (expected 56,29)", part_2(&input, 71));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input, size: u8, first: usize) -> usize {
    let blocked: HashSet<_> = input.byte_locations[..first].iter().copied().collect();
    shortest_distance(&blocked, size).unwrap_or(0)
}

#[must_use]
fn shortest_distance(blocked: &HashSet<(u8, u8)>, size: u8) -> Option<usize> {
    let start = (0_u8, 0_u8);
    let goal = (size - 1, size - 1);
    let width = size;
    let height = size;
    // A* search
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    open_set.push(Reverse((0, start)));
    g_score.insert(start, 0);
    f_score.insert(start, manhattan_distance(start, goal));
    while let Some(Reverse((_, current))) = open_set.pop() {
        if current == goal {
            return Some(*g_score.get(&current).unwrap());
        }
        for neighbor in neighbors(current, blocked, width, height) {
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(
                    neighbor,
                    tentative_g_score + manhattan_distance(neighbor, goal),
                );
                open_set.push(Reverse((f_score[&neighbor], neighbor)));
            }
        }
    }
    None
}

fn manhattan_distance((x1, y1): (u8, u8), (x2, y2): (u8, u8)) -> usize {
    x1.abs_diff(x2) as usize + y1.abs_diff(y2) as usize
}

fn neighbors(
    (x, y): (u8, u8),
    blocked: &HashSet<(u8, u8)>,
    width: u8,
    height: u8,
) -> impl Iterator<Item = (u8, u8)> + '_ {
    [
        (x.saturating_sub(1), y, x > 0),
        (x + 1, y, x + 1 < width),
        (x, y.saturating_sub(1), y > 0),
        (x, y + 1, y + 1 < height),
    ]
    .into_iter()
    .filter_map(move |(x, y, valid)| {
        if valid && !blocked.contains(&(x, y)) {
            Some((x, y))
        } else {
            None
        }
    })
}

#[must_use]
pub fn part_2(input: &Input, size: u8) -> String {
    let mut lo = 0;
    let mut hi = input.byte_locations.len();
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if shortest_distance(
            &input.byte_locations[..mid].iter().copied().collect(),
            size,
        ).is_some() {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let (x, y) = input.byte_locations[lo];
    format!("{x},{y}")
}

#[derive(Debug, Clone)]
pub struct Input {
    byte_locations: Vec<(u8, u8)>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error("Invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseIntError),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let byte_locations = text
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split_once(',')
                    .ok_or(ParseInputError::MissingDelimiter)?;
                Ok((x.parse()?, y.parse()?))
            })
            .collect::<Result<_, Self::Err>>()?;
        Ok(Self { byte_locations })
    }
}
