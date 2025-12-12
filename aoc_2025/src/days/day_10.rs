use good_lp::{Expression, SolverModel, microlp, variable};
use std::{str::FromStr, vec};

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let machines = parse(input_file);
    let a = part_a(&machines);
    let b = part_b(&machines);
    (a, b)
}

fn part_a(machines: &[Machine]) -> u64 {
    machines.iter().map(|m| m.startup()).sum()
}

fn part_b(machines: &[Machine]) -> u64 {
    machines.iter().map(|m| m.joltage_rating()).sum()
}

fn parse(input_file: &str) -> Vec<Machine> {
    input_file
        .trim()
        .lines()
        .filter_map(|line| Machine::from_str(line).ok())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s.split_whitespace().peekable();
        let lights = parts
            .next()
            .expect("Could not parse lights")
            .trim_matches(&['[', ']'] as &[_])
            .chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>();

        let mut buttons = Vec::new();
        while let Some(&next) = parts.peek() {
            if next.starts_with('(') && next.ends_with(')') {
                let part = parts.next().unwrap();
                let button = part
                    .trim_matches(&['(', ')'] as &[_])
                    .split(',')
                    .map(|s| s.parse::<usize>().expect("Could not parse button index"))
                    .collect::<Vec<usize>>();
                buttons.push(button);
            } else {
                break;
            }
        }

        let joltage = parts
            .next()
            .expect("Could not parse joltage")
            .trim_matches(&['{', '}'] as &[_])
            .split(',')
            .filter_map(|s| s.parse::<u16>().ok())
            .collect::<Vec<u16>>();
        Ok(Machine {
            lights,
            buttons,
            joltage,
        })
    }
}

impl Machine {
    #[cfg(test)]
    fn new(lights: Vec<bool>, buttons: Vec<Vec<usize>>, joltage: Vec<u16>) -> Self {
        Machine {
            lights,
            buttons,
            joltage,
        }
    }

    fn startup(&self) -> u64 {
        Self::_startup_bfs(&self.lights, &self.buttons)
    }

    fn joltage_rating(&self) -> u64 {
        let mut variables = good_lp::variables!();
        let mut objective = Expression::from_other_affine(0);

        let mut constraints = vec![];
        let mut presses = vec![];
        for i in 0..self.buttons.len() {
            let p = variable()
                .integer()
                .min(0)
                .name(format!("btn_{i}_{:?}", self.buttons[i]));
            let n_presses = variables.add(p);
            objective += n_presses;
            presses.push(n_presses);
        }

        for (i, target_joltage) in self.joltage.iter().enumerate() {
            let mut expr = Expression::from_other_affine(0);
            for (j, btn) in self.buttons.iter().enumerate() {
                if btn.contains(&i) {
                    expr += presses[j] * 1u32;
                }
            }
            constraints.push(expr.eq(*target_joltage as u32).set_name(format!("{i}")));
        }

        if let Ok(solution) = variables
            .minimise(objective)
            .using(microlp)
            .with_all(constraints)
            .solve()
        {
            return solution.into_inner().objective().round() as u64;
        }
        panic!("Could not solve MILP for joltage rating");
    }

    fn _startup_bfs(target_state: &[bool], buttons: &[Vec<usize>]) -> u64 {
        let state = vec![false; target_state.len()];
        let mut queue: std::collections::VecDeque<(u64, Vec<bool>)> =
            std::collections::VecDeque::from([(0, state.to_vec())]);
        let mut visited = std::collections::HashSet::new();
        visited.insert(state.to_vec());

        while let Some((steps, current_state)) = queue.pop_front() {
            if current_state == target_state {
                return steps;
            }

            for button in buttons {
                let new_state = Self::_apply_button(&current_state, button);
                if !visited.contains(&new_state) {
                    visited.insert(new_state.to_vec());
                    queue.push_back((steps + 1, new_state));
                }
            }
        }
        u64::MAX
    }

    fn _apply_button(state: &[bool], button: &[usize]) -> Vec<bool> {
        let mut new_state = state.to_vec();
        for &idx in button {
            if idx < new_state.len() {
                new_state[idx] = !new_state[idx];
            }
        }
        new_state
    }
}
#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        Machine::new(vec![false, true,true,false], vec![vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]], vec![3,5,4,7])
    )]
    #[case(
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        Machine::new(vec![false,false,false,true,false], vec![vec![0,2,3,4], vec![2,3], vec![0,4], vec![0,1,2], vec![1,2,3,4]], vec![7,5,12,7,2])
    )]
    #[case(
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        Machine::new(vec![false,true,true,true,false,true], vec![vec![0,1,2,3,4], vec![0,3,4], vec![0,1,2,4,5], vec![1,2]], vec![10,11,11,5,10,5])
    )]
    #[case("[.#..#.###] (0,3,5,6,7) (2,3,4,5,6,8) (0,1,2,3,4,5,6,8) (0,1,2,3) (0,1,2,6) (0,1,3,7,8) (0,1,5,6,8) (4,8) (0,2,3) (5,6) (0,3,4,6) {279,232,41,255,52,73,95,202,239}",
     Machine::new(
        vec![false, true, false, false, true, false, true, true, true],
        vec![
            vec![0,3,5,6,7],
            vec![2,3,4,5,6,8],
            vec![0,1,2,3,4,5,6,8],
            vec![0,1,2,3],
            vec![0,1,2,6],
            vec![0,1,3,7,8],
            vec![0,1,5,6,8],
            vec![4,8],
            vec![0,2,3],
            vec![5,6],
            vec![0,3,4,6],
        ],
        vec![279,232,41,255,52,73,95,202,239]
     )
    )]
    fn test_parse(#[case] input: &str, #[case] expected: Machine) {
        let machine = Machine::from_str(input).unwrap();
        println!("Parsed machine: {:?}", machine);
        assert_eq!(machine, expected);
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 3)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 2)]
    fn test_startup(#[case] input: &str, #[case] expected_startup: u64) {
        let machine = Machine::from_str(input).unwrap();
        let startup = machine.startup();
        assert_eq!(startup, expected_startup);
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 11)]
    #[case(
        "[.#..#.###] (0,3,5,6,7) (2,3,4,5,6,8) (0,1,2,3,4,5,6,8) (0,1,2,3) (0,1,2,6) (0,1,3,7,8) (0,1,5,6,8) (4,8) (0,2,3) (5,6) (0,3,4,6) {279,232,41,255,52,73,95,202,239}",
        292
    )]
    fn test_joltage(#[case] input: &str, #[case] expected_joltage: u64) {
        let machine = Machine::from_str(input).unwrap();
        let joltage = machine.joltage_rating();
        assert_eq!(joltage, expected_joltage);
    }

    #[rstest]
    fn test_part_a() {
        let input = read_test_day_input("10");
        let machines = parse(&input);
        let result = part_a(&machines);
        assert_eq!(result, 7);
    }
    #[rstest]
    fn test_part_b() {
        let input = read_test_day_input("10");
        let machines = parse(&input);
        let result = part_b(&machines);
        assert_eq!(result, 33);
    }
}
