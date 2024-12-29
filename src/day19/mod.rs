use std::collections::VecDeque;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 19");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    let (part1, part2) = part_1_and_2(&example);
    println!("|+-Part 1: {part1} (expected 6)");
    println!("|'-Part 2: {part2} (expected 16)");

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    let (part1, part2) = part_1_and_2(&input);
    println!("|+-Part 1: {part1} (expected 293)");
    println!("|'-Part 2: {part2} (expected 623_924_810_770_264)");
    println!("')");
}

#[must_use]
pub fn part_1_and_2(input: &Input) -> (u64, u64) {
    let mut matcher = AhoCorasick::new();
    for piece in &input.pieces {
        matcher.add_pattern(piece);
    }
    matcher.build_links();
    let mut count_matches = 0;
    let mut sum_counts = 0;
    for pattern in &input.target_patterns {
        let count = matcher.count_combinations(pattern);
        sum_counts += count;
        count_matches += u64::from(0 != count);
    }
    (count_matches, sum_counts)
}

#[derive(Clone, Debug)]
struct ACNode {
    children: [Option<usize>; 5],
    suffix_link: usize,
    output_link: usize,
    output: Vec<usize>,
}

impl ACNode {
    const fn new() -> Self {
        Self {
            children: [None; 5],
            suffix_link: 0,
            output_link: 0,
            output: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
struct AhoCorasick {
    nodes: Vec<ACNode>,
}

impl AhoCorasick {
    fn new() -> Self {
        Self {
            nodes: vec![ACNode::new(); 1],
        }
    }

    fn add_pattern(&mut self, pattern: &Pattern) {
        let mut node = 0;
        for &color in &pattern.colors {
            let color = usize::from(color);
            if let Some(next) = self.nodes[node].children[color] {
                node = next;
            } else {
                let next = self.nodes.len();
                self.nodes.push(ACNode::new());
                self.nodes[node].children[color] = Some(next);
                node = next;
            }
        }
        self.nodes[node].output.push(pattern.colors.len());
    }

    fn build_links(&mut self) {
        let mut queue = VecDeque::new();
        for color in Color::all() {
            let color = usize::from(color);
            if let &Some(child) = &self.nodes[0].children[color] {
                queue.push_back(child);
            }
        }
        while let Some(node) = queue.pop_front() {
            for color in Color::all() {
                let color_ix = usize::from(color);
                if let &Some(child) = &self.nodes[node].children[color_ix] {
                    let mut link = self.nodes[node].suffix_link;
                    while link != 0 && self.nodes[link].children[color_ix].is_none() {
                        link = self.nodes[link].suffix_link;
                    }
                    if let Some(next) = self.nodes[link].children[color_ix] {
                        self.nodes[child].suffix_link = next;
                    }
                    queue.push_back(child);
                }
            }
            self.nodes[node].output_link = self.nodes[node].suffix_link;
        }
    }

    fn count_combinations(&self, text: &Pattern) -> u64 {
        let mut node = 0;
        let mut result = vec![0; text.colors.len() + 1];
        result[0] = 1;
        for (i, &color) in text.colors.iter().enumerate() {
            let color = usize::from(color);
            while node != 0 && self.nodes[node].children[color].is_none() {
                node = self.nodes[node].suffix_link;
            }
            if let Some(next) = self.nodes[node].children[color] {
                node = next;
            }
            let mut link = node;
            while link != 0 {
                for &len in &self.nodes[link].output {
                    if i + 1 >= len {
                        result[i + 1] += result[i + 1 - len];
                    }
                }
                link = self.nodes[link].output_link;
            }
        }
        result[text.colors.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    pub const  fn all() -> [Self; 5] {
        [
            Color::White,
            Color::Blue,
            Color::Black,
            Color::Red,
            Color::Green,
        ]
    }
}

impl From<Color> for usize {
    fn from(color: Color) -> usize {
        match color {
            Color::White => 0,
            Color::Blue => 1,
            Color::Black => 2,
            Color::Red => 3,
            Color::Green => 4,
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = ParseInputError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'w' => Ok(Self::White),
            b'u' => Ok(Self::Blue),
            b'b' => Ok(Self::Black),
            b'r' => Ok(Self::Red),
            b'g' => Ok(Self::Green),
            _ => Err(ParseInputError::InvalidChar(value as char)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pattern {
    colors: Vec<Color>,
}

impl FromStr for Pattern {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s
            .trim()
            .bytes()
            .map(Color::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { colors })
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pieces: Vec<Pattern>,
    target_patterns: Vec<Pattern>,
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Incomplete input")]
    IncompleteInput,
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();
        let pieces = lines
            .next()
            .ok_or(ParseInputError::IncompleteInput)?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if lines.next() != Some("") {
            return Err(ParseInputError::IncompleteInput);
        }
        let target_patterns = lines.map(str::parse).collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            pieces,
            target_patterns,
        })
    }
}
