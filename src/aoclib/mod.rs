#![allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        let data = vec![Default::default(); width * height];
        Self {
            data,
            width,
            height,
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_signed(&self, x: isize, y: isize) -> Option<&T> {
        if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
            self.get(x, y)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = value;
        }
    }

    pub fn row(&self, y: usize) -> Option<&[T]> {
        if y < self.height {
            Some(&self.data[y * self.width..(y + 1) * self.width])
        } else {
            None
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(move |y| &self.data[y * self.width..(y + 1) * self.width])
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        for line in text.lines() {
            width = width.max(line.len());
            height += 1;
            for c in line.bytes() {
                data.push(c.try_into()?);
            }
        }
        Ok(Self {
            data,
            width,
            height,
        })
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
