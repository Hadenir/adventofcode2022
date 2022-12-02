use std::{fs, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string("day1/input.txt")?;

    let mut elf_cals = Vec::new();
    for (i, elf_cals_str) in contents.split("\r\n\r\n").enumerate() {
        elf_cals.push(0);
        for cals_str in elf_cals_str.lines() {
            let cals: i32 = cals_str.parse()?;
            elf_cals[i] += cals;
        }
    }

    elf_cals.sort();
    let most_cals = elf_cals.last();
    println!("Most calories: {}", most_cals.unwrap());

    let n = elf_cals.len();
    let most_3_cals: i32 = elf_cals[(n-3)..].iter().sum();
    println!("Most top 3 calories: {}", most_3_cals);

    Ok(())
}

#[cfg(test)]
mod tests {

}
