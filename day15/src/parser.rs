use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Sensor> {
    let (_, sensors) = sensor_list(input).expect("Failed to parse puzzle input");
    sensors
}

fn integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), str::parse)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(
            preceded(tag("x="), integer),
            tag(", "),
            preceded(tag("y="), integer),
        ),
        |(x, y)| Point::new(x, y),
    )(input)
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    map(
        separated_pair(
            preceded(tag("Sensor at "), point),
            tag(": "),
            preceded(tag("closest beacon is at "), point),
        ),
        |(sensor_pos, beacon_pos)| Sensor::new(sensor_pos, beacon_pos),
    )(input)
}

fn sensor_list(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(line_ending, sensor)(input)
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
        let input = "x=-2, y=3";

        let (rem, point) = point(input).unwrap();

        assert_eq!(point, Point::new(-2, 3));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_sensor() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        let (rem, sensor) = sensor(input).unwrap();

        assert_eq!(sensor.position, Point::new(2, 18));
        assert_eq!(sensor.closest_beacon, Point::new(-2, 15));
        assert!(rem.is_empty());
    }

    #[test]
    fn test_sensor_list() {
        let input = "Sensor at x=-1, y=2: closest beacon is at x=3, y=4
Sensor at x=5, y=6: closest beacon is at x=7, y=8";

        let (rem, sensors) = sensor_list(input).unwrap();

        assert_eq!(sensors.len(), 2);
        assert_eq!(sensors[0].position, Point::new(-1, 2));
        assert_eq!(sensors[0].closest_beacon, Point::new(3, 4));
        assert_eq!(sensors[1].position, Point::new(5, 6));
        assert_eq!(sensors[1].closest_beacon, Point::new(7, 8));
        assert!(rem.is_empty());
    }
}
