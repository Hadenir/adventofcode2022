use std::fs;

use day6::*;

fn main() {
    let contents = fs::read_to_string("day6/input.txt").unwrap();

    println!("Part 1: {}", solve_part_1(&contents));

    println!("Part 2: {}", solve_part_2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: [&str; 5] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn test_part_1() {
        let answers = [7, 5, 6, 10, 11];

        for (input, answer) in INPUTS.iter().zip(answers) {
            let sol = solve_part_1(input);

            assert_eq!(sol, answer)
        }
    }

    #[test]
    fn test_part_2() {
        let answers = [19, 23, 23, 29, 26];

        for (input, answer) in INPUTS.iter().zip(answers) {
            let sol = solve_part_2(input);

            assert_eq!(sol, answer)
        }}
}
