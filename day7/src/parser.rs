use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1, line_ending, not_line_ending},
    sequence::{preceded, separated_pair, delimited, terminated},
    IResult, multi::separated_list0, combinator::{opt, map}, branch::alt,
};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    ChangeDirectory(String),
    List,
}

pub fn parse_input(mut input: &str) -> FsEntry {
    let mut root  = FsEntry::new("", FsEntryType::Directory);
    let mut current_path: Vec<String> = vec![];

    while let Ok((rem, cmd)) = command(input) {
        input = rem;
        match cmd {
            Command::ChangeDirectory(dir_name) => {
                match dir_name.as_str() {
                    "/" => current_path = vec![],
                    ".." => { current_path.pop().expect("Cannot leave root directory"); },
                    _ => current_path.push(dir_name),
                }
            },
            Command::List => {
                let (rem, entries) = entries(input).expect("Failed to read ls output");
                input = rem;

                let mut current_dir = current_path
                    .iter()
                    .fold(&mut root, |dir, name| dir.get_dir(name).unwrap_or_else(|| panic!("Directory {name} does not exist")));
                current_dir.children = entries;
            },
        }
    }

    assert_eq!(input, "");

    root
}

fn file(input: &str) -> IResult<&str, (String, usize)> {
    let (input, (size_str, file_name)) = separated_pair(
        digit1,
        space1,
        not_line_ending
    )(input)?;

    let size: usize = size_str.parse().unwrap();

    Ok((input, (file_name.to_string(), size)))
}

fn directory(input: &str) -> IResult<&str, String> {
    let (input, dir_name) = preceded(
        tag("dir "),
        not_line_ending,
    )(input)?;

    Ok((input, dir_name.to_string()))
}

fn entry(input: &str) -> IResult<&str, FsEntry> {
    alt((
        map(
            file,
            |(name, size)| FsEntry::new(name, FsEntryType::File { size }),
        ),
        map(
            directory,
            |name| FsEntry::new(name, FsEntryType::Directory),
        )
    ))(input)
}

fn entries(input: &str) -> IResult<&str, Vec<FsEntry>> {
    terminated(
        separated_list0(
            line_ending,
            entry
        ),
        opt(line_ending)
    )(input)
}

fn cd_command(input: &str) -> IResult<&str, Command> {
    let (input, (_, dir_name)) = separated_pair(
        tag("cd"),
        space1,
        not_line_ending,
    )(input)?;

    Ok((input, Command::ChangeDirectory(dir_name.to_string())))
}

fn ls_command(input: &str) -> IResult<&str, Command> {
    map(
        tag("ls"),
        |_| Command::List,
    )(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    delimited(
        tag("$ "),
        alt((cd_command, ls_command)),
        opt(line_ending),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {
        let input = "12345 filename.txt";

        let (rem, file) = file(input).unwrap();

        assert_eq!(file, ("filename.txt".to_string(), 12345));
        assert_eq!(rem, "");
    }

    #[test]
    fn test_directory() {
        let input = "dir dirname";

        let (rem, dir) = directory(input).unwrap();

        assert_eq!(dir, ("dirname".to_string()));
        assert_eq!(rem, "");
    }

    #[test]
    fn test_entries() {
        let input = "dir a
dir b
12345 file1
54321 file2";

        let (rem, entries) = entries(input).unwrap();

        assert_eq!(entries, vec![
            FsEntry::new("a".to_string(), FsEntryType::Directory),
            FsEntry::new("b".to_string(), FsEntryType::Directory),
            FsEntry::new("file1".to_string(), FsEntryType::File { size: 12345 }),
            FsEntry::new("file2".to_string(), FsEntryType::File { size: 54321 }),
        ]);
        assert_eq!(rem, "");
    }

    #[test]
    fn test_cd_command() {
        let input = "cd home";

        let (rem, command) = cd_command(input).unwrap();

        assert_eq!(command, Command::ChangeDirectory("home".to_string()));
        assert_eq!(rem, "");
    }

    #[test]
    fn test_ls_command() {
        let input = "ls";

        let (rem, command) = ls_command(input).unwrap();

        assert_eq!(command, Command::List);
        assert_eq!(rem, "");
    }

    #[test]
    fn test_command() {
        let input = "$ cd home";

        let (rem, cmd) = command(input).unwrap();

        assert_eq!(cmd, Command::ChangeDirectory("home".to_string()));
        assert_eq!(rem, "");

        let input = "$ ls";

        let (rem, cmd) = command(input).unwrap();

        assert_eq!(cmd, Command::List);
        assert_eq!(rem, "");
    }
}
