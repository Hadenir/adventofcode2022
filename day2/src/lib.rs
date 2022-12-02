use std::{str::FromStr, cmp::Ordering};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Invalid move encountered")
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn cheat(&self, outcome: Outcome) -> Self {
        use Move::*;
        use Outcome::*;

        match (self, outcome) {
            (Rock, Win) => Paper,
            (Rock, Draw) => Rock,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Draw) => Paper,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Draw) => Scissors,
            (Scissors, Lose) => Paper,
        }
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Invalid move encountered")
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Move::*;

        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Some(Ordering::Greater),
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => Some(Ordering::Less),
            _ => Some(Ordering::Equal)
        }
    }
}

fn score(opponent: Move, player: Move) -> i32 {
    let mut score = player.score();

    if opponent == player {
        score += 3;
    } else if opponent < player {
        score += 6;
    }

    score
}

pub fn solve_part_1(input: &str) -> i32 {
    let strat: Vec<(Move, Move)> = input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let opponent = split.next().unwrap();
            let player = split.next().unwrap();
            (opponent.parse().unwrap(), player.parse().unwrap())
        })
        .collect();

    let final_score = strat.iter()
        .map(|(opponent, player)| score(*opponent, *player))
        .sum();

    final_score
}

pub fn solve_part_2(input: &str) -> i32 {
    let strat: Vec<(Move, Outcome)> = input.lines()
        .map(|line| {
            let mut split = line.split(' ');
            let opponent = split.next().unwrap();
            let outcome = split.next().unwrap();
            (opponent.parse().unwrap(), outcome.parse().unwrap())
        })
        .collect();

    let final_score = strat.iter()
        .map(|(opponent, outcome)| {
            let player = opponent.cheat(*outcome);
            score(*opponent, player)
        })
        .sum();

    final_score
}
