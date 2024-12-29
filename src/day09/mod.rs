use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::str::FromStr;
use thiserror::Error;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");
// const ANTE1: &str = include_str!("ante1.txt");
// const ANTE2: &str = include_str!("ante2.txt");

pub fn run() {
    println!(".Day 09");

    println!("++Example");
    let example = EXAMPLE.parse().expect("Parse example");
    println!("|+-Part 1: {} (expected 1_928)", part_1(&example));
    println!("|'-Part 2: {} (expected 2_858)", part_2(&example));

    println!("++Input");
    let input = INPUT.parse().expect("Parse input");
    println!("|+-Part 1: {} (expected 6_344_673_854_800)", part_1(&input));
    println!("|'-Part 2: {} (expected 6_360_363_199_987)", part_2(&input));

    // println!("++Ante 1");
    // let ante1 = ANTE1.parse().expect("Parse ante 1");
    // println!("|+-Part 1: {} (expected 44_652_698_743_984)", part_1(&ante1));
    // println!("|'-Part 2: {} (expected 97_898_222_299_196)", part_2(&ante1));

    // println!("++Ante 2");
    // let ante2 = ANTE2.parse().expect("Parse ante 2");
    // println!("|+-Part 1: {} (expected 226_884_355_354_768)", part_1(&ante2));
    // println!("|'-Part 2: {} (expected 5_799_706_413_896_802)", part_2(&ante2));
    println!("')");
}

#[must_use]
pub fn part_1(input: &Input) -> u64 {
    let defragmented = input.compact();
    let mut sum = 0;
    for FileEntry { id, pos, size } in defragmented {
        // sum of pos..pos+size
        sum += u64::from((pos * 2 + size - 1) * size / 2 * id);
    }
    sum
}

#[must_use]
pub fn part_2(input: &Input) -> u64 {
    let mut positioned: BTreeMap<u32, FileEntry> = BTreeMap::new();
    let mut empties: BTreeMap<u32, EmptyEntry> = BTreeMap::new();
    for &entry in &input.entries {
        match entry {
            Entry::File(file) => {
                positioned.insert(file.pos, file);
            }
            Entry::Empty(empty) => {
                empties.insert(empty.pos, empty);
            }
        }
    }
    for &entry in input.entries.iter().rev() {
        if let Entry::File(file) = entry {
            if let Some((_, &empty)) = empties
                .range(..file.pos)
                .find(|(_, &empty)| empty.size >= file.size)
            {
                empties.remove(&empty.pos);
                if empty.size > file.size {
                    let new_empty = EmptyEntry {
                        pos: empty.pos + file.size,
                        size: empty.size - file.size,
                    };
                    empties.insert(new_empty.pos, new_empty);
                }
                positioned.remove(&file.pos);
                positioned.insert(empty.pos, file.with_pos(empty.pos));
                let mut pos = file.pos;
                let mut size = file.size;
                if let Some(prev_empty) = empties
                    .range(..file.pos)
                    .next_back()
                    .and_then(|(_, &e)| (e.pos + e.size == file.pos).then_some(e))
                {
                    empties.remove(&prev_empty.pos);
                    pos = prev_empty.pos;
                    size += prev_empty.size;
                }
                if let Some(next_empty) = empties
                    .range(file.pos..)
                    .next()
                    .and_then(|(_, &e)| (file.pos + file.size == e.pos).then_some(e))
                {
                    empties.remove(&next_empty.pos);
                    size += next_empty.size;
                }
                empties.insert(pos, EmptyEntry { pos, size });
            }
        }
    }
    let mut sum = 0;
    for &FileEntry { id, pos, size } in positioned.values() {
        // sum of pos..pos+size
        let pos = u64::from(pos);
        let size = u64::from(size);
        let id = u64::from(id);
        sum += (pos * 2 + size - 1) * size / 2 * id;
    }
    sum
}

#[derive(Clone, Copy)]
pub struct FileEntry {
    pub id: u32,
    pub pos: u32,
    pub size: u32,
}

impl FileEntry {
    const fn with_pos(self, pos: u32) -> Self {
        Self { pos, ..self }
    }
}

impl Debug for FileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { id, pos, size } = self;
        write!(f, "{pos}:{size}#{id}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EmptyEntry {
    pub pos: u32,
    pub size: u32,
}

impl Debug for EmptyEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { pos, size } = self;
        write!(f, "{pos}:{size}")
    }
}

#[derive(Clone, Copy)]
pub enum Entry {
    File(FileEntry),
    Empty(EmptyEntry),
}

impl Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(file) => write!(f, "{file:?}"),
            Self::Empty(empty) => write!(f, "{empty:?}"),
        }
    }
}

impl Entry {
    const fn new_file(id: u32, pos: u32, size: u32) -> Self {
        Self::File(FileEntry { id, pos, size })
    }

    const fn new_empty(pos: u32, size: u32) -> Self {
        Self::Empty(EmptyEntry { pos, size })
    }

    const fn with_size(self, size: u32) -> Self {
        match self {
            Self::File(FileEntry { id, pos, .. }) => Self::new_file(id, pos, size),
            Self::Empty(EmptyEntry { pos, .. }) => Self::new_empty(pos, size),
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
    pos: u32,
    completed: bool,
}

impl<'a> Compact<'a> {
    fn new(input: &'a Input) -> Self {
        let head = 0;
        let tail = input.entries.len() - 1;
        let head_item = input.entries[head];
        let tail_item = input.entries[tail];
        let pos = 0;
        let completed = head >= tail;
        Self {
            input,
            head,
            tail,
            head_item,
            tail_item,
            pos,
            completed,
        }
    }
}

impl FusedIterator for Compact<'_> {}
impl Iterator for Compact<'_> {
    type Item = FileEntry;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.completed {
            if self.head >= self.tail {
                self.completed = true;
                return match self.tail_item {
                    Entry::File(file) => {
                        let file = file.with_pos(self.pos);
                        self.pos += file.size;
                        Some(file)
                    }
                    Entry::Empty(_) => None,
                };
            }
            match (self.head_item, self.tail_item) {
                (Entry::File(file), _) => {
                    let item = file.with_pos(self.pos);
                    self.pos += item.size;
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
                    Entry::Empty(EmptyEntry {
                        size: head_size, ..
                    }),
                    Entry::File(FileEntry {
                        id,
                        size: tail_size,
                        ..
                    }),
                ) => {
                    let new_size = head_size.min(tail_size);
                    let new_pos = self.pos;
                    self.pos += new_size;
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
                    return Some(FileEntry {
                        id,
                        pos: new_pos,
                        size: new_size,
                    });
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
        let mut pos = 0;
        let mut entries = Vec::new();
        for (i, ch) in text.bytes().enumerate() {
            match ch {
                b'0'..=b'9' => {
                    let size = u32::from(ch - b'0');
                    if (i & 1) == 0 {
                        entries.push(Entry::new_file(id, pos, size));
                        id += 1;
                    } else if size > 0 {
                        entries.push(Entry::new_empty(pos, size));
                    }
                    pos += size;
                }
                _ => return Err(ParseInputError::InvalidChar(ch as char)),
            }
        }
        Ok(Self { entries })
    }
}
