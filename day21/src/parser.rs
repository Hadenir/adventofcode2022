use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{anychar, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    Finish, IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    let (_, monkeys) = monkey_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    monkeys
        .into_iter()
        .map(|monkey| (monkey.identifier, monkey))
        .collect()
}

fn integer(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take(4usize)(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map_res(anychar, TryInto::try_into)(input)
}

fn number_yell(input: &str) -> IResult<&str, Yell> {
    map(integer, Yell::Number)(input)
}

fn operation_yell(input: &str) -> IResult<&str, Yell> {
    map(
        tuple((identifier, delimited(space1, operation, space1), identifier)),
        |(left, operation, right)| Yell::Operation {
            left,
            right,
            operation,
        },
    )(input)
}

fn yell(input: &str) -> IResult<&str, Yell> {
    alt((number_yell, operation_yell))(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        separated_pair(identifier, tag(": "), yell),
        |(identifier, yell)| Monkey { identifier, yell },
    )(input)
}

fn monkey_list(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(line_ending, monkey)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let input = "12";

        let (rem, int) = integer(input).unwrap();

        assert_eq!(int, 12);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_identifier() {
        let input = "brzk";

        let (rem, ident) = identifier(input).unwrap();

        assert_eq!(ident, "brzk");
        assert!(rem.is_empty());
    }

    #[test]
    fn test_operation() {
        let input = "+-*/";

        let (input, op0) = operation(input).unwrap();
        let (input, op1) = operation(input).unwrap();
        let (input, op2) = operation(input).unwrap();
        let (rem, op3) = operation(input).unwrap();

        assert_eq!(op0, Operation::Add);
        assert_eq!(op1, Operation::Sub);
        assert_eq!(op2, Operation::Mul);
        assert_eq!(op3, Operation::Div);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_yell() {
        let input = "abcd - efgh";

        let (rem, yell) = yell(input).unwrap();

        assert!(matches!(
            yell,
            Yell::Operation {
                left: "abcd",
                right: "efgh",
                operation: Operation::Sub,
            }
        ));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_monkey() {
        let input = "root: pppw + sjmn";

        let (rem, monkey) = monkey(input).unwrap();

        assert_eq!(monkey.identifier, "root");
        assert!(matches!(
            monkey.yell,
            Yell::Operation {
                left: "pppw",
                right: "sjmn",
                operation: Operation::Add,
            }
        ));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_monkey_list() {
        let input = "dbpl: 5
zczc: 2";

        let (rem, monkeys) = monkey_list(input).unwrap();

        assert_eq!(monkeys.len(), 2);
        assert_eq!(monkeys[0].identifier, "dbpl");
        assert_eq!(monkeys[1].identifier, "zczc");
        assert!(matches!(monkeys[0].yell, Yell::Number(5)));
        assert!(matches!(monkeys[1].yell, Yell::Number(2)));
        assert!(rem.is_empty());
    }
}
