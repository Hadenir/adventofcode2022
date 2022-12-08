#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tree(u8);

#[derive(Debug)]
#[allow(dead_code)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<Tree>,
}

impl Forest {
    fn position(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn look_up(&self, col: usize, row: usize) -> impl Iterator<Item = &Tree> {
        self.trees
            .iter()
            .skip(col)
            .step_by(self.width)
            .take(row)
            .rev()
    }

    fn look_down(&self, col: usize, row: usize) -> impl Iterator<Item = &Tree> {
        self.trees
            .iter()
            .skip(col)
            .step_by(self.width)
            .skip(row + 1)
    }

    fn look_left(&self, col: usize, row: usize) -> impl Iterator<Item = &Tree> {
        self.trees
            .iter()
            .skip(row * self.width)
            .take(col)
            .rev()
    }

    fn look_right(&self, col: usize, row: usize) -> impl Iterator<Item = &Tree> {
        self.trees
            .iter()
            .skip(col + row * self.width + 1)
            .take(self.width - col - 1)
    }
}

fn check_visible(forest: &Forest, i: usize) -> bool {
    let (col, row) = forest.position(i);
    let tree = &forest.trees[i];

    let ranges: [Box<dyn Iterator<Item=&Tree>>; 4] = [
        Box::new(forest.look_up(col, row)),
        Box::new(forest.look_down(col, row)),
        Box::new(forest.look_left(col, row)),
        Box::new(forest.look_right(col, row)),
    ];

    ranges
        .into_iter()
        .any(|mut r| r.all(|t| t < tree))
}

pub fn solve_part_1(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let trees: Vec<Tree> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("Invalid digit encountered"))
        })
        .map(|d| Tree(d as u8))
        .collect();

    assert_eq!(width * height, trees.len());
    let forest = Forest {
        width,
        height,
        trees,
    };

    let count_visible = forest
        .trees
        .iter()
        .enumerate()
        .filter(|(i, _)| check_visible(&forest, *i))
        .count();

    count_visible
}

fn scenic_score(forest: &Forest, i: usize) -> usize {
    let (col, row) = forest.position(i);
    let tree = &forest.trees[i];

    let ranges: [Box<dyn Iterator<Item=&Tree>>; 4] = [
        Box::new(forest.look_up(col, row)),
        Box::new(forest.look_down(col, row)),
        Box::new(forest.look_left(col, row)),
        Box::new(forest.look_right(col, row)),
    ];

    let view_distances = ranges
        .into_iter()
        .map(|r| {
            let mut eq = true;
            r.take_while(|&t| {
                if eq {
                    if t >= tree {
                        eq = false;
                        true
                    } else {
                        t < tree
                    }
                } else {
                    false
                }
            }).count()
        });

    view_distances.product()
}

pub fn solve_part_2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let trees: Vec<Tree> = input
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).expect("Invalid digit encountered"))
        })
        .map(|d| Tree(d as u8))
        .collect();

    assert_eq!(width * height, trees.len());
    let forest = Forest {
        width,
        height,
        trees,
    };

    let scenic_scores = forest.trees
        .iter()
        .enumerate()
        .map(|(i, _)| scenic_score(&forest, i));

    scenic_scores.max().expect("No scenic score")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_up() {
        let forest = Forest {
            width: 3,
            height: 3,
            trees: vec![
                Tree(1),
                Tree(2),
                Tree(3),
                Tree(1),
                Tree(8),
                Tree(8),
                Tree(1),
                Tree(9),
                Tree(1),
            ],
        };

        assert_eq!(
            forest.look_up(1, 1).collect::<Vec<_>>(),
            vec![&forest.trees[1]],
        )
    }

    #[test]
    fn test_look_down() {
        let forest = Forest {
            width: 3,
            height: 3,
            trees: vec![
                Tree(1),
                Tree(2),
                Tree(3),
                Tree(1),
                Tree(8),
                Tree(8),
                Tree(1),
                Tree(9),
                Tree(1),
            ],
        };

        assert_eq!(
            forest.look_down(1, 1).collect::<Vec<_>>(),
            vec![&forest.trees[7]],
        )
    }

    #[test]
    fn test_look_left() {
        let forest = Forest {
            width: 3,
            height: 3,
            trees: vec![
                Tree(1),
                Tree(2),
                Tree(3),
                Tree(1),
                Tree(8),
                Tree(8),
                Tree(1),
                Tree(9),
                Tree(1),
            ],
        };

        assert_eq!(
            forest.look_left(1, 1).collect::<Vec<_>>(),
            vec![&forest.trees[3]],
        )
    }

    #[test]
    fn test_look_right() {
        let forest = Forest {
            width: 3,
            height: 3,
            trees: vec![
                Tree(1),
                Tree(2),
                Tree(3),
                Tree(4),
                Tree(8),
                Tree(8),
                Tree(5),
                Tree(9),
                Tree(6),
            ],
        };

        assert_eq!(
            forest.look_right(1, 1).collect::<Vec<_>>(),
            vec![&forest.trees[5]],
        )
    }
}
