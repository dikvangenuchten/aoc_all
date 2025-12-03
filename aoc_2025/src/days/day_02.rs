use anyhow::{Error, Result, anyhow};
use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let ranges = parse(input_file);
    let a = part_a(&ranges);
    let b = part_b(&ranges);
    (a, b)
}

pub fn part_a(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.sum_invalid()).sum()
}

pub fn part_b(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.sum_invalid_b()).sum()
}

#[derive(Debug, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn sum_invalid(&self) -> u64 {
        (self.start..=self.end).filter(check_invalid_part_a).sum()
    }

    fn sum_invalid_b(&self) -> u64 {
        (self.start..=self.end).filter(check_invalid_part_b).sum()
    }
}

fn check_invalid_part_a(n: &u64) -> bool {
    let s = n.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let half_len = s.len() / 2;
    let (first_half, second_half) = s.split_at(half_len);
    first_half == second_half
}

fn check_invalid_part_b(n: &u64) -> bool {
    let s = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>();
    for i in 1..(s.len() / 2 + 1) {
        // Cannot have a repeating pattern
        if s.len() % i != 0 {
            continue;
        }
        let pattern = &s.clone()[0..i];
        if s.chunks(i).all(|w| *w == *pattern) {
            return true;
        }
    }
    false
}

impl FromStr for Range {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim_end();
        if let Some((start, end)) = s.split_once('-') {
            Ok(Range::new(start.parse::<u64>()?, end.parse::<u64>()?))
        } else {
            Err(anyhow!("Could not parse: {}.", s))
        }
    }
}

fn parse(input_file: &str) -> Vec<Range> {
    input_file
        .split(',')
        .map(|line| Range::from_str(line).unwrap_or_else(|_| panic!("Could not parse {}.", line)))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("11-22", Range::new(11, 22))]
    #[case("2121212118-2121212124", Range::new(2121212118, 2121212124))]
    fn test_parse_single(#[case] input: &str, #[case] range: Range) {
        assert_eq!(Range::from_str(input).unwrap(), range);
    }

    #[rstest]
    fn test_parse_test_input() {
        let input = read_test_day_input("02");
        assert_eq!(
            parse(&input),
            vec![
                Range::new(11, 22),
                Range::new(95, 115),
                Range::new(998, 1012),
                Range::new(1188511880, 1188511890),
                Range::new(222220, 222224),
                Range::new(1698522, 1698528),
                Range::new(446443, 446449),
                Range::new(38593856, 38593862),
                Range::new(565653, 565659),
                Range::new(824824821, 824824827),
                Range::new(2121212118, 2121212124),
            ]
        );
    }

    #[rstest]
    #[case(Range::new(11, 22), 11 + 22)]
    #[case(Range::new(95, 115), 99)]
    #[case(Range::new(998, 1012), 1010)]
    #[case(Range::new(1188511880, 1188511890), 1188511885)]
    #[case(Range::new(222220, 222224), 222222)]
    #[case(Range::new(1698522, 1698528), 0)]
    #[case(Range::new(446443, 446449), 446446)]
    #[case(Range::new(38593856, 38593862), 38593859)]
    #[case(Range::new(565653, 565659), 0)]
    #[case(Range::new(824824821, 824824827), 0)]
    #[case(Range::new(2121212118, 2121212124), 0)]
    fn test_sum_invalid(#[case] range: Range, #[case] expected: u64) {
        assert_eq!(range.sum_invalid(), expected);
    }

    #[rstest]
    fn test_day_02_a() {
        let input_file = read_test_day_input("02");
        let ranges = parse(&input_file);
        let result = part_a(&ranges);
        assert_eq!(result, 1227775554);
    }

    #[rstest]
    #[case(11, true)]
    #[case(22, true)]
    #[case(999, true)]
    #[case(1010, true)]
    #[case(12, false)]
    fn test_check_invalid_b(#[case] n: u64, #[case] expected: bool) {
        assert_eq!(check_invalid_part_b(&n), expected);
    }

    #[rstest]
    fn test_day_02_b() {
        let input_file = read_test_day_input("02");
        let ranges = parse(&input_file);
        let result = part_b(&ranges);
        assert_eq!(result, 4174379265);
    }
}
