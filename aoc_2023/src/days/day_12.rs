use std::{collections::HashMap, hash::Hash, str::FromStr};

use nom::Slice;
use tqdm::Iter;

type State<'a> = (Spring, &'a [Spring], &'a [u64], u64);

pub fn solve_day(input: &str) -> (u64, u64) {
    let lines = input
        .trim()
        .split('\n')
        .map(|l| Line::from_str(l).unwrap().compress())
        .collect::<Vec<Line>>();
    let mut cache = HashMap::new();
    let lines_2 = lines
        .iter()
        .map(|l| l.clone().convert_to_part_2().compress())
        .collect::<Vec<Line>>();
    (part_a(&lines, &mut cache), part_b(&lines_2, &mut cache))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Broken,
    Unkown,
    Good,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '?' => Self::Unkown,
            '.' => Self::Good,
            _ => unreachable!("Unknown spring character."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Line {
    springs: Vec<Spring>,
    pattern: Vec<u64>,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (springs, pattern) = s.split_once(' ').unwrap();
        Ok(Line::new(
            springs.chars().map(Spring::from).collect(),
            pattern.split(',').map(|c| c.parse().unwrap()).collect(),
        ))
    }
}

fn recursive_cached<'a>(
    cache: &mut HashMap<State<'a>, u64>,
    cur_spring: Spring,
    rem_springs: &'a [Spring],
    rem_pattern: &'a [u64],
    cur_group_size: u64,
) -> u64 {
    let state = (cur_spring, rem_springs, rem_pattern, cur_group_size);
    if let Some(x) = cache.get(&state) {
        return *x;
    }

    let return_value = if cur_spring == Spring::Unkown {
        let x = recursive_cached(
            cache,
            Spring::Good,
            rem_springs,
            rem_pattern,
            cur_group_size,
        );
        let y = recursive_cached(
            cache,
            Spring::Broken,
            rem_springs,
            rem_pattern,
            cur_group_size,
        );
        x + y
    } else if rem_springs.is_empty() {
        match slice_is_valid(&[cur_spring], rem_pattern, cur_group_size) {
            Some(true) => 1,
            Some(false) => 0,
            None => unreachable!(),
        }
    } else {
        match cur_spring {
            Spring::Broken => {
                if rem_pattern.is_empty() || Some(&cur_group_size) > rem_pattern.first() {
                    0
                } else {
                    recursive_cached(
                        cache,
                        rem_springs[0],
                        rem_springs.slice(1..),
                        rem_pattern,
                        cur_group_size + 1,
                    )
                }
            }
            Spring::Good => {
                if cur_group_size == 0 {
                    recursive_cached(
                        cache,
                        rem_springs[0],
                        rem_springs.slice(1..),
                        rem_pattern,
                        cur_group_size,
                    )
                } else if Some(&cur_group_size) == rem_pattern.first() {
                    recursive_cached(
                        cache,
                        rem_springs[0],
                        rem_springs.slice(1..),
                        rem_pattern.slice(1..),
                        0,
                    )
                } else {
                    // It is not valid
                    0
                }
            }
            Spring::Unkown => unreachable!(),
        }
    };

    cache.insert(state, return_value);
    return_value
}

fn slice_is_valid(springs: &[Spring], pattern: &[u64], cur_group_size: u64) -> Option<bool> {
    let mut pattern_idx = 0;
    let mut cur_broken_streak = cur_group_size;

    let min_length_required = pattern.iter().fold(0, |sum, p| (sum + 1 + p));
    let remaining_locations = cur_broken_streak + springs.len() as u64 + 1;
    if min_length_required > remaining_locations {
        return Some(false);
    }

    for spring in springs {
        match spring {
            Spring::Broken => {
                cur_broken_streak += 1;
                if pattern.len() == pattern_idx || cur_broken_streak > pattern[pattern_idx] {
                    return Some(false);
                }
            }
            Spring::Unkown => return None,
            Spring::Good => match cur_broken_streak {
                0 => continue,
                x if x == pattern[pattern_idx] => {
                    cur_broken_streak = 0;
                    pattern_idx += 1;
                }
                _ => return Some(false),
            },
        }
    }
    if pattern.len() == pattern_idx {
        return Some(cur_broken_streak == 0);
    }
    if cur_broken_streak == pattern[pattern_idx] {
        pattern_idx += 1;
    }
    Some(pattern_idx == pattern.len())
}

