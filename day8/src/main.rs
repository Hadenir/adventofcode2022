use std::fs;

use day8::*;

fn main() {
    let contents = fs::read_to_string("day8/input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 21);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 8);
    }
}
