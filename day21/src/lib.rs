mod parser;

use std::collections::HashMap;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> i64 {
    let monkeys = parse_input(input);

    let monkeys = solve(monkeys);

    monkeys["root"]
        .yell
        .value()
        .expect("Failed to calculate root value")
}

pub fn solve_part_2(input: &str) -> i64 {
    let monkeys = parse_input(input);

    let (left_ident, right_ident) = if let Yell::Operation { left, right, .. } = monkeys["root"].yell {
        (left, right)
    } else {
        unreachable!();
    };

    let mut num = 0;
    loop {
        let mut monkeys = monkeys.clone();
        monkeys.entry("humn").and_modify(|humn| humn.yell = Yell::Number(num));
        let monkeys = solve(monkeys);

        let left_val = monkeys[left_ident].yell.value().unwrap();
        let right_val = monkeys[right_ident].yell.value().unwrap();

        if left_val == right_val {
            return num;
        }

        // A bit hacky, but works.
        let delta = left_val - right_val;
        if delta.abs() > 10000 {
            num += delta / 10000;
        } else {
            num += delta.signum();
        }
    }
}

fn solve<'a>(mut monkeys: HashMap<&'a str, Monkey<'a>>) -> HashMap<&'a str, Monkey<'a>> {
    let mut stack = vec!["root"];
    while let Some(ident) = stack.pop() {
        let monkey = &monkeys[ident];

        if let Yell::Operation {
            left,
            right,
            operation,
        } = monkey.yell {
            let left_val = monkeys[left].yell.value();
            let right_val = monkeys[right].yell.value();

            if left_val.is_none() || right_val.is_none() {
                stack.push(ident);
            }

            if left_val.is_none() {
                stack.push(left);
            }

            if right_val.is_none() {
                stack.push(right);
            }

            if let Some(left_val) = left_val {
                if let Some(right_val) = right_val {
                    monkeys.entry(ident).and_modify(|monkey| {
                        monkey.yell = Yell::Number(operation.perform(left_val, right_val))
                    });
                }
            }
        }
    }

    monkeys
}

#[derive(Debug, Clone)]
struct Monkey<'a> {
    identifier: &'a str,
    yell: Yell<'a>,
}

#[derive(Debug, Clone)]
enum Yell<'a> {
    Number(i64),
    Operation {
        left: &'a str,
        right: &'a str,
        operation: Operation,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl<'a> Yell<'a> {
    fn value(&self) -> Option<i64> {
        use Yell::*;

        match self {
            &Number(num) => Some(num),
            Operation { .. } => None,
        }
    }
}

impl Operation {
    fn perform(&self, left: i64, right: i64) -> i64 {
        use Operation::*;

        match self {
            Add => left + right,
            Sub => left - right,
            Mul => left * right,
            Div => left / right,
        }
    }
}

impl TryFrom<char> for Operation {
    type Error = &'static str;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        use Operation::*;

        match ch {
            '+' => Ok(Add),
            '-' => Ok(Sub),
            '*' => Ok(Mul),
            '/' => Ok(Div),
            _ => Err("Invalid operation"),
        }
    }
}
