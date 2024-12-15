use std::str::FromStr;

use super::RE_DIGITS;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let machines = parse_input(input_file);
    let a = part_a(&machines);
    let b = part_b(&machines);
    (a, b)
}

fn parse_input(input_file: &str) -> Vec<Machine> {
    input_file
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Machine>().expect("Invalid input"))
        .collect()
}

fn part_a(machines: &[Machine]) -> u64 {
    machines.iter().filter_map(|m| m.brute_force()).sum()
}

fn part_b(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .filter_map(|m| m.solve_mathematically(10000000000000))
        .sum()
}
#[derive(Debug, PartialEq, Eq)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Coord,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

#[derive(Debug, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Machine {
    fn brute_force(&self) -> Option<u64> {
        let mut best_cost: Option<u64> = None;

        for n_a in 0..=100 {
            for n_b in 0..=100 {
                if n_a * self.button_a.x + n_b * self.button_b.x == self.prize.x
                    && n_a * self.button_a.y + n_b * self.button_b.y == self.prize.y
                {
                    let cost = (n_a * 3 + n_b) as u64;
                    if best_cost.is_none_or(|b| cost < b) {
                        best_cost = Some(cost)
                    }
                }
            }
        }

        best_cost
    }

    fn solve_mathematically(&self, offset: i64) -> Option<u64> {
        let p_x = self.prize.x + offset;
        let p_y = self.prize.y + offset;
        let nom_b = p_y * self.button_a.x - p_x * self.button_a.y;
        let den_b = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;
        if nom_b % den_b != 0 {
            return None;
        }
        let b = nom_b / den_b;
        let nom_a = p_x - b * self.button_b.x;
        let den_a = self.button_a.x;
        if nom_a % den_a != 0 {
            return None;
        }
        let a = nom_a / den_a;
        let cost = (a * 3 + b) as u64;
        Some(cost)
    }
}

impl FromStr for Machine {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<i64> = RE_DIGITS
            .captures_iter(s)
            .map(|x| {
                x.get(0)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Catches only digits")
            })
            .collect();
        assert_eq!(digits.len(), 6, "Machine should contain 6 digits");

        let button_a: Button = Button {
            x: digits[0],
            y: digits[1],
        };
        let button_b: Button = Button {
            x: digits[2],
            y: digits[3],
        };
        let prize: Coord = Coord {
            x: digits[4],
            y: digits[5],
        };
        Ok(Machine {
            button_a,
            button_b,
            prize,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400",
        Machine {
            button_a: Button { x: 94, y: 34 }, button_b: Button { x: 22, y: 67 },
            prize: Coord { x: 8400, y: 5400 },
        },
    )]
    #[case(
        "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176",
        Machine {
            button_a: Button { x: 26, y: 66 }, button_b: Button { x: 67, y: 21 },
            prize: Coord { x:12748, y:12176 },
        },
    )]
    fn test_parse(#[case] s: &str, #[case] machine: Machine) {
        assert_eq!(s.parse(), Ok(machine))
    }

    #[rstest]
    #[case(
        "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400",
        Some(280)
    )]
    #[case(
        "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176",
        None
    )]
    fn test_brute_force(#[case] machine: Machine, #[case] cost: Option<u64>) {
        assert_eq!(machine.brute_force(), cost)
    }

    #[rstest]
    #[case(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
",
        480
    )]
    fn test_part_a(#[case] input: &str, #[case] expected: u64) {
        let machines = parse_input(input);
        assert_eq!(part_a(&machines), expected)
    }

    #[rstest]
    #[case(
        "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400",
        Some(280)
    )]
    #[case(
        "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176",
        None
    )]
    fn test_solver(#[case] machine: Machine, #[case] cost: Option<u64>) {
        assert_eq!(machine.solve_mathematically(0), cost)
    }
}
