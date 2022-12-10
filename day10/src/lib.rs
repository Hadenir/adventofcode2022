use std::str::FromStr;


pub fn solve_part_1(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();
    let mut cpu = Cpu::new(instructions);

    let interesting_cycles: Vec<usize> = (20..=220).step_by(40).collect();
    let mut result = Vec::with_capacity(interesting_cycles.len());

    let mut program_counter = 0usize;
    let mut cycle = 0usize;
    let mut cycles_left = 0usize;
    while program_counter < cpu.instructions.len() {
        cycle += 1;
        let curr_instr = &cpu.instructions[program_counter];

        if interesting_cycles.contains(&cycle) {
            result.push(cycle as i64 * cpu.reg_x);
        }

        match (&curr_instr, cycles_left) {
            (Instruction::Addx(_), 0) => { cycles_left = 2; }
            (Instruction::Addx(v), _) => { cpu.reg_x += v; }
            (Instruction::Noop, 0) => { cycles_left = 1; }
            _ => (),
        }

        cycles_left -= 1;
        if cycles_left == 0 {
            program_counter += 1;
        }
    }

    result.into_iter().sum()
}

pub fn solve_part_2(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();
    let mut cpu = Cpu::new(instructions);

    let mut screen = Screen::new(40, 6);

    let mut program_counter = 0usize;
    let mut cycle = 0usize;
    let mut cycles_left = 0usize;
    while program_counter < cpu.instructions.len() {
        let curr_instr = &cpu.instructions[program_counter];

        let horizontal_pos = (cycle % screen.width) as i64;
        match horizontal_pos - cpu.reg_x {
            -1 | 0 | 1 => screen.put_pixel(cycle),
            _ => ()
        }

        match (&curr_instr, cycles_left) {
            (Instruction::Addx(_), 0) => { cycles_left = 2; }
            (Instruction::Addx(v), _) => { cpu.reg_x += v; }
            (Instruction::Noop, 0) => { cycles_left = 1; }
            _ => (),
        }

        cycles_left -= 1;
        if cycles_left == 0 {
            program_counter += 1;
        }
        cycle += 1;
    }

    screen.display()
}

enum Instruction {
    Addx(i64),
    Noop,
}

struct Cpu {
    instructions: Vec<Instruction>,
    reg_x: i64,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        match split.next() {
            Some("addx") => {
                let val = split
                    .next()
                    .ok_or("No value provided for addx instruction")?
                    .parse()
                    .map_err(|_| "Invalid value provided for addx instruction")?;
                Ok(Instruction::Addx(val))
            }
            Some("noop") => Ok(Instruction::Noop),
            _ => Err("Invalid instruction encountered")
        }
    }
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            reg_x: 1,
        }
    }
}

#[allow(dead_code)]
struct Screen {
    pixels: Vec<char>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec!['.'; width * height],
            width,
            height,
        }
    }

    fn display(&self) -> String {
        self.pixels
            .chunks(self.width)
            .fold(String::new(), |out, line| format!("{out}{}\n", line.iter().collect::<String>()))
    }

    fn put_pixel(&mut self, pos: usize) {
        self.pixels[pos] = '#';
    }
}
