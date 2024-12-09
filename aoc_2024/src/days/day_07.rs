use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let equations = &parse_input(input_file);
    let a = part_a(equations);
    let b = part_b(equations);
    (a, b)
}

fn parse_input(input_file: &str) -> Vec<Equation> {
    input_file
        .trim()
        .split("\n")
        .map(|s| Equation::from_str(s).unwrap())
        .collect()
}

fn part_a(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|eq| eq.is_solvable_backwards(&[Operator::Sum, Operator::Mul]))
        .map(|eq| eq.lhs)
        .sum()
}

fn part_b(equations: &Vec<Equation>) -> u64 {
    equations
        .iter()
        .filter(|eq| eq.is_solvable_backwards(&[Operator::Sum, Operator::Mul, Operator::Concat]))
        .map(|eq| eq.lhs)
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Equation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once(": ") {
            let lhs: u64 = lhs.parse().unwrap();
            let rhs: Vec<u64> = rhs.split(" ").map(|d| d.parse().unwrap()).collect();
            Ok(Equation { lhs, rhs })
        } else {
            println!("Could not parse '{}'", s);
            Err(ParseError)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operator {
    Sum,
    Mul,
    Concat,
}

impl Operator {
    fn perform(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Sum => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Concat => lhs * 10u64.pow(rhs.ilog10() + 1) + rhs,
        }
    }

    fn perform_rev(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Sum => lhs - rhs,
            Operator::Mul => lhs / rhs,
            Operator::Concat => (lhs - rhs) / 10u64.pow(rhs.ilog10() + 1),
        }
    }

    fn possible_rev(&self, lhs: u64, rhs: u64) -> bool {
        match self {
            Operator::Sum => lhs > rhs,
            Operator::Mul => lhs % rhs == 0,
            Operator::Concat => lhs > rhs && (lhs - rhs) % 10u64.pow(rhs.ilog10() + 1) == 0,
        }
    }
}

impl Equation {
    #[allow(dead_code)]
    fn is_solvable(&self, operators: &[Operator]) -> bool {
        recursive_solve(self.lhs, self.rhs[0], &self.rhs[1..], operators)
    }

    fn is_solvable_backwards(&self, operators: &[Operator]) -> bool {
        let len = self.rhs.len();
        recursive_solve_back(
            self.lhs,
            &self.rhs[..(len - 1)],
            self.rhs[len - 1],
            operators,
        )
    }
}

fn recursive_solve(lhs: u64, rhs_init: u64, rhs_rem: &[u64], operators: &[Operator]) -> bool {
    if lhs < rhs_init {
        return false;
    }
    match rhs_rem.first() {
        Some(rhs) => operators
            .iter()
            .any(|op| recursive_solve(lhs, op.perform(rhs_init, *rhs), &rhs_rem[1..], operators)),
        None => lhs == rhs_init,
    }
}

fn recursive_solve_back(lhs: u64, rhs_rem: &[u64], rhs_init: u64, operators: &[Operator]) -> bool {
    let len = rhs_rem.len();
    match rhs_rem.last() {
        Some(rhs) => operators.iter().any(|op| {
            op.possible_rev(lhs, rhs_init)
                && recursive_solve_back(
                    op.perform_rev(lhs, rhs_init),
                    &rhs_rem[..len - 1],
                    *rhs,
                    operators,
                )
        }),
        None => lhs == rhs_init,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("190: 10 19", Equation {lhs: 190, rhs: vec![10, 19,]})]
    #[case("3267: 81 40 27", Equation {lhs: 3267, rhs: vec![81, 40, 27]})]
    #[case("83: 17 5", Equation {lhs: 83, rhs: vec![17, 5,]})]
    fn test_parse(#[case] line: &str, #[case] eq: Equation) {
        assert_eq!(Equation::from_str(line), Ok(eq))
    }

    #[rstest]
    #[case("190: 10 19", true)]
    #[case("3267: 81 40 27", true)]
    #[case("83: 17 5", false)]
    #[case("156: 15 6", false)]
    #[case("7290: 6 8 6 15", false)]
    #[case("161011: 16 10 13", false)]
    #[case("192: 17 8 14", false)]
    #[case("21037: 9 7 18 13", false)]
    #[case("292: 11 6 16 20", true)]
    fn test_eq_is_solvable(#[case] eq: Equation, #[case] is_solvable: bool) {
        assert_eq!(eq.is_solvable(&[Operator::Sum, Operator::Mul]), is_solvable)
    }

    #[rstest]
    fn test_part_a() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let input = &parse_input(&input);
        assert_eq!(part_a(input), 3749)
    }

    #[rstest]
    #[case("190: 10 19", true)]
    #[case("3267: 81 40 27", true)]
    #[case("83: 17 5", false)]
    #[case("156: 15 6", true)]
    #[case("7290: 6 8 6 15", true)]
    #[case("161011: 16 10 13", false)]
    #[case("192: 17 8 14", true)]
    #[case("21037: 9 7 18 13", false)]
    #[case("292: 11 6 16 20", true)]
    fn test_eq_is_solvable_b(#[case] eq: Equation, #[case] is_solvable: bool) {
        assert_eq!(
            eq.is_solvable(&[Operator::Sum, Operator::Mul, Operator::Concat]),
            is_solvable
        )
    }

    #[rstest]
    #[case(900, 456)]
    #[case(4029, 49027)]
    fn test_rev(#[case] lhs: u64, #[case] rhs: u64) {
        for op in [Operator::Sum, Operator::Concat, Operator::Mul] {
            // To prevent underflow in case of sum, we add rhs to lhs
            let lhs = lhs + rhs;
            let forward = op.perform(lhs, rhs);
            let backward = op.perform_rev(forward, rhs);
            assert_eq!(lhs, backward);
        }
    }

    #[rstest]
    fn test_part_b() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let input = &parse_input(&input);
        assert_eq!(part_b(input), 11387)
    }
}
