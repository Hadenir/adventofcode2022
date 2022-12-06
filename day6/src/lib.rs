
pub fn solve_part_1(input: &str) -> usize {
    let mut buff = Vec::with_capacity(4);

    for (i, char) in input.chars().enumerate() {
        if let Some(j) = buff.iter().position(|&x| x == char) {
            buff.drain(0..=j);
        }

        buff.push(char);
        if buff.len() >= 4 {
            return i + 1;
        }
    }

    unreachable!()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut buff = Vec::with_capacity(4);

    for (i, char) in input.chars().enumerate() {
        if let Some(j) = buff.iter().position(|&x| x == char) {
            buff.drain(0..=j);
        }

        buff.push(char);
        if buff.len() >= 14 {
            return i + 1;
        }
    }

    unreachable!()
}
