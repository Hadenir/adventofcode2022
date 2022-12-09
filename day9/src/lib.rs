use std::{ops::{Sub, AddAssign}, str::FromStr, collections::HashSet};

mod part1;
mod part2;

pub use part1::solve_part_1;
pub use part2::solve_part_2;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    count: u8,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Invalid direction"),
        };

        Ok(dir)
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let dir_str = split.next().ok_or("No direction")?;
        let count_str = split.next().ok_or("No count")?;

        let direction = dir_str.parse()?;
        let count = count_str.parse().map_err(|_| "Invalid count")?;

        Ok(Move { direction, count })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 -= 1,
            Direction::Right => self.0 += 1,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
