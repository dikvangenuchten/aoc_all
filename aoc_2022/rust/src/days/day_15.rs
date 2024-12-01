use std::{collections::HashSet, num::ParseIntError, str::FromStr, vec};

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve(input: &str) -> (usize, u64) {
    let sensors = parse_input(input);
    let part_1 = solve_part_1(&sensors, 2000000);
    let part_2 = solve_part_2(&sensors, 4000000);
    (part_1, part_2)
}

fn solve_part_1(sensors: &Vec<Sensor>, row: i64) -> usize {
    sensors
        .iter()
        .map(|s| s.get_excluded_region_at_y(row))
        .reduce(|mut collector, s| {
            collector.extend(s);
            collector
        })
        .unwrap_or_default()
        .len()
}

fn solve_part_2(sensors: &Vec<Sensor>, max_row: i64) -> u64 {
    let beacon = find_possible_loc(sensors, max_row);
    (beacon.0 * 4000000 + beacon.1) as u64
}

fn find_possible_loc(sensors: &Vec<Sensor>, max_row: i64) -> (i64, i64) {
    for sensor in sensors {
        for border_loc in sensor.iter_border() {
            if loc_is_possible(border_loc, sensors, max_row) {
                return border_loc;
            }
        }
    }
    unreachable!("There should be a possible location!")
}

fn loc_is_possible(loc: (i64, i64), sensors: &Vec<Sensor>, max: i64) -> bool {
    0 <= loc.0 && loc.0 <= max && 0 <= loc.1 && loc.1 <= max && {
        for sensor in sensors {
            if !sensor.can_contain_unknown_beacon(&loc) {
                return false;
            }
        }
        // println!("{:?} is possible", loc);
        return true;
    }
}

fn manhattan_distance(left: &(i64, i64), right: &(i64, i64)) -> u64 {
    left.0.abs_diff(right.0) + left.1.abs_diff(right.1)
}

