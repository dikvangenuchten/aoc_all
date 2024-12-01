use lazy_static::lazy_static;
use phf::phf_map;

use regex::{Captures, Regex};

const DIGIT_MAP_REPLACE: phf::Map<&str, &str> = phf_map! {
    "one" => "o1e",
    "two" => "t2o",
    "three" => "thr3e",
    "four" => "fo4r",
    "five" => "fi5e",
    "six" => "s6x",
    "seven" => "sev7n",
    "eight" => "eig8t",
    "nine" => "ni9e",
};

lazy_static! {
    static ref REPLACE_REGEX: Regex =
        Regex::new("(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
}

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

pub fn part_a(input: &str) -> u64 {
    input.trim().split('\n').map(decode_line).sum()
}

pub fn part_b(input: &str) -> u64 {
    part_a(&replace_2_pass(input))
}

fn replacer(capture: &Captures) -> String {
    (*DIGIT_MAP_REPLACE
        .get(capture.get(0).unwrap().as_str())
        .unwrap())
    .into()
}

fn replace_2_pass(input: &str) -> String {
    let input = REPLACE_REGEX.replace_all(input, replacer);
    let input = REPLACE_REGEX.replace_all(&input, replacer);
    input.to_string()
}

fn decode_line(input: &str) -> u64 {
    let first_number: u64 = input
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap_or_else(|| panic!("Invalid input: {input}"))
        .to_digit(10)
        .unwrap()
        .into();
    let last_number: u64 = input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .next_back()
        .unwrap_or_else(|| panic!("Invalid input: {input}"))
        .to_digit(10)
        .unwrap()
        .into();
    10 * first_number + last_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case("1test1", 11)]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_decode_line(#[case] input: &str, #[case] expected_output: u64) {
        assert_eq!(decode_line(input), expected_output)
    }

    #[fixture]
    fn test_input_a() -> &'static str {
        return "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
    }

    #[rstest]
    fn test_part_1(test_input_a: &str) {
        assert_eq!(part_a(test_input_a), 142);
    }

    #[rstest]
    #[case("one", 11)]
    #[case("two", 22)]
    #[case("three", 33)]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("eightwo", 82)]
    fn test_decode_line_b(#[case] input: &str, #[case] expected_output: u64) {
        assert_eq!(decode_line(&replace_2_pass(input)), expected_output)
    }
}
