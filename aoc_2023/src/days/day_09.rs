use lazy_static::lazy_static;
use regex::Regex;
use std::{iter::Sum, str::FromStr};

pub fn solve_day(input: &str) -> (u64, u64) {
    let sequences = parse_input(input);
    let prediction: Prediction = sequences.iter().map(|s| s.predict_both()).sum();
    (
        prediction.part_a.try_into().unwrap(),
        prediction.part_b.try_into().unwrap(),
    )
}

#[derive(Debug, PartialEq, Eq)]
struct Prediction {
    part_a: i64,
    part_b: i64,
}

impl Sum for Prediction {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            Prediction {
                part_a: 0,
                part_b: 0,
            },
            |mut acc_p, prediction| {
                acc_p.part_a += prediction.part_a;
                acc_p.part_b += prediction.part_b;
                acc_p
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sequence {
    vals: Vec<i64>,
}

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(-?\d+)").unwrap();
}

impl FromStr for Sequence {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            vals: RE_DIGITS
                .captures_iter(s)
                .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
                .collect(),
        })
    }
}

impl Sequence {
    fn predict(&self) -> i64 {
        let vals: Vec<i64> = self
            .vals
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        if vals.iter().all(|x| x == &0) {
            return self.vals[self.vals.len() - 1];
        }
        let s = Sequence { vals };
        let p = s.predict();

        self.vals[self.vals.len() - 1] + p
    }

    fn predict_back(&self) -> i64 {
        let vals: Vec<i64> = self
            .vals
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        if vals.iter().all(|x| x == &0) {
            return self.vals[0];
        }
        let s = Sequence { vals };
        let p = s.predict_back();

        self.vals[0] - p
    }

    fn predict_both(&self) -> Prediction {
        let vals: Vec<i64> = self
            .vals
            .windows(2)
            .map(|slice| slice[1] - slice[0])
            .collect();
        if vals.iter().all(|x| x == &0) {
            return Prediction {
                part_a: self.vals[self.vals.len() - 1],
                part_b: self.vals[0],
            };
        }
        let s = Sequence { vals };
        let pred = s.predict_both();

        Prediction {
            part_a: self.vals[self.vals.len() - 1] + pred.part_a,
            part_b: self.vals[0] - pred.part_b,
        }
    }
}

fn parse_input(input: &str) -> Vec<Sequence> {
    input
        .trim()
        .split('\n')
        .map(|s| Sequence::from_str(s).unwrap())
        .collect()
}

#[allow(dead_code)]
fn part_a(sequences: &[Sequence]) -> i64 {
    sequences.iter().map(|s| s.predict()).sum()
}

#[allow(dead_code)]

fn part_b(sequences: &[Sequence]) -> i64 {
    sequences.iter().map(|s| s.predict_back()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
    }

    #[fixture]
    fn sequences(example_input: &str) -> Vec<Sequence> {
        return parse_input(example_input);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", Sequence { vals: vec![0,3,6,9,12,15]})]
    #[case("1 3 6 10 15 21", Sequence { vals: vec![1, 3, 6, 10, 15, 21]})]
    #[case("10 13 16 21 30 45", Sequence { vals: vec![10, 13, 16, 21, 30, 45]})]
    fn test_parse_sequence(#[case] input: &str, #[case] expected: Sequence) {
        assert_eq!(Sequence::from_str(input).unwrap(), expected)
    }

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    fn test_predict(#[case] sequence: Sequence, #[case] expected: i64) {
        assert_eq!(sequence.predict(), expected)
    }

    #[rstest]
    fn test_part_a(sequences: Vec<Sequence>) {
        assert_eq!(part_a(&sequences), 114)
    }

    #[rstest]
    #[case("0 3 6 9 12 15", -3)]
    #[case("1 3 6 10 15 21", 0)]
    #[case("10 13 16 21 30 45", 5)]
    fn test_predict_back(#[case] sequence: Sequence, #[case] expected: i64) {
        assert_eq!(sequence.predict_back(), expected)
    }

    #[rstest]
    #[case("0 3 6 9 12 15", -3, 18)]
    #[case("1 3 6 10 15 21", 0, 28)]
    #[case("10 13 16 21 30 45", 5, 68)]
    fn test_predict_both(#[case] sequence: Sequence, #[case] part_b: i64, #[case] part_a: i64) {
        assert_eq!(sequence.predict_both(), Prediction { part_a, part_b })
    }

    #[rstest]
    fn test_part_b(sequences: Vec<Sequence>) {
        assert_eq!(part_b(&sequences), 2)
    }

    #[rstest]
    fn test_day(example_input: &str) {
        assert_eq!(solve_day(example_input), (114, 2))
    }
}
