use std::fs;

use day5::*;

fn main() {
    let contents = fs::read_to_string("day5/input.txt").unwrap();
    let (stacks, moves) = parser::parse_input(&contents);

    println!("Part 1: {}", solve_part_1(stacks.clone(), moves.clone()));

    println!("Part 2: {}", solve_part_2(stacks, moves));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    \n\
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        let (stacks, moves) = parser::parse_input(INPUT);
        let sol = solve_part_1(stacks, moves);

        assert_eq!(sol, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let (stacks, moves) = parser::parse_input(INPUT);
        let sol = solve_part_2(stacks, moves);

        assert_eq!(sol, "MCD")
    }
}
