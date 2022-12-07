pub mod parser;

#[derive(Debug, PartialEq, Eq, Clone)]
enum FsEntryType {
    File { size: usize },
    Directory,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FsEntry {
    name: String,
    entry_type: FsEntryType,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn new(name: impl ToString, entry_type: FsEntryType) -> Self {
        Self {
            name: name.to_string(),
            entry_type,
            children: vec![],
        }
    }

    fn get_dir(&mut self, name: &str) -> Option<&mut Self> {
        self.children
            .iter_mut()
            .filter(|ch| ch.entry_type == FsEntryType::Directory)
            .find(|ch| ch.name == name)
    }

    fn dirs(&self) -> impl Iterator<Item = &Self> {
        self.children
            .iter()
            .filter(|ch| ch.entry_type == FsEntryType::Directory)
    }
}

pub fn solve_part_1(input: FsEntry) -> usize {
    let root = input;

    let mut dirs: Vec<&FsEntry> = root.dirs().collect();
    let mut total: usize = 0;
    while let Some(dir) = dirs.pop() {
        dirs.extend(dir.dirs());

        let size = calc_size(dir);
        if size <= 100000 {
            total += size;
        }
    }

    total
}

fn calc_size(entry: &FsEntry) -> usize {
    match entry.entry_type {
        FsEntryType::File { size } => size,
        FsEntryType::Directory => entry.children.iter().fold(0usize, |acc, ch| acc + calc_size(ch))
    }
}

pub fn solve_part_2(input: FsEntry) -> usize {
    let root = input;

    let max_size = 70000000usize;
    let needed_size = 30000000usize;
    let taken_size = calc_size(&root);
    let free_size = max_size - taken_size;
    let missing_size = needed_size - free_size;

    let mut to_delete = max_size;

    let mut dirs: Vec<&FsEntry> = root.dirs().collect();
    while let Some(dir) = dirs.pop() {
        dirs.extend(dir.dirs());

        let size = calc_size(dir);
        if size >= missing_size && size < to_delete {
            to_delete = size;
        }
    }

    to_delete
}
