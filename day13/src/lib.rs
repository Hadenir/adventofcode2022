mod parser;

use std::cmp::Ordering;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> usize {
    let packet_pairs = parse_input(input);

    packet_pairs
        .into_iter()
        .enumerate()
        .filter(|(_, (p1, p2))| p1 < p2)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let packet_pairs = parse_input(input);

    let p1 = Packet::new_list(vec![Packet::new_list(vec![Packet::new_int(2)])]);
    let p2 = Packet::new_list(vec![Packet::new_list(vec![Packet::new_int(6)])]);

    let mut packets: Vec<Packet> = packet_pairs.into_iter()
        .flat_map(|(p1, p2)| [p1, p2])
        .chain([p1.clone(), p2.clone()])
        .collect();

    packets.sort();

    let idx1 = packets.iter().position(|p| p == &p1).map(|i| i + 1).expect("Couldn't find first decoder key");
    let idx2 = packets.iter().position(|p| p == &p2).map(|i| i + 1).expect("Couldn't find second decoder key");

    idx1 * idx2
}

type Int = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(Int),
    List(Vec<Packet>),
}

impl Packet {
    fn new_int(int: Int) -> Self {
        Self::Integer(int)
    }

    fn new_list(list: Vec<Self>) -> Self {
        Self::List(list)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Packet::*;

        match (self, other) {
            (Integer(ref x), Integer(ref y)) => x.partial_cmp(y),
            (List(ref packets), List(ref other_packets)) => {
                let ord = packets
                    .iter()
                    .zip(other_packets)
                    .fold(None, |ord, (p1, p2)| {
                        ord.or(match p1.cmp(p2) {
                            Ordering::Less => Some(Ordering::Less),
                            Ordering::Equal => None,
                            Ordering::Greater => Some(Ordering::Greater),
                        })
                    });

                ord.or_else(|| packets.len().partial_cmp(&other_packets.len()))
            }
            (List(_), Integer(_)) => self.partial_cmp(&Packet::new_list(vec![other.clone()])),
            (Integer(_), List(_)) => Packet::new_list(vec![self.clone()]).partial_cmp(other),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
