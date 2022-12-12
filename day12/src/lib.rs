use std::collections::VecDeque;

pub fn solve_part_1(input: &str) -> i32 {
    let heightmap = Heightmap::new(input);

    find_path_to_target(&heightmap, heightmap.start)
        .expect("Couldn't find path from start to the target")
}

pub fn solve_part_2(input: &str) -> i32 {
    let heightmap = Heightmap::new(input);

    let possible_starts = heightmap
        .squares
        .iter()
        .enumerate()
        .filter(|(_, &s)| s == 0)
        .map(|(i, _)| i);

    let path_lengths = possible_starts.map(|start| find_path_to_target(&heightmap, start));

    path_lengths.into_iter().flatten().min().unwrap()
}

fn find_path_to_target(heightmap: &Heightmap, start: usize) -> Option<i32> {
    let mut steps_needed = vec![None; heightmap.squares.len()];
    steps_needed[start] = Some(0);

    let mut to_consider = VecDeque::from([start]);
    while let Some(square_idx) = to_consider.pop_front() {
        let square = heightmap.squares[square_idx];
        let neighbours = heightmap.get_neighbours(square_idx);

        for n_idx in neighbours {
            let neigh = heightmap.squares[n_idx];
            if neigh <= square + 1 {
                let steps = steps_needed[square_idx].unwrap() + 1;
                if steps_needed[n_idx].is_none() || steps_needed[n_idx].unwrap() > steps {
                    steps_needed[n_idx] = Some(steps);
                    to_consider.push_back(n_idx);
                }
            }
        }

        if square_idx == heightmap.target {
            break;
        }
    }

    steps_needed[heightmap.target]
}

fn read_height_char(ch: char) -> u8 {
    ch as u8 - b'a'
}

struct Heightmap {
    squares: Vec<u8>,
    width: usize,
    height: usize,
    start: usize,
    target: usize,
}

impl Heightmap {
    fn new(input: &str) -> Self {
        let mut start = 0;
        let mut target = 0;

        let squares = input
            .lines()
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(i, ch)| match ch {
                'S' => {
                    start = i;
                    read_height_char('a')
                }
                'E' => {
                    target = i;
                    read_height_char('z')
                }
                _ => read_height_char(ch),
            })
            .collect();

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        Self {
            squares,
            width,
            height,
            start,
            target,
        }
    }

    fn get_neighbours(&self, idx: usize) -> Vec<usize> {
        let x = idx % self.width;
        let y = idx / self.width;

        let mut neighbours = Vec::new();

        if y as isize > 0 {
            neighbours.push(x + (y - 1) * self.width);
        }
        if x + 1 < self.width {
            neighbours.push(x + 1 + y * self.width);
        }
        if x as isize > 0 {
            neighbours.push(x - 1 + y * self.width);
        }
        if y + 1 < self.height {
            neighbours.push(x + (y + 1) * self.width)
        }

        neighbours
    }
}
