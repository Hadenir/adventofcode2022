use nom::{
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    sequence::{delimited, pair, tuple},
    IResult, multi::separated_list0,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Point> {
    let (_, droplet) = droplet(input).expect("Failed to parse puzzle input");
    droplet
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), str::parse)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        tuple((integer, delimited(char(','), integer, char(',')), integer)),
        |(x, y, z)| Point::new(x, y, z),
    )(input)
}

fn droplet(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list0(line_ending, point)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let input = "-123";

        let (rem, int) = integer(input).unwrap();

        assert_eq!(int, -123);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_point() {
        let input = "1,2,3";

        let (rem, point) = point(input).unwrap();

        assert_eq!(point, Point::new(1, 2, 3));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_droplet() {
        let input = "1,2,3
4,5,6";

        let (rem, droplet) = droplet(input).unwrap();

        assert_eq!(droplet, vec![Point::new(1, 2, 3), Point::new(4, 5, 6)]);
        assert!(rem.is_empty());
    }
}
