use std::{fmt::Debug, iter::FusedIterator, str::FromStr};
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

pub fn run() {
    println!(".Day 09");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 1_928)", part_1(&example));
    println!("|'-Part 2: {} (expected XXX)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6_344_673_854_800)", part_1(&input));
    println!("|'-Part 2: {} (expected XXX)", part_2(&input));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    let defragmented = input.compact();
    let mut pos = 0;
    let mut sum = 0;
    for entry in defragmented {
        match entry {
            Entry::File(FileEntry { id, size }) => {
                // sum of pos..pos+size
                sum += u64::from((pos * 2 + size - 1) * size / 2 * id);
                pos += size;
            }
            Entry::Empty(_) => unreachable!(),
        }
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> usize {
    let _ = input;
    0
}

#[derive(Clone, Copy, Debug)]
pub struct FileEntry {
    pub id: u32,
    pub size: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct EmptyEntry {
    pub size: u32,
}

#[derive(Clone, Copy)]
pub enum Entry {
    File(FileEntry),
    Empty(EmptyEntry),
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(FileEntry { id, size }) => write!(f, "{size}#{id}"),
            Self::Empty(EmptyEntry { size }) => write!(f, "{size}"),
        }
    }
}

impl Entry {
    fn new_file(id: u32, size: u32) -> Self {
        Self::File(FileEntry { id, size })
    }

    fn with_size(self, size: u32) -> Self {
        match self {
            Self::File(FileEntry { id, .. }) => Entry::File(FileEntry { id, size }),
            Self::Empty(EmptyEntry { .. }) => Entry::Empty(EmptyEntry { size }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    entries: Vec<Entry>,
}

impl Input {
    #[must_use]
    pub fn compact(&self) -> Compact<'_> {
        Compact::new(self)
    }
}

pub struct Compact<'a> {
    input: &'a Input,
    head: usize,
    tail: usize,
    head_item: Entry,
    tail_item: Entry,
    completed: bool,
}

impl<'a> Compact<'a> {
    fn new(input: &'a Input) -> Self {
        let head = 0;
        let tail = input.entries.len() - 1;
        let head_item = input.entries[head];
        let tail_item = input.entries[tail];
        let completed = head >= tail;
        Self {
            input,
            head,
            tail,
            head_item,
            tail_item,
            completed,
        }
    }
}

impl FusedIterator for Compact<'_> {}
impl Iterator for Compact<'_> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.completed {
            if self.head >= self.tail {
                self.completed = true;
                return Some(self.tail_item);
            }
            match (self.head_item, self.tail_item) {
                (Entry::File(_), _) => {
                    let item = self.head_item;
                    self.head += 1;
                    self.head_item = self.input.entries[self.head];
                    if self.head >= self.tail {
                        self.completed = true;
                    }
                    return Some(item);
                }
                (_, Entry::Empty(_)) => {
                    self.tail -= 1;
                    self.tail_item = self.input.entries[self.tail];
                    if self.head >= self.tail {
                        self.completed = true;
                    }
                    // loop
                }
                (
                    Entry::Empty(EmptyEntry { size: head_size }),
                    Entry::File(FileEntry {
                        id,
                        size: tail_size,
                    }),
                ) => {
                    let new_size = head_size.min(tail_size);
                    if head_size <= tail_size {
                        self.head += 1;
                        self.head_item = self.input.entries[self.head];
                    } else {
                        self.head_item = self.head_item.with_size(head_size - tail_size);
                    }
                    if head_size >= tail_size {
                        self.tail -= 1;
                        self.tail_item = self.input.entries[self.tail];
                    } else {
                        self.tail_item = self.tail_item.with_size(tail_size - head_size);
                    }
                    return Some(Entry::new_file(id, new_size));
                }
            }
        }
        None
    }
}

#[derive(Debug, Error)]
pub enum ParseInputError {
    #[error("Unexpected character: '{0}'")]
    InvalidChar(char),
}

impl FromStr for Input {
    type Err = ParseInputError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut entries = Vec::new();
        for (i, ch) in text.bytes().enumerate() {
            match ch {
                b'0'..=b'9' => {
                    let size = u32::from(ch - b'0');
                    if (i & 1) == 0 {
                        entries.push(Entry::File(FileEntry { id, size }));
                        id += 1;
                    } else {
                        entries.push(Entry::Empty(EmptyEntry { size }));
                    }
                }
                _ => return Err(ParseInputError::InvalidChar(ch as char)),
            }
        }
        Ok(Self { entries })
    }
}
