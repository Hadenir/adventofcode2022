use nom::{
    branch::alt,
    character::complete::{char, alpha1, line_ending, digit1},
    combinator::map,
    sequence::{delimited, tuple, preceded, terminated},
    IResult, bytes::complete::tag, multi::{separated_list1, many1},
};

use crate::*;

pub fn parse_input(input: &str) -> (Vec<CrateStack>, Vec<Move>) {
    tuple((
        terminated(crate_stacks, stack_numbers),
        moves_list
    ))(input).unwrap().1
}

fn crate_box(input: &str) -> IResult<&str, Option<Crate>> {
    map(
        alt((
            tag("   "),
            delimited(char('['), alpha1, char(']'))
        )),
        |s| match s {
            "   " => None,
            ch => Some(Crate(ch.chars().next().unwrap()))
        },
    )(input)
}

fn crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(
        char(' '),
        crate_box
    )(input)
}

fn crate_stacks(input: &str) -> IResult<&str, Vec<CrateStack>> {
    let (input, lines) = separated_list1(line_ending, crate_line)(input)?;

    let mut stacks: Vec<CrateStack> = vec![vec![]; lines.last().unwrap().len()];
    for line in lines.into_iter().rev() {
        for (i, crt) in line.into_iter().enumerate() {
            if let Some(crt) = crt {
                stacks[i].push(crt)
            }
        }
    }

    Ok((input, stacks))
}

fn stack_numbers(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((
        line_ending,
        char(' '),
        separated_list1(
            tag("   "),
            digit1
        ),
        many1(line_ending)
    ))(input)?;

    Ok((input, ()))
}

fn single_move(input: &str) -> IResult<&str, Move> {
    map(
        tuple((
            preceded(tag("move "), digit1),
            preceded(tag(" from "), digit1),
            preceded(tag(" to "), digit1),
        )),
        |(count, from, to): (&str, &str, &str)| Move {
            count: count.parse().unwrap(),
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    )(input)
}

fn moves_list(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, single_move)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_box() {
        let (rem, crt) = crate_box("[A]").unwrap();

        assert_eq!(crt, Some(Crate('A')));
        assert_eq!(rem, "");

        let (rem, crt) = crate_box("    ").unwrap();

        assert_eq!(crt, None);
        assert_eq!(rem, " ");
    }

    #[test]
    fn test_crate_line() {
        let (rem, line) = crate_line("[A] [B] [C]").unwrap();

        assert_eq!(line, vec![Some(Crate('A')), Some(Crate('B')), Some(Crate('C'))]);
        assert_eq!(rem, "");

        let (rem, line) = crate_line("    [D]     ").unwrap();

        assert_eq!(line, vec![None, Some(Crate('D')), None]);
        assert_eq!(rem, " ");
    }

    #[test]
    fn test_crate_stacks() {
        let (rem, stacks) = crate_stacks("[A] [B] [C]").unwrap();

        assert_eq!(stacks, vec![vec![Crate('A')], vec![Crate('B')], vec![Crate('C')]]);
        assert_eq!(rem, "");

        let (rem, stacks) = crate_stacks("    [A]    \n\
[B] [C]    \n\
[D] [E] [F]\n\
").unwrap();

        assert_eq!(stacks, vec![vec![Crate('D'), Crate('B')], vec![Crate('E'), Crate('C'), Crate('A')], vec![Crate('F')]]);
        assert_eq!(rem, "\n");
    }

    #[test]
    fn test_stack_numbers() {
        let (rem, _) = stack_numbers("\n 1   2   3   4\n\nmove").unwrap();

        assert_eq!(rem, "move");
    }

    #[test]
    fn test_single_move() {
        let (rem, mov) = single_move("move 1 from 2 to 3").unwrap();

        assert_eq!(mov, Move { count: 1, from: 2, to: 3 });
        assert_eq!(rem, "");

        let (rem, mov) = single_move("move 1 from 2 to 3\n").unwrap();

        assert_eq!(mov, Move { count: 1, from: 2, to: 3 });
        assert_eq!(rem, "\n");
    }

    #[test]
    fn test_moves_list() {
        let (rem, moves) = moves_list("move 1 from 2 to 3
move 3 from 2 to 1
move 2 from 2 to 2").unwrap();

        assert_eq!(moves, vec![
            Move { count: 1, from: 2, to: 3 },
            Move { count: 3, from: 2, to: 1 },
            Move { count: 2, from: 2, to: 2 },
        ]);
        assert_eq!(rem, "");
    }

    #[test]
    fn test_parse_input() {
        let input = r"[A] [B] [C]
 1   2   3

move 1 from 2 to 3";

        let (stacks, moves) = parse_input(input);
        assert_eq!(stacks, vec![vec![Crate('A')], vec![Crate('B')], vec![Crate('C')]]);
        assert_eq!(moves, vec![Move { count: 1, from: 2, to: 3 }]);
    }
}
