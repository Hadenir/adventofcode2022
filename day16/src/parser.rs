use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{
        complete::{digit1, line_ending},
        streaming::space1,
    },
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Valve> {
    let (_, vertices) = valve_list(input).expect("Failed to parse puzzle input");
    vertices
}

fn integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn name(input: &str) -> IResult<&str, &str> {
    take(2usize)(input)
}

fn name_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(tag(", "), name)(input)
}

fn valve(input: &str) -> IResult<&str, Valve> {
    map(
        tuple((
            preceded(tag("Valve "), name),
            delimited(tag(" has flow rate="), integer, tag("; ")),
            preceded(
                tuple((
                    alt((tag("tunnels lead"), tag("tunnel leads"))),
                    tag(" to "),
                    alt((tag("valves"), tag("valve"))),
                    space1,
                )),
                name_list,
            ),
        )),
        |(name, flow_rate, adjacencies)| Valve::new(name, flow_rate, adjacencies),
    )(input)
}

fn valve_list(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list0(line_ending, valve)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let input = "123";

        let (rem, int) = integer(input).unwrap();

        assert_eq!(int, 123);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_name() {
        let input = "AB";

        let (rem, name) = name(input).unwrap();

        assert_eq!(name, "AB");
        assert!(rem.is_empty());
    }

    #[test]
    fn test_name_list() {
        let input = "AB, CD, EF";

        let (rem, names) = name_list(input).unwrap();

        assert_eq!(names, vec!["AB", "CD", "EF"]);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_valve() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";

        let (rem, vert) = valve(input).unwrap();

        assert_eq!(vert, Valve::new("AA", 0, vec!["DD", "II", "BB"]));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_valve_list() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA";

        let (rem, verts) = valve_list(input).unwrap();

        assert_eq!(
            verts,
            vec![
                Valve::new("AA", 0, vec!["DD", "II", "BB"]),
                Valve::new("BB", 13, vec!["CC", "AA"]),
            ]
        );
        assert!(rem.is_empty());
    }
}
