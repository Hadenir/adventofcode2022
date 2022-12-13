use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair, pair},
    IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let (_, packets) = packet_pairs(input).expect("Failed to parse input");

    packets
}

fn integer(input: &str) -> IResult<&str, Int> {
    map_res(digit1, str::parse)(input)
}

fn int_packet(input: &str) -> IResult<&str, Packet> {
    map(integer, Packet::new_int)(input)
}

fn list(input: &str) -> IResult<&str, Vec<Packet>> {
    delimited(tag("["), separated_list0(tag(","), packet), tag("]"))(input)
}

fn list_packet(input: &str) -> IResult<&str, Packet> {
    map(list, Packet::new_list)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((int_packet, list_packet))(input)
}

fn packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    separated_pair(packet, line_ending, packet)(input)
}

fn packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    separated_list0(pair(line_ending, line_ending), packet_pair)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_packet() {
        let input = "1";

        let (rem, packet) = int_packet(input).unwrap();

        assert_eq!(packet, Packet::new_int(1));
        assert_eq!(rem, "");
    }

    #[test]
    fn test_list_packet() {
        let input = "[1,[]]";

        let (rem, packet) = list_packet(input).unwrap();

        assert_eq!(
            packet,
            Packet::new_list(vec![Packet::new_int(1), Packet::new_list(vec![])])
        );
        assert_eq!(rem, "");
    }

    #[test]
    fn test_packet() {
        let input = "[[],[1],2]";

        let (rem, packet) = packet(input).unwrap();

        assert_eq!(
            packet,
            Packet::new_list(vec![
                Packet::new_list(vec![]),
                Packet::new_list(vec![Packet::new_int(1)]),
                Packet::new_int(2),
            ])
        );
        assert_eq!(rem, "");
    }

    #[test]
    fn test_packet_pair() {
        let input = "[1,2]
[[]]";

        let (rem, pair) = packet_pair(input).unwrap();

        assert_eq!(
            pair.0,
            Packet::new_list(vec![Packet::new_int(1), Packet::new_int(2)])
        );
        assert_eq!(
            pair.1,
            Packet::new_list(vec![Packet::new_list(vec![])])
        );
        assert_eq!(rem, "");
    }
}
