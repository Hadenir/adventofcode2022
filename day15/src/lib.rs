mod parser;

use std::collections::HashSet;

use parser::parse_input;

pub fn solve_part_1(input: &str, query_y: i32) -> usize {
    let sensors = parse_input(input);

    let (x_min, x_max) = sensors
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), sensor| {
            (
                sensor.position.x.min(sensor.closest_beacon.x).min(min),
                sensor.position.x.max(sensor.closest_beacon.x).max(max),
            )
        });
    let max_dist = sensors
        .iter()
        .fold(i32::MIN, |max_dist, sensor| sensor.distance.max(max_dist));

    let mut count = 0;
    'outer: for x in (x_min - max_dist)..=(x_max + max_dist) {
        let p = Point::new(x, query_y);
        for sensor in sensors.iter() {
            if p == sensor.position || p == sensor.closest_beacon {
                continue;
            }

            let dist = manhattan(p, sensor.position);
            if dist <= sensor.distance {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}

pub fn solve_part_2(input: &str) -> u128 {
    let sensors = parse_input(input);

    let mut beacon = None;
    'outer: for (i, sensor) in sensors.iter().enumerate() {
        println!("Sensor {i}...");

        let sensor_x = sensor.position.x;
        let sensor_y = sensor.position.y;
        let dist = sensor.distance + 1;
        for x in (sensor_x - dist)..=(sensor_x + dist) {
            let h = dist - (x - sensor_x);
            for y in HashSet::from([(sensor_y - h), (sensor_y + h)]) {
                let point = Point::new(x, y);

                let points = [
                    Point::new(point.x - 1, point.y),
                    Point::new(point.x + 1, point.y),
                    Point::new(point.x, point.y - 1),
                    Point::new(point.x, point.y + 1),
                ];

                if points.into_iter().all(|p| {
                    sensors
                        .iter()
                        .any(|s| s.distance >= manhattan(s.position, p))
                }) && sensors
                    .iter()
                    .all(|s| s.distance < manhattan(s.position, point))
                {
                    beacon = Some(point);
                    break 'outer;
                }
            }
        }
    }

    let p = beacon.expect("Failed to find location for beacon");
    4000000u128 * p.x as u128 + p.y as u128
}

fn manhattan(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
    distance: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        Self {
            position,
            closest_beacon,
            distance: manhattan(position, closest_beacon),
        }
    }
}