fn parse_input(input_str: &str) -> Vec<Sensor> {
    input_str
        .trim()
        .split('\n')
        .map(Sensor::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    loc: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn can_contain_unknown_beacon(&self, loc: &(i64, i64)) -> bool {
        manhattan_distance(&self.loc, &self.beacon) < manhattan_distance(&self.loc, &loc)
    }

    fn get_excluded_region_at_y(&self, row: i64) -> HashSet<i64> {
        let exclusion_range = self.range() as i64;
        let dist_to_row = self.loc.1.abs_diff(row) as i64;
        let diff = exclusion_range - dist_to_row;
        if diff <= 0 {
            return HashSet::new();
        }
        let mut start = -diff + self.loc.0;
        let mut end = diff + self.loc.0;
        // Beacon is always at the edge of a region
        if start == self.beacon.0 {
            start += 1;
        }
        if end == self.beacon.0 {
            end -= 1;
        }
        HashSet::from_iter(start..=end)
    }

    fn range(&self) -> u64 {
        manhattan_distance(&self.loc, &self.beacon)
    }

    fn iter_border(&self) -> impl Iterator<Item = (i64, i64)> {
        let dist = manhattan_distance(&self.loc, &self.beacon) as i64 + 1;
        let mut border_locs = vec![];
        for i in 0..dist {
            border_locs.push((self.loc.0 + i, self.loc.1 + (dist - i)));
            border_locs.push((self.loc.0 - i, self.loc.1 + (dist - i)));
            border_locs.push((self.loc.0 + i, self.loc.1 - (dist - i)));
            border_locs.push((self.loc.0 - i, self.loc.1 - (dist - i)));
        }
        border_locs.into_iter()
    }
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let _own_loc = (
            captures
                .get(1)
                .expect(&format!("Could not parse {s}"))
                .as_str()
                .parse()
                .expect(&format!("Could not parse {s}")),
            captures
                .get(2)
                .expect(&format!("Could not parse {s}"))
                .as_str()
                .parse()
                .expect(&format!("Could not parse {s}")),
        );
        let _beacon_loc = (
            captures
                .get(3)
                .expect(&format!("Could not parse {s}"))
                .as_str()
                .parse()
                .expect(&format!("Could not parse {s}")),
            captures
                .get(4)
                .expect(&format!("Could not parse {s}"))
                .as_str()
                .parse()
                .expect(&format!("Could not parse {s}")),
        );
        Ok(Sensor {
            loc: _own_loc,
            beacon: _beacon_loc,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("Sensor at x=2, y=18: closest beacon is at x=-2, y=15", Sensor { loc: (2, 18), beacon: (-2, 15)})]
    #[case("Sensor at x=9, y=16: closest beacon is at x=10, y=16", Sensor { loc: (9, 16), beacon: (10, 16)})]
    #[case("Sensor at x=13, y=2: closest beacon is at x=15, y=3", Sensor { loc: (13, 2), beacon: (15, 3)})]
    #[case("Sensor at x=12, y=14: closest beacon is at x=10, y=16", Sensor { loc: (12, 14), beacon: (10, 16)})]
    #[case("Sensor at x=10, y=20: closest beacon is at x=10, y=16", Sensor { loc: (10, 20), beacon: (10, 16)})]
    #[case("Sensor at x=14, y=17: closest beacon is at x=10, y=16", Sensor { loc: (14, 17), beacon: (10, 16)})]
    #[case("Sensor at x=8, y=7: closest beacon is at x=2, y=10", Sensor { loc: (8, 7), beacon: (2, 10)})]
    #[case("Sensor at x=2, y=0: closest beacon is at x=2, y=10", Sensor { loc: (2, 0), beacon: (2, 10)})]
    #[case("Sensor at x=0, y=11: closest beacon is at x=2, y=10", Sensor { loc: (0, 11), beacon: (2, 10)})]
    #[case("Sensor at x=20, y=14: closest beacon is at x=25, y=17", Sensor { loc: (20, 14), beacon: (25, 17)})]
    #[case("Sensor at x=17, y=20: closest beacon is at x=21, y=22", Sensor { loc: (17, 20), beacon: (21, 22)})]
    #[case("Sensor at x=16, y=7: closest beacon is at x=15, y=3", Sensor { loc: (16, 7), beacon: (15, 3)})]
    #[case("Sensor at x=14, y=3: closest beacon is at x=15, y=3", Sensor { loc: (14, 3), beacon: (15, 3)})]
    #[case("Sensor at x=20, y=1: closest beacon is at x=15, y=3", Sensor { loc: (20, 1), beacon: (15, 3)})]
    fn test_parse_input(#[case] input_str: &str, #[case] sensor: Sensor) {
        assert_eq!(Sensor::from_str(input_str), Ok(sensor))
    }

    #[fixture]
    fn example_input_str() -> String {
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3".into()
    }

    #[fixture]
    fn example_sensors(example_input_str: String) -> Vec<Sensor> {
        parse_input(&example_input_str)
    }

    #[fixture]
    fn actual_input_str() -> String {
        read_day_input("day_15")
    }

    #[fixture]
    fn actual_sensors(actual_input_str: String) -> Vec<Sensor> {
        parse_input(&actual_input_str)
    }
    #[rstest]
    fn test_solve_part_1(example_sensors: Vec<Sensor>) {
        assert_eq!(solve_part_1(&example_sensors, 10), 26);
    }

    #[rstest]
    #[ignore]
    fn test_solve_part_1_actual(actual_sensors: Vec<Sensor>) {
        assert_eq!(solve_part_1(&actual_sensors, 2000000), 5125700);
    }

    #[rstest]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, 0, 5)]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, -1, 3)]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, 1, 7)]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, 10, 12)] // Beacon should be excluded
    fn test_get_excluded_region_at_y(
        #[case] sensor: Sensor,
        #[case] row: i64,
        #[case] len_free: usize,
    ) {
        assert_eq!(sensor.get_excluded_region_at_y(row).len(), len_free)
    }

    #[rstest]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, 0, HashSet::from([6, 7, 8, 9, 10]))]
    #[case(Sensor { loc: (8, 7), beacon: (2, 10)}, -1, HashSet::from([7, 8, 9]))]
    fn test_get_excluded_region_at_y_set(
        #[case] sensor: Sensor,
        #[case] row: i64,
        #[case] set: HashSet<i64>,
    ) {
        assert_eq!(sensor.get_excluded_region_at_y(row), set)
    }

    #[rstest]
    fn test_solve_part_2(example_sensors: Vec<Sensor>) {
        assert_eq!(solve_part_2(&example_sensors, 20), 56000011);
    }
}
