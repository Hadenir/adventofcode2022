use std::{error, fs};

use day4::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string("day4/input.txt")?;

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let sol = solve_part_1(input);
        assert_eq!(sol, 2);
    }

    #[test]
    fn test_part_2() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let sol = solve_part_2(input);
        assert_eq!(sol, 4);
    }
}
