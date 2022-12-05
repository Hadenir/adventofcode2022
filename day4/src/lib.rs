use std::{ops::RangeInclusive, str::FromStr};

struct Pair(RangeInclusive<u32>, RangeInclusive<u32>);

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let ranges = line
            .split(',')
            .map(|pair_str| {
                let mut split = pair_str.split('-');
                let start_str = split.next().ok_or("Invalid pair start")?.parse().unwrap();
                let end_str = split.next().ok_or("Invalid pair end")?.parse().unwrap();
                Ok(start_str..=end_str)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut r = ranges.into_iter();
        Ok(Pair(r.next().unwrap(), r.next().unwrap()))
    }
}

impl Pair {
    fn check_contains(&self) -> bool {
        let (s1, e1) = self.0.clone().into_inner();
        let (s2, e2) = self.1.clone().into_inner();
        s1 <= s2 && e1 >= e2 || s1 >= s2 && e1 <= e2
    }

    fn check_overlap(&self) -> bool {
        let (s1, e1) = self.0.clone().into_inner();
        let (s2, e2) = self.1.clone().into_inner();
        s1.max(s2) <= e1.min(e2)
    }
}

pub fn solve_part_1(input: &str) -> i32 {
    input.lines().map(|line| line.parse().unwrap()).filter(Pair::check_contains).count() as i32
}

pub fn solve_part_2(input: &str) -> i32 {
    input.lines().map(|line| line.parse().unwrap()).filter(Pair::check_overlap).count() as i32
}
