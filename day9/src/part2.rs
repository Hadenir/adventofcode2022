use super::*;

pub fn solve_part_2(input: &str) -> usize {
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse input"))
        .collect();

    let mut bridge = Bridge::new(10);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(*bridge.tail_mut());
    for mov in moves {
        for _ in 0..mov.count {
            bridge.move_head(mov.direction);
            visited.insert(*bridge.tail_mut());
        }
    }

    visited.len()
}

#[derive(Debug)]
struct Bridge {
    knots: Vec<Point>,
}

impl Bridge {
    fn new(len: usize) -> Self {
        Self {
            knots: vec![Point(0, 0); len],
        }
    }

    fn head_mut(&mut self) -> &mut Point {
        self.knots.first_mut().expect("Bridge has no head")
    }

    fn tail_mut(&mut self) -> &mut Point {
        self.knots.last_mut().expect("Bridge has no tail")
    }

    /// Moves head in specified direction, then rest of the knots according to the rules.
    fn move_head(&mut self, dir: Direction) {
        self.head_mut().mov(dir);

        let indices_vec = Vec::from_iter(0..self.knots.len());
        for indices in indices_vec.windows(2) {
            let prev_knot = self.knots[indices[0]];
            let curr_knot = self.knots[indices[1]];
            let Point(x_offset, y_offset) = prev_knot - curr_knot;

            let curr_knot = &mut self.knots[indices[1]];
            if x_offset.abs() >= 2 {
                curr_knot.mov(
                    if x_offset > 0 { Direction::Right } else { Direction::Left }
                );

                if y_offset.abs() >= 1 {
                    curr_knot.mov(
                        if y_offset > 0 { Direction::Up } else { Direction::Down }
                    );
                }
            } else if y_offset.abs() >= 2 {
                curr_knot.mov(
                    if y_offset > 0 { Direction::Up } else { Direction::Down }
                );

                if x_offset.abs() >= 1 {
                    curr_knot.mov(
                        if x_offset > 0 { Direction::Right } else { Direction::Left }
                    );
                }
            }
        }
    }
}
