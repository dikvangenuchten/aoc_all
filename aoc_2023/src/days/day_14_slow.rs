use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::FromStr,
};

use itertools::Itertools;
use tqdm::Iter;

pub fn solve_day(input: &str) -> (u64, u64) {
    let mut plane = Plane::from_str(input).unwrap();
    (part_a(&plane), part_b(&mut plane))
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Location {
    x: usize,
    y: usize,
}

impl From<[usize; 2]> for Location {
    fn from(value: [usize; 2]) -> Self {
        Location {
            x: value[0],
            y: value[1],
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Rock {
    Round(Location),
    Square(Location),
}

impl Rock {
    fn loc(&self) -> &Location {
        match self {
            Rock::Round(loc) => loc,
            Rock::Square(loc) => loc,
        }
    }

    fn loc_mut(&mut self) -> &mut Location {
        match self {
            Rock::Round(loc) => loc,
            Rock::Square(loc) => loc,
        }
    }
}

impl From<(char, usize, usize)> for Rock {
    fn from(value: (char, usize, usize)) -> Self {
        match value.0 {
            '#' => Rock::Square([value.1, value.2].into()),
            'O' => Rock::Round([value.1, value.2].into()),
            '.' => panic!(". should not be passed to Rock::from"),
            _ => panic!("Unknown character {}", value.0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Plane {
    rocks: Vec<Rock>,
    height: usize,
    width: usize,
}

impl Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(rock) = self
                    .rocks
                    .iter()
                    .filter(|r| r.loc() == &mut Location { x, y })
                    .next()
                {
                    match rock {
                        Rock::Round(_) => f.write_char('O')?,
                        Rock::Square(_) => f.write_char('#')?,
                    };
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Plane {
    fn new(rocks: Vec<Rock>, height: usize, width: usize) -> Self {
        Self {
            rocks,
            height,
            width,
        }
    }

    fn sort(&mut self) {
        self.rocks.sort();
    }

    fn count_north_weight(&self) -> u64 {
        (0..self.width).map(|idx| self.col_weight(idx)).sum()
    }

    fn get_col_static_ys(&self, col_idx: usize) -> Vec<usize> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if let Rock::Square(loc) = r {
                    if loc.x == col_idx {
                        return Some(loc.y);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_row_static_xs(&self, row_idx: usize) -> Vec<usize> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if let Rock::Square(loc) = r {
                    if loc.y == row_idx {
                        return Some(loc.x);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_col_dyn_ys(&self, col_idx: usize) -> Vec<usize> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if let Rock::Round(loc) = r {
                    if loc.x == col_idx {
                        return Some(loc.y);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_col_static(&self, col_idx: usize) -> Vec<&Rock> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if let Rock::Square(loc) = r {
                    if loc.x == col_idx {
                        return Some(r);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_col_dyn(&mut self, col_idx: usize) -> Vec<&mut Rock> {
        self.rocks
            .iter_mut()
            .filter_map(|r| {
                if let Rock::Round(loc) = r {
                    if loc.x == col_idx {
                        return Some(r);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_row_static(&self, row_idx: usize) -> Vec<&Rock> {
        self.rocks
            .iter()
            .filter_map(|r| {
                if let Rock::Square(loc) = r {
                    if loc.x == row_idx {
                        return Some(r);
                    }
                }
                return None;
            })
            .collect()
    }

    fn get_row_dyn(&mut self, row_idx: usize) -> Vec<&mut Rock> {
        self.rocks
            .iter_mut()
            .filter_map(|r| {
                if let Rock::Round(loc) = r {
                    if loc.y == row_idx {
                        return Some(r);
                    }
                }
                return None;
            })
            .collect()
    }

    fn col_weight(&self, col_idx: usize) -> u64 {
        let mut squares = self.get_col_static_ys(col_idx);
        let rounds = self.get_col_dyn_ys(col_idx);
        squares.sort();
        let (_, _, tot_weight) = rounds
            .iter()
            // The round rocks cannot be ontop of square rocks
            .map(|y| (&squares).binary_search(y).err().unwrap())
            .fold(
                (self.height, 0, 0),
                |(mut h, prev_idx, mut tot_weight), idx| {
                    if prev_idx == idx {
                        tot_weight += h;
                        h -= 1;
                    } else {
                        h = self.height - &squares[idx - 1] - 1;
                        tot_weight += h;
                        h -= 1;
                    }
                    (h, idx, tot_weight)
                },
            );

        tot_weight as u64
    }

    fn count_weight(&self) -> u64 {
        let height = self.height;
        self.rocks
            .iter()
            .filter_map(|r| match r {
                Rock::Round(loc) => Some(loc.y),
                Rock::Square(_) => None,
            })
            .map(|y| (height - y) as u64)
            .sum()
    }

    fn cycle_north(&mut self) {
        for col_idx in 0..self.width {
            self._cycle_north_col(col_idx)
        }
    }

    fn _cycle_north_col(&mut self, col_idx: usize) {
        let mut blocked = self.get_col_static_ys(col_idx);
        let rounds = self.get_col_dyn(col_idx);
        blocked.sort();
        for round in rounds {
            let loc = round.loc_mut();
            let idx = match (&blocked).binary_search(&loc.y) {
                // The current location of this rock is overtaken by anoter round rock
                // => We need to buble it up on top of the cur stack
                Ok(mut idx) => {
                    if idx == 0 {
                        idx += 1;
                    }
                    while idx < blocked.len() {
                        if blocked[idx] > blocked[idx - 1] + 1 {
                            break;
                        }
                        idx += 1
                    }
                    idx
                }
                Err(idx) => idx,
            };
            if idx == 0 {
                loc.y = 0;
                blocked.insert(0, loc.y);
            } else if let Some(y) = blocked.get(idx - 1) {
                loc.y = *y + 1;
                blocked.insert(idx, loc.y);
            } else {
                unreachable!();
            }
        }
    }

    fn cycle_west(&mut self) {
        for row_idx in 0..self.height {
            self._cycle_west_row(row_idx)
        }
    }

    fn _cycle_west_row(&mut self, row_idx: usize) {
        let mut blocked = self.get_row_static_xs(row_idx);
        let rounds = self.get_row_dyn(row_idx);
        blocked.sort();
        for round in rounds {
            let loc = round.loc_mut();
            let idx = match (&blocked).binary_search(&loc.x) {
                // The current location of this rock is overtaken by anoter round rock
                // => We need to buble it up on top of the cur stack
                Ok(mut idx) => {
                    if idx == 0 {
                        idx = 1
                    }
                    while idx < blocked.len() {
                        if blocked[idx] > blocked[idx - 1] + 1 {
                            break;
                        }
                        idx += 1
                    }
                    idx
                }
                Err(idx) => idx,
            };
            if idx == 0 {
                loc.x = 0;
                blocked.insert(0, loc.x);
            } else if let Some(x) = blocked.get(idx - 1) {
                loc.x = *x + 1;
                blocked.insert(idx, loc.x);
            } else {
                unreachable!()
            }
        }
    }

    fn cycle_south(&mut self) {
        for col_idx in 0..self.width {
            self._cycle_south_col(col_idx)
        }
    }

    fn _cycle_south_col(&mut self, col_idx: usize) {
        let height = self.height - 1;
        let mut blocked = self.get_col_static_ys(col_idx);
        let rounds = self.get_col_dyn(col_idx);
        blocked.sort();
        for round in rounds {
            let loc = round.loc_mut();
            let idx = match (&blocked).binary_search(&loc.y) {
                // The current location of this rock is overtaken by anoter round rock
                // => We need to buble it up on top of the cur stack
                Ok(mut idx) => {
                    while idx > 0 {
                        if blocked[idx] > blocked[idx - 1] + 1 {
                            break;
                        }
                        idx -= 1
                    }
                    idx
                }
                Err(idx) => idx,
            };
            if let Some(y) = blocked.get(idx) {
                loc.y = *y - 1;
                blocked.insert(idx, loc.y);
            } else {
                loc.y = height;
                blocked.push(loc.y);
            }
        }
    }

    fn cycle_east(&mut self) {
        for row_idx in 0..self.height {
            self._cycle_east_row(row_idx)
        }
    }

    fn _cycle_east_row(&mut self, row_idx: usize) {
        let width = self.width - 1;

        let mut blocked = self.get_row_static_xs(row_idx);
        let rounds = self.get_row_dyn(row_idx);
        blocked.sort();
        for round in rounds {
            let loc = round.loc_mut();

            let idx = match (&blocked).binary_search(&loc.x) {
                // The current location of this rock is overtaken by anoter round rock
                // => We need to buble it up on top of the cur stack
                Ok(mut idx) => {
                    while idx > 0 {
                        if blocked[idx] > blocked[idx - 1] + 1 {
                            break;
                        }
                        idx -= 1
                    }
                    idx
                }
                Err(idx) => idx,
            };
            if let Some(x) = blocked.get(idx) {
                loc.x = *x - 1;
                blocked.insert(idx, loc.x);
            } else {
                loc.x = width;
                blocked.push(loc.x);
            }
        }
    }

    fn run_cycle(&mut self) {
        self.cycle_north();
        self.cycle_west();
        self.cycle_south();
        self.cycle_east();
    }

    fn run_cycles(&mut self, mut n: u64) {
        let mut history = HashMap::with_capacity(100);
        // let mut history_vec = vec![];
        let mut i = 0;
        let before_count = format!("{self}").chars().counts();
        while i < n {
            if let Some(prev) = history.get(self) {
                let cycle_length = i - prev;
                let cycles_left = (n - i) / cycle_length;
                let cycles_todo = (n - i) - cycle_length * cycles_left;
                n = i + cycles_todo;
                println!("Cycle detected!")
            } else {
                history.insert(self.clone(), i);
            }
            self.run_cycle();
            let after = format!("{self}").chars().counts();
            if before_count != after {
                panic!();
            }
            print!("Finished: {i}\r");
            i += 1;
        }
    }
}

fn parse_row(row: &str, y: usize) -> Vec<Rock> {
    row.chars()
        .enumerate()
        .filter(|(_, c)| c == &'#' || c == &'O')
        .map(|(x, c)| Rock::from((c, x, y)))
        .collect()
}

impl FromStr for Plane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split('\n')
                .enumerate()
                .flat_map(|(y, row)| parse_row(row, y))
                .collect(),
            s.trim().split('\n').count(),
            s.split_once('\n')
                .unwrap_or_else(|| (s, ""))
                .0
                .chars()
                .count(),
        ))
    }
}

fn part_a(plane: &Plane) -> u64 {
    plane.count_north_weight()
}

fn part_b(plane: &mut Plane) -> u64 {
    plane.run_cycles(1_000_000_000);
    plane.count_weight()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....\n";
    }

    #[rstest]
    #[case("O....#....", 0, vec![Rock::Round([0, 0].into()), Rock::Square([5, 0].into())])]
    #[case("O.OO#....#", 1, vec![Rock::Round([0, 1].into()), Rock::Round([2, 1].into()), Rock::Round([3, 1].into()), Rock::Square([4, 1].into()), Rock::Square([9, 1].into())])]
    fn test_parse_row(#[case] row_str: &str, #[case] y: usize, #[case] row: Vec<Rock>) {
        assert_eq!(parse_row(row_str, y), row)
    }

    #[rstest]
    #[case(Plane::new(parse_row("O....#....", 0).into_iter()
        .chain(parse_row("O.OO#....#", 1))
        .chain(parse_row(".....##...", 2))
        .chain(parse_row("OO.#O....O", 3))
        .chain(parse_row(".O.....O#.", 4))
        .chain(parse_row("O.#..O.#.#", 5))
        .chain(parse_row("..O..#O..O", 6))
        .chain(parse_row(".......O..", 7))
        .chain(parse_row("#....###..", 8))
        .chain(parse_row("#OO..#....", 9)).collect(),
        10, 10
    ))]
    fn test_parse_plane(example_input: &str, #[case] expected: Plane) {
        assert_eq!(Plane::from_str(example_input).unwrap(), expected)
    }

    #[rstest]
    #[case(0, vec![8, 9])]
    #[case(1, vec![])]
    #[case(2, vec![5])]
    #[case(3, vec![3])]
    #[case(4, vec![1])]
    #[case(5, vec![0, 2, 6, 8, 9])]
    fn test_get_static_ys(
        example_input: &str,
        #[case] col_idx: usize,
        #[case] expected: Vec<usize>,
    ) {
        let plane = Plane::from_str(example_input).unwrap();
        assert_eq!(plane.get_col_static_ys(col_idx), expected)
    }

    #[rstest]
    #[case(0, vec![0, 1, 3, 5])]
    fn test_get_dyn_ys(example_input: &str, #[case] col_idx: usize, #[case] expected: Vec<usize>) {
        let plane = Plane::from_str(example_input).unwrap();
        assert_eq!(plane.get_col_dyn_ys(col_idx), expected)
    }

    #[rstest]
    #[case(0, 10 + 9 + 8 + 7)]
    #[case(1, 10 + 9 + 8)]
    #[case(2, 10 + 4 + 3)]
    #[case(3, 10)]
    #[case(4, 8)]
    #[case(5, 7)]
    #[case(6, 7)]
    #[case(7, 10 + 4)]
    #[case(8, 0)]
    #[case(9, 8 + 4)]
    fn test_weight_col(example_input: &str, #[case] col: usize, #[case] weight: u64) {
        let plane = Plane::from_str(example_input).unwrap();
        assert_eq!(plane.col_weight(col), weight)
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        let plane = Plane::from_str(example_input).unwrap();
        assert_eq!(part_a(&plane), 136)
    }

    #[rstest]
    #[case("OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....")]
    fn test_cycle_north(example_input: &str, #[case] mut expected: Plane) {
        let mut plane = Plane::from_str(example_input).unwrap();
        plane.cycle_north();
        // Ensure order does not matter
        plane.sort();
        expected.sort();
        assert_eq!(plane, expected)
    }

    #[rstest]
    #[case(".OO\n#O#\nO.#", "OO.\n#O#\nO.#")]
    #[case("O..\n.O.\n..O", "O..\nO..\nO..")]
    #[case("OOO\nOOO\nOOO", "OOO\nOOO\nOOO")]
    #[case("#O#\nO#O\nOOO", "#O#\nO#O\nOOO")]
    #[case("#O#", "#O#")]
    #[case("OOO", "OOO")]
    #[case("OO#", "OO#")]
    #[case("O.#", "O.#")]
    #[case("O.O#", "OO.#")]
    #[case("OOOO.#.O..", "OOOO.#O...")]
    fn test_cycle_west(#[case] mut pre_cycle: Plane, #[case] mut expected: Plane) {
        println!("pre_cycle:\n{pre_cycle}");
        pre_cycle.cycle_west();
        // Ensure order does not matter
        println!("post_cycle:\n{pre_cycle}");
        println!("expected:\n{expected}");
        pre_cycle.sort();
        expected.sort();
        assert_eq!(pre_cycle, expected)
    }

    #[rstest]
    #[case("O..\n.O.\n..O", "..O\n..O\n..O")]
    #[case("OO.\n.O.\nO.O", ".OO\n..O\n.OO")]
    #[case("OOO\nOOO\nOOO", "OOO\nOOO\nOOO")]
    #[case("#O#\nO#O\nOOO", "#O#\nO#O\nOOO")]
    #[case("#O#", "#O#")]
    #[case("OOO", "OOO")]
    #[case("OO#", "OO#")]
    #[case("O..#", "..O#")]
    #[case("O.....OO..", ".......OOO")]
    #[case("#O..O###..", "#..OO###..")]
    #[case("#O..O#O.O.", "#..OO#..OO")]
    // #[case(".....#....\n....#.O..#\n.....##...\n..O#......\nO.O....O#.\nO.#..O.#.#\nO....#O...\nO.....OO..\n#O..O###..\n#O.OO#..O.\n", "")]
    fn test_cycle_east(#[case] mut pre_cycle: Plane, #[case] mut expected: Plane) {
        println!("pre_cycle:\n{pre_cycle}");
        pre_cycle.cycle_east();
        // Ensure order does not matter
        println!("post_cycle:\n{pre_cycle}");
        println!("expected:\n{expected}");
        pre_cycle.sort();
        expected.sort();
        assert_eq!(pre_cycle, expected)
    }

    #[rstest]
    #[case("O..\n.O.\n..O", "...\n...\nOOO")]
    #[case("OO.\n.O.\nO.O", "...\nOO.\nOOO")]
    #[case("OOO\nOOO\nOOO", "OOO\nOOO\nOOO")]
    #[case("#O#\nO#O\nOOO", "#O#\nO#O\nOOO")]
    #[case("#\nO\nO", "#\nO\nO")]
    #[case("#\nO\n#", "#\nO\n#")]
    fn test_cycle_south(#[case] mut pre_cycle: Plane, #[case] mut expected: Plane) {
        println!("pre_cycle:\n{pre_cycle}");
        pre_cycle.cycle_south();
        // Ensure order does not matter
        println!("post_cycle:\n{pre_cycle}");
        println!("expected:\n{expected}");
        pre_cycle.sort();
        expected.sort();
        assert_eq!(pre_cycle, expected)
    }

    #[rstest]
    #[case("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....")]
    #[case(".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O")]
    #[case(".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O")]
    fn test_cycle(#[case] mut before: Plane, #[case] mut after: Plane) {
        before.run_cycle();
        before.sort();

        println!("Before:\n{before}");
        println!("expected:\n{after}");
        after.sort();
        assert_eq!(before, after)
    }

    #[rstest]
    #[case("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....", 1)]
    #[case("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O", 2)]
    #[case("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O", 3)]
    fn test_cycles(#[case] mut before: Plane, #[case] mut after: Plane, #[case] n: u64) {
        before.run_cycles(n);
        before.sort();

        println!("Before:\n{before}");
        println!("expected:\n{after}");
        after.sort();
        assert_eq!(before, after)
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        let mut plane = Plane::from_str(example_input).unwrap();
        assert_eq!(part_b(&mut plane), 64);
    }
}
