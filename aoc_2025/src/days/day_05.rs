use anyhow::Result;
use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let (ranges, ids) = parse(input_file);
    let a = part_a(&ranges, &ids);
    let b = part_b(ranges);
    (a, b)
}

pub fn part_a(ranges: &[Range], ids: &[u64]) -> u64 {
    let mut count = 0;
    for id in ids {
        for range in ranges {
            if range.contains(*id) {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn part_b(mut ranges: Vec<Range>) -> u64 {
    let mut count = 0;
    loop {
        let init_len = ranges.len();
        ranges = merge_ranges(ranges);
        if ranges.len() == init_len {
            break;
        }
        count += 1;
    }
    println!("Merged {} times", count);
    ranges.iter().map(|r| r.len()).sum()
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort();
    for i in 0..ranges.len() {
        let mut offset = 0;
        for j in i+1..ranges.len() {
            if let Some(merged) = ranges[i].merge(&ranges[j - offset]) {
                ranges[i] = merged;
                ranges.remove(j - offset);
                offset += 1;
            }
        }
    }
    ranges
}

fn parse(input_file: &str) -> (Vec<Range>, Vec<u64>) {
    if let Some((ranges, ids)) = input_file.split_once("\n\n") {
        let ranges = ranges.trim().split('\n').map(|r| Range::from_str(r)).collect::<Result<Vec<_>, _>>().unwrap();
        let ids = ids.trim().lines().map(|id| id.parse::<u64>()).collect::<Result<Vec<_>, _>>().unwrap();
        return (ranges, ids);
    }
    panic!("Invalid input format");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }

    fn len(&self) -> u64 {
        self.end - self.start + 1
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Range {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or_else(|| anyhow::anyhow!("Invalid range format"))?;
        Ok(Range {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let input_file = read_test_day_input("05");
        let (ranges, ids) = parse(&input_file);
        assert_eq!(
            ranges,
            vec![
                Range { start: 3, end: 5 },
                Range { start: 10, end: 14 },
                Range { start: 16, end: 20 },
                Range { start: 12, end: 18 },
            ]
        );
        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32,]);
    }

    #[rstest]
    fn test_part_a() {
        let input_file = read_test_day_input("05");
        let (ranges, ids) = parse(&input_file);
        assert_eq!(part_a(&ranges, &ids), 3);
    }

    #[rstest]
    fn test_part_b() {
        let input_file = read_test_day_input("05");
        let (ranges, _) = parse(&input_file);
        assert_eq!(part_b(ranges), 14);
    }
}
