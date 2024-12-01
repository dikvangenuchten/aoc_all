use std::{str::FromStr, string::ParseError};

use itertools::Itertools;
use serde_json::Value;

pub fn solve(input: &str) -> (usize, usize) {
    let mut pairs = parse_input(input);
    let part_1 = solve_part_1(&pairs);
    let part_2 = solve_part_2(&mut pairs);
    (part_1, part_2)
}

fn solve_part_1(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn solve_part_2(pairs: &mut Vec<(Packet, Packet)>) -> usize {
    let distres_packet = (
        Packet::List(vec![Packet::Val(2)]),
        Packet::List(vec![Packet::Val(6)]),
    );
    pairs.push(distres_packet.clone());
    pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .sorted()
        .enumerate()
        .filter(|(_, packet)| &&distres_packet.0 == packet || &&distres_packet.1 == packet)
        .map(|(i, _)| i + 1)
        .product()
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input.trim().split("\n\n").map(parse_pair).collect()
}

fn parse_pair(input: &str) -> (Packet, Packet) {
    let (l, r) = input.split_once('\n').unwrap();
    (Packet::from_str(l).unwrap(), Packet::from_str(r).unwrap())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Val(u64),
}

impl FromStr for Packet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Packet::from_value(&serde_json::from_str(s).unwrap()))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Val(x), Packet::Val(y)) => x.cmp(y),
            (Packet::List(x), Packet::List(y)) => {
                for (l, r) in x.iter().zip(y) {
                    match l.cmp(r) {
                        std::cmp::Ordering::Equal => continue,
                        not_equal => return not_equal,
                    }
                }
                x.len().cmp(&y.len())
            }
            (Packet::List(_), Packet::Val(y)) => self.cmp(&Packet::List(vec![Packet::Val(*y)])),
            (Packet::Val(x), Packet::List(_)) => Packet::List(vec![Packet::Val(*x)]).cmp(other),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn from_value(val: &Value) -> Packet {
        match val {
            Value::Number(x) => Packet::Val(x.as_u64().unwrap()),
            Value::Array(x) => Packet::List(x.iter().map(Packet::from_value).collect()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[fixture]
    fn example_input_str() -> String {
        read_day_input("test_day_13")
    }

    #[fixture]
    fn example_pairs(example_input_str: String) -> Vec<(Packet, Packet)> {
        parse_input(&example_input_str)
    }

    #[rstest]
    #[case("[1,1,3,1,1]", Packet::List(vec![
        Packet::Val(1),
        Packet::Val(1),
        Packet::Val(3),
        Packet::Val(1),
        Packet::Val(1),
    ]))]
    #[case("[1,1,5,1,1]", Packet::List(vec![
        Packet::Val(1),
        Packet::Val(1),
        Packet::Val(5),
        Packet::Val(1),
        Packet::Val(1),
    ]))]
    #[case("[[1],[2,3,4]]", Packet::List(vec![
        Packet::List(vec![Packet::Val(1)]),
        Packet::List(vec![Packet::Val(2), Packet::Val(3), Packet::Val(4)])
    ]))]

    fn test_parse_packet(#[case] packet_str: &str, #[case] expected_packet: Packet) {
        assert_eq!(Packet::from_str(packet_str), Ok(expected_packet))
    }

    #[rstest]
    #[case("[1,1,3,1,1]", "[1,1,5,1,1]", true)]
    #[case("[[1],[2,3,4]]", "[[1],4]", true)]
    #[case("[9]", "[[8,7,6]]", false)]
    #[case("[[4,4],4,4]", "[[4,4],4,4,4]", true)]
    #[case("[7,7,7,7]", "[7,7,7]", false)]
    #[case("[]", "[3]", true)]
    #[case("[[[]]]", "[[]]", false)]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", false)]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[[1],4]", true)]
    fn test_pair_right_order(#[case] left: &str, #[case] right: &str, #[case] correct_order: bool) {
        let left = Packet::from_str(left).unwrap();
        let right = Packet::from_str(right).unwrap();
        assert_eq!(left < right, correct_order)
    }

    #[rstest]
    fn test_solve_part_1(example_pairs: Vec<(Packet, Packet)>) {
        assert_eq!(solve_part_1(&example_pairs), 13)
    }

    #[rstest]
    fn test_solve_part_2(mut example_pairs: Vec<(Packet, Packet)>) {
        assert_eq!(solve_part_2(&mut example_pairs), 140)
    }
}
