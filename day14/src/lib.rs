use std::fmt::{Debug, Write};

mod parser;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> usize {
    let mut cave = parse_input(input);
    let sand_spawn = Point::new(500, 0);

    let mut sand = sand_spawn;
    while sand.y as usize <= cave.height - 2 {
        let targets = [
            Point::new(sand.x, sand.y + 1),
            Point::new(sand.x - 1, sand.y + 1),
            Point::new(sand.x + 1, sand.y + 1),
        ];

        if let Some(target_pos) = targets
            .into_iter()
            .find(|pos| cave.get_tile(pos) == &Tile::Air)
        {
            sand = target_pos;
        } else {
            *cave.get_tile_mut(&sand) = Tile::Sand;
            sand = sand_spawn;
        }
    }

    cave.tiles
        .iter()
        .filter(|&&tile| tile == Tile::Sand)
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut cave = parse_input(input);
    let sand_spawn = Point::new(500, 0);

    let mut sand = sand_spawn;
    loop {
        let targets = [
            Point::new(sand.x, sand.y + 1),
            Point::new(sand.x - 1, sand.y + 1),
            Point::new(sand.x + 1, sand.y + 1),
        ];

        if let Some(target_pos) = targets
            .into_iter()
            .find(|pos| cave.height > pos.y as usize && cave.get_tile(pos) == &Tile::Air)
        {
            sand = target_pos;
        } else if sand == sand_spawn {
            break;
        } else {
            *cave.get_tile_mut(&sand) = Tile::Sand;
            sand = sand_spawn;
        }
    }

    cave.tiles
        .iter()
        .filter(|&&tile| tile == Tile::Sand)
        .count() + 1
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

struct Cave {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    left: i32,
}

impl Cave {
    fn new(rocks: Vec<Point>) -> Self {
        let bottom = rocks.iter()
            .fold(0, |bottom, rock| bottom.max(rock.y)) + 2;
        let left = 500 - bottom;
        let right = 500 + bottom;

        let width = (right - left - 1) as usize;
        let height = bottom as usize;
        let mut tiles = vec![Tile::Air; width * height];
        for rock in rocks {
            let x = (rock.x - left - 1) as usize;
            let y = rock.y as usize;
            tiles[x + y * width] = Tile::Rock;
        }

        Self {
            tiles,
            width,
            height,
            left,
        }
    }

    fn get_tile(&self, point: &Point) -> &Tile {
        let x = (point.x - self.left - 1) as usize;
        let y = point.y as usize;
        &self.tiles[x + y * self.width]
    }

    fn get_tile_mut(&mut self, point: &Point) -> &mut Tile {
        let x = (point.x - self.left - 1) as usize;
        let y = point.y as usize;
        &mut self.tiles[x + y * self.width]
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}x{}\n", self.width, self.height))?;
        for y in 0..self.height {
            for x in (self.width/5)..=(self.width*4/5) {
                let ch = match self.tiles[x + y * self.width] {
                    Tile::Air => '.',
                    Tile::Rock => '#',
                    Tile::Sand => 'o',
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
