use std::{num::ParseIntError, str::FromStr};

pub fn solve(input: &str) -> (i32, String) {
    let instructions = parse_input(input);
    let cpu = Cpu::default().apply_many(instructions);
    let part_1 = solve_part_1(&cpu);
    let part_2 = solve_part_2(&cpu);
    (part_1, part_2)
}

fn parse_input(input_str: &str) -> Vec<Op> {
    input_str
        .split('\n')
        .map(Op::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn solve_part_1(cpu: &Cpu) -> i32 {
    cpu.get_signal_stregth_at(20)
        + cpu.get_signal_stregth_at(60)
        + cpu.get_signal_stregth_at(100)
        + cpu.get_signal_stregth_at(140)
        + cpu.get_signal_stregth_at(180)
        + cpu.get_signal_stregth_at(220)
}

fn solve_part_2(cpu: &Cpu) -> String {
    (0..6)
        .map(|x| get_line(cpu, x))
        .collect::<Vec<String>>()
        .join("\n")
}

fn get_line(cpu: &Cpu, line: u32) -> String {
    let start = (line * 40) + 1;
    let end = start + 40;
    (start..end)
        .map(|x| cpu.get_pixel(x))
        .map(|lit| if lit { '#' } else { '.' })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Cpu {
    register: i32,
    cycle: u32,
    states: Vec<(u32, i32)>,
}
impl Cpu {
    pub(crate) fn default() -> Cpu {
        Cpu {
            register: 1,
            cycle: 0,
            states: vec![(0, 1)],
        }
    }
}

impl Cpu {
    fn apply_many(self, ops: Vec<Op>) -> Self {
        ops.iter().fold(self, |cpu, op| cpu.apply_return(op))
    }

    fn apply(&mut self, op: &Op) {
        self.cycle += op.cost();
        match op {
            Op::NoOp() => (),
            Op::AddX(x) => {
                self.register += x;
                self.states.push((self.cycle, self.register))
            }
        }
    }

    fn apply_return(mut self, op: &Op) -> Self {
        self.apply(op);
        self
    }

    fn get_pixel(&self, x: u32) -> bool {
        let register = self.get_register_at(x);
        let pixel_loc = (((x - 1) % 40) + 1) as i32;
        register <= pixel_loc && pixel_loc < register + 3
    }

    fn get_register_at(&self, state: u32) -> i32 {
        let (_, register) = self
            .states
            .iter()
            .filter(|(state_, _)| state_ < &state)
            .last()
            .unwrap();
        *register
    }

    fn get_signal_stregth_at(&self, state: u32) -> i32 {
        self.get_register_at(state) * state as i32
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    NoOp(),
    AddX(i32),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (operation, value) = s.split_once(' ').unwrap_or((s, " "));
        let op = match operation {
            "noop" => Op::NoOp(),
            "addx" => Op::AddX(value.parse::<i32>()?),
            _ => unreachable!(),
        };
        Ok(op)
    }
}

impl Op {
    fn cost(&self) -> u32 {
        match self {
            Op::NoOp() => 1,
            Op::AddX(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input_str() -> String {
        read_day_input("test_day_10")
    }

    #[fixture]
    fn test_operations(test_input_str: String) -> Vec<Op> {
        parse_input(&test_input_str)
    }

    #[fixture]
    fn test_cpu(test_operations: Vec<Op>) -> Cpu {
        Cpu::default().apply_many(test_operations)
    }

    #[rstest]
    #[case("noop", Op::NoOp())]
    #[case("addx 15", Op::AddX(15))]
    #[case("addx -15", Op::AddX(-15))]
    fn parse_op(#[case] input: &str, #[case] expected: Op) {
        assert_eq!(Op::from_str(input), Ok(expected))
    }

    #[rstest]
    fn test_default_cpu() {
        let cpu = Cpu::default();
        assert_eq!(
            cpu,
            Cpu {
                register: 1,
                cycle: 0,
                states: vec![(0, 1)],
            }
        )
    }

    #[rstest]
    #[case(20, 420)]
    #[case(60, 1140)]
    #[case(100, 1800)]
    #[case(140, 2940)]
    #[case(180, 2880)]
    #[case(220, 3960)]
    fn test_state_of_cpu_is_correct(
        test_operations: Vec<Op>,
        #[case] state: u32,
        #[case] strength: i32,
    ) {
        let cpu = Cpu::default().apply_many(test_operations);
        assert_eq!(cpu.get_signal_stregth_at(state), strength)
    }

    #[rstest]
    fn test_solve_part_1(test_cpu: Cpu) {
        assert_eq!(solve_part_1(&test_cpu), 13140);
    }

    #[rstest]
    #[case(1, true)]
    #[case(2, true)]
    #[case(3, false)]
    #[case(4, false)]
    #[case(5, true)]
    #[case(6, true)]
    fn test_get_pixel(test_cpu: Cpu, #[case] x: u32, #[case] lit: bool) {
        assert_eq!(test_cpu.get_pixel(x), lit)
    }

    #[rstest]
    #[case(0, "##..##..##..##..##..##..##..##..##..##..")]
    #[case(1, "###...###...###...###...###...###...###.")]
    #[case(2, "####....####....####....####....####....")]
    #[case(3, "#####.....#####.....#####.....#####.....")]
    #[case(4, "######......######......######......####")]
    #[case(5, "#######.......#######.......#######.....")]
    fn test_get_line(test_cpu: Cpu, #[case] x: u32, #[case] line: String) {
        assert_eq!(get_line(&test_cpu, x), line)
    }

    #[rstest]
    #[case("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....")]
    fn test_solve_part_2(test_cpu: Cpu, #[case] expected: String) {
        assert_eq!(solve_part_2(&test_cpu), expected)
    }
}
