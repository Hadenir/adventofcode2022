use std::fs;

use day9::*;

fn main() {
    let contents = fs::read_to_string("day9/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 13);
    }

    #[test]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 1);
    }

    #[test]
    fn test_part_2_larger() {
        let sol = solve_part_2("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");

        assert_eq!(sol, 36);
    }
}
