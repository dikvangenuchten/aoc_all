use std::{
    cmp::{max, min},
    collections::HashSet,
    num::ParseIntError,
    str::FromStr,
};

pub fn solve(input: &str) -> (usize, usize) {
    let movements = parse_input(input);
    let part_1 = solve_part_1(&movements);
    let part_2 = solve_part_2(&movements);
    (part_1, part_2)
}

fn solve_part_1(movements: &Vec<Movement>) -> usize {
    let rope = vec![(0, 0); 2];
    simulate_movement(rope, movements).len()
}

fn solve_part_2(movements: &Vec<Movement>) -> usize {
    let rope = vec![(0, 0); 10];
    simulate_movement(rope, movements).len()
}

fn simulate_movement(mut rope: Vec<(i32, i32)>, movements: &Vec<Movement>) -> HashSet<(i32, i32)> {
    let mut tail_locs = HashSet::from([(0, 0)]);
    for movement in movements {
        for _ in 0..movement.size() {
            match movement {
                Movement::R(_) => rope[0].0 += 1,
                Movement::U(_) => rope[0].1 += 1,
                Movement::L(_) => rope[0].0 -= 1,
                Movement::D(_) => rope[0].1 -= 1,
            }
            let mut locs = HashSet::new();
            for i in 1..rope.len() {
                (rope[i], locs) = tail_movement(rope[i - 1], rope[i]);
            }
            tail_locs.extend(locs);
        }
    }
    tail_locs
}

fn tail_movement(head: (i32, i32), mut tail: (i32, i32)) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut tail_visits = HashSet::new();
    if (manhattan_distance(head, tail)) > 2 {
        // Disconnected
        let x_dist = (head.0 - tail.0).abs();
        let y_dist = (head.1 - tail.1).abs();
        if x_dist < y_dist {
            tail.0 = head.0;
        }
        if y_dist < x_dist {
            tail.1 = head.1;
        }
    }

    if (head.0 - tail.0).abs() > 1 {
        let dist = head.0 - tail.0;

        let min_ = min(head.0, tail.0);
        let max_ = max(head.0, tail.0) - 1;
        tail.0 = head.0 - dist.signum();
        tail_visits = (min_..max_).map(|i| (i + 1, tail.1)).collect();
    }
    if (head.1 - tail.1).abs() > 1 {
        let dist = head.1 - tail.1;

        let min_ = min(head.1, tail.1);
        let max_ = max(head.1, tail.1) - 1;
        tail.1 = head.1 - dist.signum();
        tail_visits = (min_..max_).map(|i| (tail.0, i + 1)).collect();
    }
    (tail, tail_visits)
}

fn parse_input(input_str: &str) -> Vec<Movement> {
    input_str
        .trim()
        .split('\n')
        .map(Movement::from_str)
        .collect::<Result<Vec<Movement>, ParseIntError>>()
        .unwrap()
}

fn manhattan_distance(left: (i32, i32), right: (i32, i32)) -> i32 {
    (left.0 - right.0).abs() + (left.1 - right.1).abs()
}

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    R(u32),
    U(u32),
    D(u32),
    L(u32),
}

impl Movement {
    fn size(&self) -> u32 {
        *match self {
            Movement::R(x) => x,
            Movement::U(x) => x,
            Movement::D(x) => x,
            Movement::L(x) => x,
        }
    }
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let str = s.split_once(' ');
        Ok(match str {
            Some(("R", x)) => Movement::R(x.parse::<u32>()?),
            Some(("U", x)) => Movement::U(x.parse::<u32>()?),
            Some(("D", x)) => Movement::D(x.parse::<u32>()?),
            Some(("L", x)) => Movement::L(x.parse::<u32>()?),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("R 4", Movement::R(4))]
    #[case("U 4", Movement::U(4))]
    #[case("L 3", Movement::L(3))]
    #[case("D 1", Movement::D(1))]
    #[case("R 4", Movement::R(4))]
    #[case("D 1", Movement::D(1))]
    #[case("L 5", Movement::L(5))]
    #[case("R 2", Movement::R(2))]
    fn test_parse_input_single(#[case] input_str: &str, #[case] expected: Movement) {
        assert_eq!(Movement::from_str(input_str), Ok(expected))
    }

    #[rstest]
    #[case("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2", 13)]
    fn test_solve_part_1(#[case] input_str: &str, #[case] expected: usize) {
        let input = parse_input(input_str);
        assert_eq!(solve_part_1(&input), expected)
    }

    #[rstest]
    #[case("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2", 1)]
    #[case("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20", 36)]
    fn test_solve_part_2(#[case] input_str: &str, #[case] expected: usize) {
        let input = parse_input(input_str);
        assert_eq!(solve_part_2(&input), expected)
    }

    #[fixture]
    fn actual_input() -> String {
        read_day_input("day_09")
    }

    #[fixture]
    fn movements(actual_input: String) -> Vec<Movement> {
        parse_input(&actual_input)
    }

    #[rstest]
    fn test_solve_part_1_actual(movements: Vec<Movement>) {
        assert_eq!(solve_part_1(&movements), 6522)
    }

    #[rstest]
    fn test_solve_part_2_actual(movements: Vec<Movement>) {
        assert_eq!(solve_part_2(&movements), 2717)
    }
}
