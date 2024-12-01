use crate::days::RE_DIGITS;

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    record: u64,
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, records) = input.split_once('\n').unwrap();
    let times: Vec<u64> = RE_DIGITS
        .captures_iter(times)
        .map(|s| s.get(0).unwrap().as_str().parse().unwrap())
        .collect();
    let records: Vec<u64> = RE_DIGITS
        .captures_iter(records)
        .map(|s| s.get(0).unwrap().as_str().parse().unwrap())
        .collect();
    times
        .into_iter()
        .zip(records)
        .map(|(time, record)| Race { time, record })
        .collect()
}

impl Race {
    fn number_of_possible_wins(&self) -> u64 {
        let t = self.time as f64;
        let r = self.record as f64;

        let d = (t.powf(2.0) - 4.0 * r).sqrt();
        let l_bound = 0.5 * (t - d);
        let r_bound = 0.5 * (t + d);
        (r_bound).ceil() as u64 - (l_bound + 1.0).floor() as u64
    }
}

fn part_a(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|r| r.number_of_possible_wins())
        .product()
}

fn parse_input_2(input: &str) -> Race {
    let (times, records) = input.split_once('\n').unwrap();
    let time = times
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |a, b| a * 10 + b.to_digit(10).unwrap() as u64);
    let record = records
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0u64, |a, b| a * 10 + b.to_digit(10).unwrap() as u64);
    Race { time, record }
}

fn part_b(input: &str) -> u64 {
    parse_input_2(input).number_of_possible_wins()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "Time:      7  15   30\nDistance:  9  40  200";
    }

    #[rstest]
    fn test_parse_input(example_input: &str) {
        assert_eq!(
            parse_input(example_input),
            vec![
                Race { time: 7, record: 9 },
                Race {
                    time: 15,
                    record: 40
                },
                Race {
                    time: 30,
                    record: 200
                },
            ]
        )
    }

    #[rstest]
    #[case(Race { time: 7, record: 9 }, 4)]
    #[case(Race { time: 15, record: 40 }, 8)]
    #[case(Race { time: 30, record: 200 }, 9)]
    #[case(Race { time: 71530, record: 940200 }, 71503)]
    fn test_number_of_possible_wins(#[case] race: Race, #[case] n_wins: u64) {
        assert_eq!(race.number_of_possible_wins(), n_wins);
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        assert_eq!(part_a(example_input), 288)
    }

    #[rstest]
    fn test_parse_input_2(example_input: &str) {
        assert_eq!(
            parse_input_2(example_input),
            Race {
                time: 71530,
                record: 940200
            }
        )
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        assert_eq!(part_b(example_input), 71503)
    }
}
