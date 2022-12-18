use std::ops::AddAssign;

fn parse_input(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|&ch| ch != '\n' && ch != '\r')
        .map(|ch| match ch {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid character encountered in puzzle input"),
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> i64 {
    let jets = parse_input(input);

    let mut rocks_queue = Vec::from([
        Rock::horizontal,
        Rock::plus,
        Rock::l_shape,
        Rock::vertical,
        Rock::sqaure,
    ]).into_iter().cycle();

    let mut num_rocks = 0;
    let mut rocks = vec![rocks_queue.next().unwrap()(Point::new(2, 3))];
    for &jet_dir in jets.iter().cycle() {
        if !check_collision(&rocks, jet_dir) {
            rocks.last_mut().unwrap().push(jet_dir);
        }

        if !check_collision(&rocks, Direction::Down) {
            rocks.last_mut().unwrap().push(Direction::Down);
        } else {
            num_rocks += 1;
            if num_rocks >= 2022 {
                break;
            }

            let height = stack_height(&rocks);
            rocks.push(rocks_queue.next().unwrap()(Point::new(2, height + 3)));
        }
    }

    stack_height(&rocks)
}

pub fn solve_part_2(input: &str) -> i64 {
    todo!()
}

/// Checks whether the last rock in `cave` would collide with other rocks or cave walls
/// if it were to move one tile in `dir` direction.
fn check_collision(cave: &Vec<Rock>, dir: Direction) -> bool {
    let rock = cave.last().expect("Cave has no rocks");

    let delta_p = dir.delta();
    let dx = delta_p.x;
    let dy = delta_p.y;

    let rock_pos = rock.position;
    if rock_pos.x + dx < 0 || rock_pos.x + dx + rock.width > 7 || rock_pos.y + dy < 0 {
        return true;
    }

    if cave.len() < 2 {
        return false;
    }

    cave[0..cave.len() - 1]
        .iter()
        .skip((cave.len() as isize - 40).max(0) as usize)
        .flat_map(|r| &r.tiles)
        .any(|t1| {
            rock.tiles
                .iter()
                .any(|t0| &Point::new(t0.x + dx, t0.y + dy) == t1)
        })
}

fn stack_height(cave: &[Rock]) -> i64 {
    cave.iter()
        .skip((cave.len() as isize - 40).max(0) as usize)
        .flat_map(|rock| &rock.tiles)
        .map(|tile| tile.y)
        .max()
        .expect("Cave is empty") + 1
}

#[allow(unused)]
fn print_cave(cave: &[Rock]) {
    let points: Vec<_> = cave.iter().flat_map(|rock| &rock.tiles).collect();
    let height = points.iter().max_by_key(|p| p.y).unwrap().y;

    for y in (0..=height).rev() {
        print!("|");
        for x in 0..7 {
            let p = Point::new(x, y);
            if points.contains(&&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }

    println!("+-------+\n");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone)]
struct Rock {
    tiles: Vec<Point>,
    position: Point,
    width: i64,
    _height: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[allow(clippy::identity_op)]
impl Rock {
    fn new(tiles: Vec<Point>) -> Self {
        let (x_min, x_max, y_min, y_max) = tiles.iter().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(x_min, x_max, y_min, y_max), tile| {
                (
                    tile.x.min(x_min),
                    tile.x.max(x_max),
                    tile.y.min(y_min),
                    tile.y.max(y_max),
                )
            },
        );

        let width = x_max - x_min + 1;
        let height = y_max - y_min + 1;

        Self {
            tiles,
            position: Point::new(x_min, y_min),
            width,
            _height: height,
        }
    }

    fn horizontal(pos: Point) -> Self {
        Self::new(vec![
            Point::new(pos.x + 0, pos.y),
            Point::new(pos.x + 1, pos.y),
            Point::new(pos.x + 2, pos.y),
            Point::new(pos.x + 3, pos.y),
        ])
    }

    fn vertical(pos: Point) -> Self {
        Self::new(vec![
            Point::new(pos.x, pos.y + 0),
            Point::new(pos.x, pos.y + 1),
            Point::new(pos.x, pos.y + 2),
            Point::new(pos.x, pos.y + 3),
        ])
    }

    fn plus(pos: Point) -> Self {
        Self::new(vec![
            Point::new(pos.x + 1, pos.y + 0),
            Point::new(pos.x + 0, pos.y + 1),
            Point::new(pos.x + 1, pos.y + 1),
            Point::new(pos.x + 2, pos.y + 1),
            Point::new(pos.x + 1, pos.y + 2),
        ])
    }

    fn l_shape(pos: Point) -> Self {
        Self::new(vec![
            Point::new(pos.x + 0, pos.y + 0),
            Point::new(pos.x + 1, pos.y + 0),
            Point::new(pos.x + 2, pos.y + 0),
            Point::new(pos.x + 2, pos.y + 1),
            Point::new(pos.x + 2, pos.y + 2),
        ])
    }

    fn sqaure(pos: Point) -> Self {
        Self::new(vec![
            Point::new(pos.x + 0, pos.y + 0),
            Point::new(pos.x + 1, pos.y + 0),
            Point::new(pos.x + 0, pos.y + 1),
            Point::new(pos.x + 1, pos.y + 1),
        ])
    }

    fn push(&mut self, dir: Direction) {
        let offset = dir.delta();
        self.position += offset;
        for tile in &mut self.tiles {
            *tile += offset;
        }
    }
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
            Direction::Down => Point::new(0, -1),
        }
    }
}
