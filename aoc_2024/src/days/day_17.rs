use rayon::iter::ParallelIterator;
use rayon::prelude::*;

use super::RE_DIGITS;

pub fn solve_day(input_file: &str) -> (String, u64) {
    let a = part_a(&input_file);
    // let b = part_b(&input_file);
    let b = 0;
    (a, b)
}

fn part_a(input_file: &str) -> String {
    let (mut computer, program) = parse(input_file);
    let out = computer.run(&program);
    out.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_b(input_file: &str) -> u64 {
    let (computer, program) = parse(input_file);
    let MIN_A = 8usize.pow((program.len() - 1) as u32);
    println!("{}", MIN_A);

    let first = (MIN_A..10_000_000_000_000_000)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|a: &usize| {
            let mut computer = computer;
            computer.reg.a = *a as u64;
            program == computer.run(&program)
        });

    if let Some(first) = first {
        println!("A: {first}");
        return first as u64;
    } else {
        panic!()
    }
}

fn parse(input_file: &str) -> (Computer, Vec<u64>) {
    let digits: Vec<u64> = RE_DIGITS
        .captures_iter(input_file)
        .map(|x| {
            x.get(0)
                .unwrap()
                .as_str()
                .parse()
                .expect("Catches only digits")
        })
        .collect();

    (
        (digits[0], digits[1], digits[2]).into(),
        digits.into_iter().skip(3).collect(),
    )
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    fn from_opcode(opcode: &u64) -> Instruction {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("Should not be reachable"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone, Copy)]
struct Computer {
    reg: Register,
    ptr: usize,
}

impl Computer {
    fn run(&mut self, program: &[u64]) -> Vec<u64> {
        let mut out = vec![];
        while let Some((ins, lit)) = self.get_ins_lit(&program) {
            let inst = Instruction::from_opcode(&ins);
            match inst {
                Instruction::Adv => self.reg.a = self.division(lit),
                Instruction::Bxl => self.reg.b ^= program.get(self.ptr + 1).expect("Invalid state"),
                Instruction::Bst => self.reg.b = self.get_combo(lit) % 8,
                Instruction::Jnz => {
                    if self.reg.a != 0 {
                        self.ptr = lit as usize;
                        continue;
                    }
                }
                Instruction::Bxc => self.reg.b ^= self.reg.c,
                Instruction::Out => {
                    out.push(self.get_combo(lit) % 8);
                }
                Instruction::Bdv => self.reg.b = self.division(lit),
                Instruction::Cdv => self.reg.c = self.division(lit),
            }
            self.ptr += 2;
        }
        out
    }

    fn division(&self, lit: u64) -> u64 {
        let t = 2_u64.pow(self.get_combo(lit) as u32);
        return self.reg.a / t;
    }

    fn get_ins_lit(&self, program: &[u64]) -> Option<(u64, u64)> {
        if let Some(ins) = program.get(self.ptr) {
            if let Some(lit) = program.get(self.ptr + 1) {
                return Some((*ins, *lit));
            }
            panic!()
        }
        None
    }

    fn get_combo(&self, lit: u64) -> u64 {
        match lit {
            (0..=3) => lit,
            4 => self.reg.a,
            5 => self.reg.b,
            6 => self.reg.c,
            7 => panic!("Invalid program"),
            _ => unreachable!(),
        }
    }
}

impl From<(u64, u64, u64)> for Computer {
    fn from(value: (u64, u64, u64)) -> Self {
        Computer {
            reg: value.into(),
            ptr: 0,
        }
    }
}

impl From<(u64, u64, u64)> for Register {
    fn from(value: (u64, u64, u64)) -> Self {
        Register {
            a: value.0,
            b: value.1,
            c: value.2,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {}

    #[rstest]
    #[case((0, 0, 9).into(), vec![2, 6], (0, 1, 9).into(), vec![])]
    #[case((10, 0, 0).into(), vec![5,0,5,1,5,4], (10, 0, 0).into(), vec![0, 1, 2])]
    #[case((2024, 0, 0).into(), vec![0,1,5,4,3,0], (0, 0, 0).into(), vec![4,2,5,6,7,7,7,7,3,1,0])]
    #[case((0, 29, 0).into(), vec![1,7], (0, 26, 0).into(), vec![])]
    #[case((0, 2024, 43690).into(), vec![4,0], (0, 44354, 43690).into(), vec![])]
    #[timeout(Duration::from_millis(10))]
    fn test_example_instructions(
        #[case] mut computer: Computer,
        #[case] program: Vec<u64>,
        #[case] end_register: Register,
        #[case] output: Vec<u64>,
    ) {
        let out = computer.run(&program);
        assert_eq!(out, output);
        assert_eq!(computer.reg, end_register);
    }

    #[rstest]
    #[case(
        "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0",
        "4,6,3,5,6,3,5,2,1,0"
    )]
    fn test_part_a(#[case] input: &str, #[case] output: &str) {
        assert_eq!(part_a(input), output)
    }

    #[rstest]
    #[case(
        "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0",
        117440
    )]
    fn test_part_b(#[case] input: &str, #[case] output: u64) {
        assert_eq!(part_b(input), output)
    }
}
