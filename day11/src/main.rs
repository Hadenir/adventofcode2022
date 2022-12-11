use std::fs;

use day11::{*, parser::parse_input};

fn main() {
    let contents = fs::read_to_string("day11/input.txt").expect("Failed to read puzzle input");

    let (_, input) = parse_input(&contents);
    println!("Part 1: {}", solve_part_1(input));

    let input = parse_input(&contents);
    println!("Part 2: {}", solve_part_2(input));
}


#[cfg(test)]
mod tests {
    use crate::parser::parse_input;

    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part_1() {
        let (_, input) = parse_input(INPUT);

        let sol = solve_part_1(input);

        assert_eq!(sol, 10605);
    }


    #[test]
    fn test_part_2() {
        let input = parse_input(INPUT);

        let sol = solve_part_2(input);

        assert_eq!(sol, 2713310158u64);
    }
}
