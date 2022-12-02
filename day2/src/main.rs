use std::{error, fs};

use day2::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string("day2/input.txt")?;

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"A Y
B X
C Z";

        let sol = solve_part_1(input);
        assert_eq!(sol, 15)
    }

    #[test]
    fn test_part_2() {
        let input = r"A Y
B X
C Z";

        let sol = solve_part_2(input);
        assert_eq!(sol, 12)
    }
}
