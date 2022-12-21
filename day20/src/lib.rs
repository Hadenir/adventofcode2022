
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse coordinate"))
        .collect()
}

pub fn solve_part_1(input: &str) -> i32 {
    let encrypted = parse_input(input);
    let decrypted = mix(encrypted);

    let len = decrypted.len();
    let idx = decrypted
        .iter()
        .enumerate()
        .find(|&(_, &n)| n == 0)
        .map(|(i, _)| i)
        .unwrap();

    [idx + 1000, idx + 2000, idx + 3000]
        .into_iter()
        .map(|i| decrypted[i.rem_euclid(len)])
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    todo!()
}

fn mix(encrypted: Vec<i32>) -> Vec<i32> {
    let mut sequence: Vec<_> = encrypted.into_iter().enumerate().collect();

    let len = sequence.len();
    for i in 0..len {
        let (idx, _) = sequence
            .iter()
            .enumerate()
            .find(|&(_, &(j, _))| i == j)
            .unwrap();
        let el = sequence[idx];
        let num = el.1;

        let mut j = idx;
        let dj = num.signum();
        for _ in 0..(num % len as i32).abs() {
            let k = (j as i32 + dj).rem_euclid(len as i32) as usize;
            sequence.swap(j, k);
            j = k;
        }

        dbg!(sequence.iter().map(|(_,n)|n).collect::<Vec<_>>());
    }

    sequence.into_iter().map(|(_, n)| n).collect()
}


#[test]
fn test_test() {
    let enc = vec![-6, 0, -4, -10, 3, 1];

    assert_eq!(enc, vec![0, -4, -10, 3, -6, 1]);
}
