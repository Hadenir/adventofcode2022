use std::fs;

use day20::*;

fn main() {
    let contents = fs::read_to_string("day20/input.txt").expect("Failed to read puzzle input");

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part_1() {
        let sol = solve_part_1(INPUT);

        assert_eq!(sol, 3);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let sol = solve_part_2(INPUT);

        assert_eq!(sol, 0);
    }
}
