use std::{
    cmp::{max, min},
    collections::HashSet,
};

use itertools::Itertools;

pub fn solve(input: &str) -> (u16, u16) {
    let rocks = parse_rocks(input);
    solve_combo(rocks)
}

fn solve_combo(mut rocks: HashSet<(u16, u16)>) -> (u16, u16) {
    let mut hit_floor = false;
    let initial_spawn = (500, 0);
    let floor = rocks.iter().fold(0, |max_, (_, y)| max(max_, *y)) + 2;
    let mut counter = 0;
    let mut part_1 = 0;
    loop {
        let mut sand_loc = initial_spawn;
        loop {
            if sand_loc.1 == floor - 1 {
                rocks.insert(sand_loc);
                hit_floor = true;
                break;
            } else if !rocks.contains(&(sand_loc.0, sand_loc.1 + 1)) {
                sand_loc = (sand_loc.0, sand_loc.1 + 1)
            } else if !rocks.contains(&(sand_loc.0 - 1, sand_loc.1 + 1)) {
                sand_loc = (sand_loc.0 - 1, sand_loc.1 + 1)
            } else if !rocks.contains(&(sand_loc.0 + 1, sand_loc.1 + 1)) {
                sand_loc = (sand_loc.0 + 1, sand_loc.1 + 1)
            } else {
                rocks.insert(sand_loc);
                break;
            }
        }
        counter += 1;
        if hit_floor && part_1 == 0 {
            part_1 = counter - 1;
        }
        if sand_loc == initial_spawn {
            break;
        }
    }
    (part_1, counter)
}

fn parse_rocks(input_str: &str) -> HashSet<(u16, u16)> {
    input_str
        .split('\n')
        .map(parse_rock)
        .reduce(|mut l, r| {
            l.extend(r);
            l
        })
        .unwrap()
}

fn parse_rock(input_str: &str) -> HashSet<(u16, u16)> {
    input_str
        .split(" -> ")
        .map(|rock_str| {
            let (x, y) = rock_str.split_once(',').unwrap();
            (
                x.parse::<u16>().expect(&format!("{x} should be parsable")),
                y.parse::<u16>().expect(&format!("{y} should be parsable")),
            )
        })
        .tuple_windows()
        .flat_map(|((l_x, l_y), (r_x, r_y))| {
            if l_x != r_x {
                (min(l_x, r_x)..=max(l_x, r_x))
                    .map(|x: u16| (x, r_y))
                    .collect::<HashSet<_>>()
            } else {
                (min(l_y, r_y)..=max(l_y, r_y))
                    .map(|y: u16| (r_x, y))
                    .collect::<HashSet<_>>()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("498,4 -> 498,6 -> 496,6", HashSet::from([
        (498, 4),
        (498, 5),
        (498, 6),
        (497, 6),
        (496, 6),

    ]))]
    #[case("503,4 -> 502,4 -> 502,9 -> 494,9", HashSet::from([
        (503,4),
        (502,4),
        (502,5),
        (502,6),
        (502,7),
        (502,8),
        (502,9),
        (501,9),
        (500,9),
        (499,9),
        (498,9),
        (497,9),
        (496,9),
        (495,9),
        (494,9),
    ]))]
    #[case("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9", HashSet::from([
        (503,4),
        (502,4),
        (502,5),
        (502,6),
        (502,7),
        (502,8),
        (502,9),
        (501,9),
        (500,9),
        (499,9),
        (498,9),
        (497,9),
        (496,9),
        (495,9),
        (494,9),
        (498, 4),
        (498, 5),
        (498, 6),
        (497, 6),
        (496, 6),
    ]))]
    fn test_parse_rocks(#[case] input_str: &str, #[case] rock_places: HashSet<(u16, u16)>) {
        assert_eq!(parse_rocks(input_str), rock_places)
    }

    #[rstest]
    fn test_solve_combo() {
        let input_str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let rocks = parse_rocks(input_str);
        assert_eq!(solve_combo(rocks.clone()), (24, 93));
    }
}
