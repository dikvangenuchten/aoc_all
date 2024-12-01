use std::collections::VecDeque;

use itertools::{
    self,
    FoldWhile::{Continue, Done},
    Itertools,
};

pub fn solve(input_str: &str) -> (usize, usize) {
    let part_1 = solve_part_1(input_str);
    let part_2 = solve_part_2(input_str);
    (part_1, part_2)
}

fn solve_part_1(input_str: &str) -> usize {
    input_str
        .chars()
        .tuple_windows::<(char, char, char, char)>()
        .enumerate()
        .find(|(_, (a, b, c, d))| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap()
        .0
        + 4
}

fn solve_part_2(input_str: &str) -> usize {
    input_str
        .chars()
        .enumerate()
        .fold_while(
            (0, VecDeque::with_capacity(15)),
            |(_, mut queue), (i, char)| {
                queue.push_back(char);
                if queue.len() < 15 {
                    return Continue((i, queue));
                }
                queue.pop_front();
                match queue.iter().all_unique() {
                    true => Done((i, queue)),
                    false => Continue((i, queue)),
                }
            },
        )
        .into_inner()
        .0
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part_1(#[case] input_str: &str, #[case] expected: usize) {
        assert_eq!(solve_part_1(input_str), expected)
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_part_2(#[case] input_str: &str, #[case] expected: usize) {
        assert_eq!(solve_part_2(input_str), expected)
    }
}
