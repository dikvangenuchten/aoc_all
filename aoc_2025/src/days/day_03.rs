use anyhow::Result;
use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let banks = parse(input_file);
    let a = part_a(&banks);
    let b = part_b(&banks);
    (a, b)
}

pub fn part_a(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| bank.max_joltage(2)).sum()
}

pub fn part_b(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| bank.max_joltage(12)).sum()
}

#[derive(Debug, PartialEq)]
pub struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn max_joltage(&self, n: usize) -> u64 {
        Bank::_max_joltage_recursion(&self.batteries, n)
    }

    fn _max_joltage_recursion(batteries: &[u8], n: usize) -> u64 {
        let (idx, first_max) = batteries[0..batteries.len() - (n - 1)]
            .iter()
            .enumerate()
            .max_by_key(|(_idx, val)| (**val, -(*_idx as i32)))
            .unwrap();
        if n == 1 {
            return *first_max as u64;
        }
        let remaining = Bank::_max_joltage_recursion(&batteries[idx + 1..], n - 1);
        *first_max as u64 * 10u64.pow((n - 1) as u32) + remaining
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|num_str| Ok(num_str.to_digit(10).ok_or(())? as u8))
            .collect::<Result<Vec<u8>, ()>>()?;
        Ok(Bank { batteries })
    }
}

fn parse(input_file: &str) -> Vec<Bank> {
    input_file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Bank>>()
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", Bank { batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1] })]
    #[case("811111111111119", Bank { batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9] })]
    #[case("234234234234278", Bank { batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8] })]
    #[case("818181911112111", Bank { batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1] })]
    fn test_bank_from_str(#[case] s: &str, #[case] expected_bank: Bank) {
        let bank: Bank = s.parse().unwrap();
        assert_eq!(bank, expected_bank);
    }

    #[rstest]
    #[case(Bank { batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1] }, 2, 98)]
    #[case(Bank { batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9] }, 2, 89)]
    #[case(Bank { batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8] }, 2, 78)]
    #[case(Bank { batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1] }, 2, 92)]
    #[case(Bank { batteries: vec![8,1,9,1,8,1,9,1,1,1,1,2,1,1,1] }, 2, 99)]
    #[case(Bank { batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1] }, 12, 987654321111)]
    fn test_bank_max_joltage(#[case] bank: Bank, #[case] n: usize, #[case] expected_max: u64) {
        assert_eq!(bank.max_joltage(n), expected_max);
    }

    #[rstest]
    fn test_day_03_a() {
        let input_file = read_test_day_input("03");
        let banks = parse(&input_file);
        let result = part_a(&banks);
        assert_eq!(result, 357);
    }

    #[rstest]
    fn test_day_03_b() {
        let input_file = read_test_day_input("03");
        let banks = parse(&input_file);
        let result = part_b(&banks);
        assert_eq!(result, 3121910778619);
    }
}
