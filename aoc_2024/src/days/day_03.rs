use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let instructions = parse_instructions(&input_file);
    let a = part_a(&instructions);
    let b = part_b(&instructions);
    (a, b)
}

lazy_static! {
    static ref RE_INST: Regex =
        Regex::new(r"((mul)\((\d{1,3}),(\d{1,3})\)|don't|do)").expect("regex should be valid");
}

fn part_a(instructions: &[Instruction]) -> u32 {
    instructions.iter().map(|ins| ins.solve()).sum()
}

fn part_b(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold((true, 0), |(enabled, sum), inst| match inst {
            Instruction::Mul(x, y) => match enabled {
                true => (enabled, sum + x * y),
                false => (enabled, sum),
            },
            Instruction::Do() => (true, sum),
            Instruction::Dont() => (false, sum),
        })
        .1
}

fn parse_instructions(input_file: &str) -> Vec<Instruction> {
    RE_INST
        .captures_iter(input_file)
        .map(|x| Instruction::new(x))
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do(),
    Dont(),
}

impl Instruction {
    fn new(x: Captures) -> Self {
        let inst = &x.get(2).unwrap_or(x.get(1).unwrap()).as_str();
        match *inst {
            "mul" => Instruction::Mul(x[3].parse().unwrap(), x[4].parse().unwrap()),
            "don't" => Instruction::Dont(),
            "do" => Instruction::Do(),
            a => todo!("{:?}", a),
        }
    }

    fn solve(&self) -> u32 {
        match self {
            Instruction::Mul(x, y) => x * y,
            Instruction::Do() => 0,
            Instruction::Dont() => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mul(1,1)", vec![Instruction::Mul(1, 1)])]
    #[case("mul(1,2)mul(1111)", vec![Instruction::Mul(1, 2)])]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        vec![Instruction::Mul(2, 4), Instruction::Do(), Instruction::Mul(5, 5), Instruction::Mul(11, 8), Instruction::Mul(8, 5)]
    )]
    #[case("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", vec![
        Instruction::Mul(2, 4), Instruction::Dont(), Instruction::Mul(5, 5), Instruction::Mul(11, 8), Instruction::Do(), Instruction::Mul(8, 5)
        ])]
    fn test_parse(#[case] text: &str, #[case] expected: Vec<Instruction>) {
        assert_eq!(parse_instructions(text), expected);
    }

    #[rstest]
    fn test_part_a() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let input = parse_instructions(input);
        let result = part_a(&input);
        assert_eq!(result, 161)
    }

    #[rstest]
    fn test_part_b() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = parse_instructions(input);
        let result = part_b(&input);
        assert_eq!(result, 48)
    }
}
