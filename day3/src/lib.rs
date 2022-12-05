use std::collections::BTreeSet;

struct Rucksack {
    first_comp: String,
    second_comp: String,
}

impl Rucksack {
    fn new(contents: &str) -> Self {
        let half_len = contents.len() / 2;


        Self {
            first_comp: contents[..half_len].to_string(),
            second_comp: contents[half_len..].to_string(),
        }
    }

    fn common(&self) -> String {
        self.first_comp.chars()
            .filter(|ch| self.second_comp.contains(*ch))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    fn whole(&self) -> String {
        self.first_comp.to_owned() + &self.second_comp
    }
}

fn priority(ch: char) -> i32 {
    if ('a'..='z').contains(&ch) {
        (ch as i32) - ('a' as i32) + 1
    } else if ('A'..='Z').contains(&ch) {
        (ch as i32) - ('A' as i32) + 27
    } else {
        panic!("Invalid item in sack")
    }
}

fn badge(sacks: &[Rucksack]) -> char {
    if let [s1, s2, s3] = sacks {
        let s1 = s1.whole();
        let s2 = s2.whole();
        let s3 = s3.whole();
        s1.chars()
            .filter(|ch| s2.contains(*ch) && s3.contains(*ch))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .next()
            .unwrap()
    } else {
        panic!("Invalid number of sacks");
    }
}

pub fn solve_part_1(input: &str) -> i32 {
    let sucksacks: Vec<Rucksack> = input.lines().map(Rucksack::new).collect();

    sucksacks.iter()
        .flat_map(|sack| sack.common()
            .chars()
            .map(priority)
            .collect::<Vec<i32>>())
        .sum()
}

pub fn solve_part_2(input: &str) -> i32 {
    let sucksacks: Vec<Rucksack> = input.lines().map(Rucksack::new).collect();

    sucksacks.chunks(3)
        .map(badge)
        .map(priority)
        .sum()
}