impl Line {
    fn new(springs: Vec<Spring>, pattern: Vec<u64>) -> Self {
        Line { springs, pattern }
    }

    fn convert_to_part_2(mut self) -> Self {
        self.springs.push(Spring::Unkown);
        self.springs = self.springs.repeat(5);
        self.springs.pop();
        self.pattern = self.pattern.repeat(5);
        self
    }

    fn compress(mut self) -> Self {
        self.springs
            .dedup_by(|a, b| a == &Spring::Good && b == &Spring::Good);
        self
    }

    fn num_arrangements_cache<'a>(&'a self, cache: &mut HashMap<State<'a>, u64>) -> u64 {
        // let mut cache = HashMap::new();
        recursive_cached(cache, self.springs[0], &self.springs[1..], &self.pattern, 0)
    }
}

fn part_a<'a>(lines: &'a Vec<Line>, cache: &mut HashMap<State<'a>, u64>) -> u64 {
    let mut sum = 0;
    for line in lines {
        sum += line.num_arrangements_cache(cache);
    }
    sum
}

fn part_b<'a>(lines: &'a [Line], cache: &mut HashMap<State<'a>, u64>) -> u64 {
    let mut sum = 0;
    for line in lines.iter().tqdm() {
        sum += line.num_arrangements_cache(cache);
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
    }

    #[rstest]
    #[case("???.### 1,1,3", Line::new(vec![
        Spring::Unkown,
        Spring::Unkown,
        Spring::Unkown,
        Spring::Good,
        Spring::Broken,
        Spring::Broken,
        Spring::Broken,
    ], vec![1, 1, 3] ))]
    fn test_parse_line(#[case] line: &str, #[case] expected: Line) {
        assert_eq!(Line::from_str(line).unwrap(), expected);
    }

    #[rstest]
    #[case("##?.### 1,1,3", 0)]
    #[case("#.?.### 1,1,3", 1)]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[case("####???????? 3,2,1", 0)]
    #[case(".###???????? 3,2,1", 10)]
    #[case(".###.??????? 3,2,1", 10)]
    #[case(".####??????? 3,2,1", 0)]
    #[case(".###........ 3,2,1", 0)]
    #[case("???? 1", 4)]
    fn test_num_possible_arrangements2(#[case] line: Line, #[case] num_arrangements: u64) {
        let mut cache = HashMap::new();
        let num_arr = line.clone().num_arrangements_cache(&mut cache);
        assert_eq!(num_arr, num_arrangements);
    }

    #[rstest]
    #[case(".##.#...###?????? 2,1,3,1")]
    #[case("##?.### 1,1,3")]
    #[case("???.### 1,1,3")]
    #[case(".??..??...?##. 1,1,3")]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6")]
    #[case("????.#...#... 4,1,1")]
    #[case("????.######..#####. 1,6,5")]
    #[case(".###........ 3,2,1")]
    fn test_compression(#[case] line: Line) {
        let compress = line.clone().compress();
        let mut cache = HashMap::new();
        assert_eq!(
            line.convert_to_part_2().num_arrangements_cache(&mut cache),
            compress
                .clone()
                .convert_to_part_2()
                .compress()
                .num_arrangements_cache(&mut cache)
        )
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        let lines = example_input
            .trim()
            .split('\n')
            .map(|l| Line::from_str(l).unwrap().compress())
            .collect::<Vec<Line>>();
        let mut cache = HashMap::new();
        assert_eq!(part_a(&lines, &mut cache), 21);
    }

    #[rstest]
    #[case(".# 1", ".#?.#?.#?.#?.# 1,1,1,1,1")]
    #[case(
        "???.### 1,1,3",
        "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
    )]
    fn test_to_part_2(#[case] part_1: Line, #[case] part_2: Line) {
        assert_eq!(part_1.convert_to_part_2(), part_2)
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        let lines = example_input
            .trim()
            .split('\n')
            .map(|l| Line::from_str(l).unwrap().convert_to_part_2().compress())
            .collect::<Vec<Line>>();
        let mut cache = HashMap::new();
        assert_eq!(part_b(&lines, &mut cache), 525152)
    }
}
