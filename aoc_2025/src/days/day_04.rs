use anyhow::Result;
use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let mut map = parse(input_file);
    let a = part_a(&map);
    let b = part_b(&mut map);
    (a, b)
}

pub fn part_a(map: &Map) -> u64 {
    let mut count = 0;
    for y in 0..map.is_paper.len() {
        for x in 0..map.is_paper[y].len() {
            if map.check_paper_reachable(x, y) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_b(map: &mut Map) -> u64 {
    let mut count = 0;
    let mut any_removed = true;
    while any_removed {
        let mut to_be_removed = vec![];
        for y in 0..map.is_paper.len() {
            for x in 0..map.is_paper[y].len() {
                if map.check_paper_reachable(x, y) {
                    count += 1;
                    to_be_removed.push((x, y));
                }
            }
        }
        any_removed = !to_be_removed.is_empty();
        for (x, y) in to_be_removed {
            map.remove(x, y);
        }
    }
    count
}

fn parse(input_file: &str) -> Map {
    Map::from_str(input_file).unwrap()
}

#[derive(Debug, PartialEq)]
pub struct Map {
    is_paper: Vec<Vec<bool>>,
}

impl Map {
    fn check_paper_reachable(&self, x: usize, y: usize) -> bool {
        let mut count = 0;
        let tile = self.get(x, y);
        if tile.is_none() {
            return false;
        }
        if let Some(false) = tile {
            return false;
        }
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                let check_x = x.checked_add_signed(x_offset);
                let check_y = y.checked_add_signed(y_offset);
                if check_x.is_none() || check_y.is_none() {
                    continue;
                }
                let check_x = check_x.unwrap();
                let check_y = check_y.unwrap();
                let tile = self.get(check_x, check_y);
                if tile.is_none() {
                    continue;
                }
                if tile.unwrap() {
                    count += 1;
                }
                if count > 4 {
                    return false;
                }
            }
        }
        true
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if y >= self.is_paper.len() || x >= self.is_paper[y].len() {
            return None;
        }
        Some(self.is_paper[y][x])
    }

    fn remove(&mut self, x: usize, y: usize) {
        if y >= self.is_paper.len() || x >= self.is_paper[y].len() {
            return;
        }
        self.is_paper[y][x] = false;
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_paper = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '@' => Ok(true),
                        '.' => Ok(false),
                        _ => Err(()),
                    })
                    .collect::<Result<Vec<bool>, ()>>()
            })
            .collect::<Result<Vec<Vec<bool>>, ()>>()?;
        Ok(Map { is_paper })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::{read_test_day_input};

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse_test_input() {
        let input_file = read_test_day_input("04");
        let map = parse(&input_file);
        assert_eq!(
            map,
            Map {
                is_paper: vec![
                    vec![
                        false, false, true, true, false, true, true, true, true, false
                    ],
                    vec![
                        true, true, true, false, true, false, true, false, true, true
                    ],
                    vec![true, true, true, true, true, false, true, false, true, true],
                    vec![
                        true, false, true, true, true, true, false, false, true, false
                    ],
                    vec![true, true, false, true, true, true, true, false, true, true],
                    vec![false, true, true, true, true, true, true, true, false, true],
                    vec![
                        false, true, false, true, false, true, false, true, true, true
                    ],
                    vec![true, false, true, true, true, false, true, true, true, true],
                    vec![false, true, true, true, true, true, true, true, true, false],
                    vec![
                        true, false, true, false, true, true, true, false, true, false
                    ],
                ]
            }
        )
    }

    #[rstest]
    #[case(0, 0, false)]
    #[case(1, 0, false)]
    #[case(2, 0, true)]
    #[case(3, 0, true)]
    #[case(5, 0, true)]
    #[case(6, 0, true)]
    #[case(0, 1, true)]
    #[case(1, 1, false)]
    fn test_check_paper_reachable(#[case] x: usize, #[case] y: usize, #[case] expected: bool) {
        let input_file = read_test_day_input("04");
        let map = parse(&input_file);
        assert_eq!(map.check_paper_reachable(x, y), expected);
    }

    #[rstest]
    fn test_day_04_a() {
        let input_file = read_test_day_input("04");
        let map = parse(&input_file);
        let result = part_a(&map);
        assert_eq!(result, 13);
    }

        #[rstest]
    fn test_day_04_b() {
        let input_file = read_test_day_input("04");
        let mut map = parse(&input_file);
        let result = part_b(&mut map);
        assert_eq!(result, 43);
    }
}
