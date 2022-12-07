use std::fs;

use day7::*;

fn main() {
    let contents = fs::read_to_string("day7/input.txt").unwrap();

    let input = parser::parse_input(&contents);

    println!("Part 1: {}", solve_part_1(input.clone()));

    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_1() {
        let input = parser::parse_input(INPUT);

        let sol = solve_part_1(input);

        assert_eq!(sol, 95437);
    }

    #[test]
    fn test_part_2() {
        let input = parser::parse_input(INPUT);

        let sol = solve_part_2(input);

        assert_eq!(sol, 24933642);
    }
}
