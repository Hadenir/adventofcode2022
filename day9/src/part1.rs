use super::*;

pub fn solve_part_1(input: &str) -> usize {
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse input"))
        .collect();

    let mut bridge = Bridge::new();
    let mut visited = HashSet::new();
    visited.insert(bridge.tail);
    for mov in moves {
        for _ in 0..mov.count {
            bridge.head.mov(mov.direction);

            let Point(x_offset, y_offset) = bridge.head - bridge.tail;
            if x_offset.abs() >= 2 {
                bridge.tail.mov(
                    if x_offset > 0 { Direction::Right } else { Direction::Left }
                );

                if y_offset.abs() >= 1 {
                    bridge.tail.mov(
                        if y_offset > 0 { Direction::Up } else { Direction::Down }
                    );
                }
            } else if y_offset.abs() >= 2 {
                bridge.tail.mov(
                    if y_offset > 0 { Direction::Up } else { Direction::Down }
                );

                if x_offset.abs() >= 1 {
                    bridge.tail.mov(
                        if x_offset > 0 { Direction::Right } else { Direction::Left }
                    );
                }
            }

            visited.insert(bridge.tail);
        }
    }

    visited.len()
}

struct Bridge {
    head: Point,
    tail: Point,
}

impl Bridge {
    fn new() -> Self {
        Self::default()
    }
}

impl Default for Bridge {
    fn default() -> Self {
        Self {
            head: Point(0, 0),
            tail: Point(0, 0),
        }
    }
}
