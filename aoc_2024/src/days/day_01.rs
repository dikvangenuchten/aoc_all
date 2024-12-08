use std::collections::HashMap;

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let a = part_a(input_file);
    let b = part_b(input_file);
    (a, b)
}

pub fn part_a(input_file: &str) -> u32 {
    let (mut left, mut right) = parse(input_file);
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .fold(0, |tot, (l, r)| tot + l.abs_diff(*r))
}

pub fn part_b(input_file: &str) -> u32 {
    let (left, right) = parse(input_file);
    let mut right_counts = HashMap::<u32, u32>::with_capacity(right.len());
    for number in right {
        right_counts
            .entry(number)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    left.iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0))
        .sum()
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .trim()
        .split("\n")
        .map(|w| {
            w.split_ascii_whitespace()
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|line| (line[0], line[1]))
        .unzip();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let result = parse(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result.0, vec![3, 4, 2, 1, 3, 3], "Vec 1 is not equal");
        assert_eq!(result.1, vec![4, 3, 5, 3, 9, 3], "Vec 2 is not equal")
    }

    #[rstest]
    fn test_day_01_a() {
        let result = part_a(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 11)
    }

    #[rstest]
    fn test_day_01_b() {
        let result = part_b(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, 31)
    }
}
