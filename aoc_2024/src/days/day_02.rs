use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let a = part_a(&input_file);
    let b = part_b(&input_file);
    (a, b)
}

pub fn part_a(input_file: &str) -> u32 {
    let reports = parse(input_file);
    reports.iter().filter(|r| r.is_safe_a()).count() as u32
}

pub fn part_b(input_file: &str) -> u32 {
    let reports = parse(input_file);
    reports.iter().filter(|r| r.is_safe_b()).count() as u32
}

fn parse(input_file: &str) -> Vec<Report> {
    input_file
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe_a(&self) -> bool {
        check_safe_levels(&self.levels)
    }

    fn is_safe_b(&self) -> bool {
        // safe b is stricter, thus if a is already fine we are good.
        if self.is_safe_a() {
            return true;
        }

        for i in 0..self.levels.len() {
            // Check if it is safe after removing given index
            let levels = [&self.levels[..i], &self.levels[(i + 1)..]].concat();
            if check_safe_levels(&levels) {
                return true;
            }
        }

        false
    }
}

fn check_safe_levels(levels: &[u32]) -> bool {
    let mut increasing: Option<bool> = None;
    for l in levels.windows(2) {
        let prev = l[0];
        let next = l[1];
        if prev.abs_diff(next) == 0 || prev.abs_diff(next) > 3 {
            return false;
        }
        if prev > next {
            if let Some(increasing) = increasing {
                // increasing is true, but now we are decreasing
                if increasing {
                    return false;
                }
            } else {
                // prev > next => decreasing
                increasing = Some(false)
            }
        } else if let Some(increasing) = increasing {
            // increasing is false, but now we are increasing
            if !increasing {
                return false;
            }
        } else {
            // prev > next => decreasing
            increasing = Some(true)
        }
    }
    true
}

#[derive(Debug, PartialEq, Eq)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels: Result<Vec<u32>, _> = s
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>())
            .collect();
        if let Ok(levels) = levels {
            return Ok(Report { levels });
        }
        dbg!("Could not parse: {:?}", s);
        Err(ParseReportError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1", Report { levels: vec![7, 6, 4, 2, 1]})]
    #[case("1 2 7 8 9", Report { levels: vec![1, 2, 7, 8, 9]})]
    #[case("9 7 6 2 1", Report { levels: vec![9, 7, 6, 2, 1]})]
    #[case("1 3 2 4 5", Report { levels: vec![1, 3, 2, 4, 5]})]
    #[case("8 6 4 4 1", Report { levels: vec![8, 6, 4, 4, 1]})]
    #[case("1 3 6 7 9", Report { levels: vec![1, 3, 6, 7, 9]})]
    fn test_parse_line(#[case] line: &str, #[case] report: Report) {
        assert_eq!(Report::from_str(line), Ok(report))
    }

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", false)]
    #[case("8 6 4 4 1", false)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe_a(#[case] report: Report, #[case] should_be_safe: bool) {
        assert_eq!(report.is_safe_a(), should_be_safe)
    }

    #[rstest]
    #[case("7 6 4 2 1", true)]
    #[case("1 2 7 8 9", false)]
    #[case("9 7 6 2 1", false)]
    #[case("1 3 2 4 5", true)]
    #[case("8 6 4 4 1", true)]
    #[case("1 3 6 7 9", true)]
    fn test_is_safe_b(#[case] report: Report, #[case] should_be_safe: bool) {
        assert_eq!(report.is_safe_b(), should_be_safe)
    }

    #[rstest]
    fn test_full_parse() {
        let expected = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];
        let actual = parse(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(expected, actual)
    }

    #[rstest]
    fn test_day_02_a() {
        let result = part_a(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 2)
    }

    #[rstest]
    fn test_day_02_b() {
        let result = part_b(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, 4)
    }
}
