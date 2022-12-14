use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, line_ending},
    combinator::{map, map_res},
    sequence::{separated_pair, delimited},
    IResult, multi::separated_list1,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Cave {
    let (_, rock_paths) = path_list(input).expect("Failed to parse puzzle input");

    let rocks: HashSet<Point> = rock_paths
        .into_iter()
        .flat_map(create_path)
        .collect();

    Cave::new(rocks.into_iter().collect())
}

fn create_path(path_points: Vec<Point>) -> Vec<Point> {
    path_points
        .windows(2)
        .flat_map(|points| {
            let start = points[0];
            let end = points[1];

            if start.x == end.x {
                let x = start.x;
                let sy = start.y.min(end.y);
                let ey = start.y.max(end.y);
                (sy..=ey).into_iter()
                    .map(|y| Point::new(x, y))
                    .collect::<Vec<Point>>()
            } else if start.y == end.y {
                let y = start.y;
                let sx = start.x.min(end.x);
                let ex = start.x.max(end.x);
                (sx..=ex).into_iter()
                    .map(|x| Point::new(x, y))
                    .collect::<Vec<Point>>()
            } else {
                panic!("Path is not a straight line");
            }
        })
        .collect()
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(integer, tag(","), integer),
        |(x, y)| Point::new(x, y)
    )(input)
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(
        delimited(multispace0, tag("->"), multispace0),
        point
    )(input)
}

fn path_list(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(
        line_ending,
        path
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let input = "1234";

        let (rem, int) = integer(input).unwrap();

        assert_eq!(int, 1234);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_point() {
        let input = "12,34";

        let (rem, point) = point(input).unwrap();

        assert_eq!(point, Point::new(12, 34));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_path() {
        let input = "12,34 -> 34,56 -> 56,78";

        let (rem, path) = path(input).unwrap();

        assert_eq!(path, vec![Point::new(12, 34), Point::new(34, 56), Point::new(56, 78)]);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_path_list() {
        let input = "12,34 -> 34,56
98,76 -> 76,54";

        let (rem, paths) = path_list(input).unwrap();

        assert_eq!(
            paths,
            vec![
                vec![Point::new(12, 34), Point::new(34, 56)],
                vec![Point::new(98, 76), Point::new(76, 54)],
            ]
        );
        assert!(rem.is_empty());
    }
}
