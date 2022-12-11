use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, digit1, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::*;

pub fn parse_input(input: &str) -> (Worry, Vec<Monkey>) {
    let (_, mut result) =
        separated_list0(line_ending, monkey)(input).expect("Failed to parse input");

    result.sort_by_key(|(id, _, _)| *id);
    let (divs, monkeys): (Vec<u64>, Vec<Monkey>) = result.into_iter().map(|(_, div, monkey)| (div, monkey)).unzip();
    (divs.iter().product(), monkeys)
}

fn starting_items(input: &str) -> IResult<&str, Vec<Item>> {
    let (input, items) = preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), digit1),
    )(input)?;

    let items = items
        .into_iter()
        .map(|item| item.parse().unwrap())
        .map(|worry| Item { worry })
        .collect();

    Ok((input, items))
}

fn operation(input: &str) -> IResult<&str, OperationFn> {
    preceded(tag("  Operation: "), operation_fn)(input)
}

fn operation_fn(input: &str) -> IResult<&str, OperationFn> {
    map(
        preceded(
            tag("new = old "),
            separated_pair(anychar, tag(" "), alphanumeric1),
        ),
        get_operation_fn,
    )(input)
}

fn get_operation_fn((op, val_str): (char, &str)) -> OperationFn {
    let val: Option<Worry> = val_str.parse().ok();

    match (op, val_str, val) {
        ('+', "old", _) => Box::new(move |old| old + old),
        ('+', _, Some(v)) => Box::new(move |old| old + v),
        ('*', "old", _) => Box::new(move |old| old * old),
        ('*', _, Some(v)) => Box::new(move |old| old * v),
        _ => {
            panic!("Invalid operation encountered!")
        }
    }
}

fn test(input: &str) -> IResult<&str, (Worry, TestFn)> {
    let (input, (div_str, if_true, if_false)) = tuple((
        preceded(tag("  Test: divisible by "), digit1),
        preceded(
            preceded(line_ending, tag("    If true: throw to monkey ")),
            digit1,
        ),
        preceded(
            preceded(line_ending, tag("    If false: throw to monkey ")),
            digit1,
        ),
    ))(input)?;

    let div: Worry = div_str.parse().unwrap();
    let if_true: usize = if_true.parse().unwrap();
    let if_false: usize = if_false.parse().unwrap();

    Ok((
        input,
        (
            div,
            Box::new(
                move |worry| {
                    if worry % div == 0 {
                        if_true
                    } else {
                        if_false
                    }
                },
            ),
        ),
    ))
}

fn monkey(input: &str) -> IResult<&str, (usize, Worry, Monkey)> {
    let (input, id) = delimited(tag("Monkey "), digit1, terminated(tag(":"), line_ending))(input)?;

    let id: usize = id.parse().unwrap();

    let (input, items) = terminated(starting_items, line_ending)(input)?;
    let (input, operation) = terminated(operation, line_ending)(input)?;
    let (input, (worry, test)) = terminated(test, line_ending)(input)?;

    Ok((
        input,
        (
            id,
            worry,
            Monkey {
                items,
                operation,
                test,
            },
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_items() {
        let input = "  Starting items: 1, 2, 12, 13";

        let (rem, items) = starting_items(input).unwrap();

        assert_eq!(
            items,
            vec![
                Item { worry: 1 },
                Item { worry: 2 },
                Item { worry: 12 },
                Item { worry: 13 }
            ]
        );
        assert_eq!(rem, "");
    }

    #[test]
    fn test_operation_fn() {
        let input = "  Operation: new = old + 1";

        let (rem, op) = operation(input).unwrap();

        let new = op(2);
        assert_eq!(new, 3);
        assert_eq!(rem, "");

        let input = "  Operation: new = old + old";

        let (rem, op) = operation(input).unwrap();

        let new = op(2);
        assert_eq!(new, 4);
        assert_eq!(rem, "");
    }

    #[test]
    fn test_test_fn() {
        let input = "  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 2";

        let (rem, (div, test_fn)) = test(input).unwrap();

        assert_eq!(test_fn(&6), 1);
        assert_eq!(test_fn(&2), 2);
        assert_eq!(div, 3);
        assert_eq!(rem, "");
    }
}
