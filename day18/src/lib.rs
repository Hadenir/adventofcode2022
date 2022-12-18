mod parser;

use std::ops::Add;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> usize {
    let lava_droplet = parse_input(input);

    lava_droplet
        .iter()
        .map(|cube| {
            let dps = [
                Point::new(-1, 0, 0),
                Point::new(1, 0, 0),
                Point::new(0, -1, 0),
                Point::new(0, 1, 0),
                Point::new(0, 0, -1),
                Point::new(0, 0, 1),
            ];

            dps.into_iter()
                .filter(|dp| !lava_droplet.contains(&(*cube + *dp)))
                .count()
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let lava_droplet = parse_input(input);

    let (min_p, max_p) = bounding_box(&lava_droplet);
    let min_p = min_p + Point::new(-1, -1, -1);
    let max_p = max_p + Point::new(1, 1, 1);

    let mut stack = vec![min_p];
    let mut outside = Vec::new();
    while let Some(cube) = stack.pop() {
        if lava_droplet.contains(&cube) || outside.contains(&cube) {
            continue;
        }

        if cube.x > min_p.x {
            stack.push(cube + Point::new(-1, 0, 0));
        }
        if cube.x < max_p.x {
            stack.push(cube + Point::new(1, 0, 0));
        }
        if cube.y > min_p.y {
            stack.push(cube + Point::new(0, -1, 0));
        }
        if cube.y < max_p.y {
            stack.push(cube + Point::new(0, 1, 0));
        }
        if cube.z > min_p.z {
            stack.push(cube + Point::new(0, 0, -1));
        }
        if cube.z < max_p.z {
            stack.push(cube + Point::new(0, 0, 1));
        }

        outside.push(cube);
    }

    outside.into_iter()
        .map(|cube| {
            let dps = [
                Point::new(-1, 0, 0),
                Point::new(1, 0, 0),
                Point::new(0, -1, 0),
                Point::new(0, 1, 0),
                Point::new(0, 0, -1),
                Point::new(0, 0, 1),
            ];

            dps.into_iter()
                .filter(|dp| lava_droplet.contains(&(cube + *dp)))
                .count()
        })
        .sum()
}

fn bounding_box(points: &[Point]) -> (Point, Point) {
    points.iter().fold(
        (
            Point::new(i32::MAX, i32::MAX, i32::MAX),
            Point::new(i32::MIN, i32::MIN, i32::MIN),
        ),
        |(min, max), point| (point.min(&min), point.max(&max)),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
