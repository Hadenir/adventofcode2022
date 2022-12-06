pub mod parser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Crate(char);

pub type CrateStack = Vec<Crate>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn solve_part_1(mut stacks: Vec<CrateStack>, moves: Vec<Move>) -> String {
    for mov in moves {
        let from = mov.from - 1;
        let to = mov.to - 1;
        for _ in 0..mov.count {
            let crt = stacks[from].pop().expect("Encountered empty stack");
            stacks[to].push(crt);
        }
    }

    stacks.iter()
        .fold(String::new(), |acc, stack| {
            format!("{}{}", acc, stack.last().expect("Encountered empty stack").0)
        })
}

pub fn solve_part_2(mut stacks: Vec<CrateStack>, moves: Vec<Move>) -> String {
    for mov in moves {
        let from = mov.from - 1;
        let to = mov.to - 1;

        let src_stack = &mut stacks[from];
        let crates = src_stack.split_off(src_stack.len() - mov.count);
        stacks[to].extend(crates);
    }

    stacks.iter()
        .fold(String::new(), |acc, stack| {
            format!("{}{}", acc, stack.last().expect("Encountered empty stack").0)
        })
}
