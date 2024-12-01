use std::{collections::VecDeque, str::FromStr, string::ParseError};

use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &str) -> (usize, usize) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .trim()
        .split("\n\n")
        .map(Monkey::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn solve_part_1(input_str: &str) -> usize {
    let mut monkeys = parse_input(input_str);
    for _ in 0..20 {
        monkeys = do_round(monkeys);
    }
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn solve_part_2(input_str: &str) -> usize {
    let mut monkeys = parse_input(input_str);
    let modulo: usize = monkeys.iter().map(|m| m.div_check).product();
    for _ in 0..10000 {
        monkeys = {
            let mut monkeys = monkeys;
            for i in 0..monkeys.len() {
                while !monkeys[i].items.is_empty() {
                    let item = monkeys[i].items.pop_front().unwrap();
                    let wl = monkeys[i].operation.new_wory_level(item);
                    let wl = wl % modulo;
                    monkeys[i].inspected += 1;
                    if wl % monkeys[i].div_check == 0 {
                        let j = monkeys[i].true_monkey;
                        monkeys[j as usize].items.push_back(wl);
                    } else {
                        let j = monkeys[i].false_monkey;
                        monkeys[j as usize].items.push_back(wl);
                    };
                }
            }
            monkeys
        };
    }
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn do_round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            let item = monkeys[i].items.pop_front().unwrap();
            let wl = monkeys[i].operation.new_wory_level(item);
            let wl = wl / 3;
            monkeys[i].inspected += 1;
            if wl % monkeys[i].div_check == 0 {
                let j = monkeys[i].true_monkey;
                monkeys[j as usize].items.push_back(wl);
            } else {
                let j = monkeys[i].false_monkey;
                monkeys[j as usize].items.push_back(wl);
            };
        }
    }
    monkeys
}

#[derive(Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    div_check: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspected: usize,
}
#[derive(Debug, PartialEq, Eq)]
struct Operation {
    left: String,
    operator: String,
    right: String,
}

impl Operation {
    fn new_wory_level(&self, old: usize) -> usize {
        let left = match self.left.as_str() {
            "old" => old,
            x => x.parse().unwrap(),
        };
        let right = match self.right.as_str() {
            "old" => old,
            x => x.parse().unwrap(),
        };

        match self.operator.as_str() {
            "*" => left * right,
            "+" => left + right,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, operator, right): (&str, &str, &str) = s
            .split_once('=')
            .unwrap()
            .1
            .trim()
            .split(' ')
            .collect_tuple()
            .unwrap();
        Ok(Operation {
            left: left.to_string(),
            operator: operator.to_string(),
            right: right.to_string(),
        })
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_items_re = Regex::new(r"Starting items:((?: \d+,*)+)").unwrap();
        let items: VecDeque<usize> = start_items_re
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .split(',')
            .map(|item| item.trim().parse::<usize>().unwrap())
            .collect();

        let operation_re = Regex::new(r"Operation: (.*)\n").unwrap();
        let operation_str = operation_re.captures(s).unwrap().get(1).unwrap().as_str();
        let operation = Operation::from_str(operation_str).unwrap();

        let div_check_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
        let div_check = div_check_re
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let true_monkey_re = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
        let true_monkey: usize = true_monkey_re
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let false_monkey_re = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        let false_monkey: usize = false_monkey_re
            .captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            operation,
            div_check,
            true_monkey,
            false_monkey,
            inspected: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3", 
Monkey {
    items: VecDeque::from(vec![79, 98]),
    operation: Operation { left: "old".to_string(), operator: "*".to_string(), right: "19".to_string()},
    div_check: 23,
    true_monkey: 2,
    false_monkey: 3,
    inspected: 0
})]
    fn test_parse_monkey(#[case] input: &str, #[case] expected: Monkey) {
        assert_eq!(Monkey::from_str(input), Ok(expected))
    }

    #[fixture]
    fn test_input() -> String {
        read_day_input("test_day_11")
    }

    #[fixture]
    fn test_monkeys(test_input: String) -> Vec<Monkey> {
        parse_input(&test_input)
    }

    #[rstest]
    fn test_round(mut test_monkeys: Vec<Monkey>) {
        for _ in 0..20 {
            test_monkeys = do_round(test_monkeys);
        }
        assert_eq!(
            test_monkeys[0].items,
            VecDeque::from(vec![10, 12, 14, 26, 34])
        );
        assert_eq!(test_monkeys[0].inspected, 101);
        assert_eq!(test_monkeys[1].inspected, 95);
        assert_eq!(test_monkeys[2].inspected, 7);
        assert_eq!(test_monkeys[3].inspected, 105);
    }

    #[rstest]
    fn test_solve_part_1(test_input: String) {
        assert_eq!(solve_part_1(&test_input), 10605)
    }
}
