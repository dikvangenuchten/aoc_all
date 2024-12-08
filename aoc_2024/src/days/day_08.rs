use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let a = part_a(input_file);
    let b = part_b(input_file);
    (a, b)
}

fn part_a(input_file: &str) -> u32 {
    let (all_antennas, map_size) = parse_input(input_file);

    let mut antinodes = HashSet::<Point>::new();
    for antennas in all_antennas.values() {
        for a1 in antennas {
            for a2 in antennas {
                if a1 != a2 {
                    let anti = find_antinodes_a(a1, a2, map_size);
                    antinodes.extend(anti);
                }
            }
        }
    }
    antinodes.len() as u32
}

fn part_b(input_file: &str) -> u32 {
    let (all_antennas, map_size) = parse_input(input_file);

    let mut antinodes = HashSet::<Point>::new();
    for antennas in all_antennas.values() {
        for a1 in antennas {
            for a2 in antennas {
                if a1 != a2 {
                    let anti = find_antinodes_b(a1, a2, map_size);
                    antinodes.extend(anti);
                }
            }
        }
    }
    antinodes.len() as u32
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn is_divisible_by(&self, n: isize) -> bool {
        self.x.rem_euclid(n) == 0 && self.y.rem_euclid(n) == 0
    }

    fn is_on_map(&self, x: isize, y: isize) -> bool {
        self.x >= 0 && self.y >= 0 && self.x <= x && self.y <= y
    }

    fn smallest_vector(self) -> Self {
        let divider = gcd(self.x.unsigned_abs(), self.y.unsigned_abs()) as isize;
        Point {
            x: self.x / divider,
            y: self.y / divider,
        }
    }
}

fn gcd(x: usize, y: usize) -> usize {
    let (mut x, mut y) = (x.max(y), x.min(y));
    if y == 0 {
        return x;
    }
    loop {
        let remainder = x % y;
        if remainder == 0 {
            return y;
        }
        x = y;
        y = remainder;
    }
}

fn parse_input(input_file: &str) -> (HashMap<char, Vec<Point>>, (isize, isize)) {
    let mut max_x = 0;
    let mut max_y = 0;
    (
        input_file
            .trim()
            .split("\n")
            .enumerate()
            .flat_map(|(y, row)| {
                max_y = max_y.max(y as isize);
                row.char_indices()
                    .filter_map(|(x, char)| {
                        max_x = max_x.max(x as isize);
                        match char {
                            '.' => None,
                            _ => Some((
                                char,
                                Point {
                                    x: x as isize,
                                    y: y as isize,
                                },
                            )),
                        }
                    })
                    .collect::<Vec<(char, Point)>>()
            })
            .fold(HashMap::new(), |mut acc, (char, point)| {
                acc.entry(char).or_default().push(point);
                acc
            }),
        (max_x, max_y),
    )
}

fn find_antinodes_a(antenna_1: &Point, antenna_2: &Point, map_size: (isize, isize)) -> Vec<Point> {
    // There are a maximum of 4 antinodes
    // Let the vector between antenna_1 and antenna_2 be x
    // Then
    // a1 - 2x  (if on map)
    // a1 + x/3 (if integer)
    // a2 - x/3 (if integer)
    // a2 + 2x  (if on map)
    let vector = Point {
        x: antenna_1.x - antenna_2.x,
        y: antenna_1.y - antenna_2.y,
    };
    let mut antinodes = Vec::with_capacity(4);
    if vector.is_divisible_by(3) {
        antinodes.push(Point {
            x: antenna_1.x - vector.x / 3,
            y: antenna_1.y - vector.y / 3,
        });
        antinodes.push(Point {
            x: antenna_2.x + vector.x / 3,
            y: antenna_2.y + vector.y / 3,
        });
    }

    let antinode_1 = Point {
        x: antenna_1.x - 2 * vector.x,
        y: antenna_1.y - 2 * vector.y,
    };
    if antinode_1.is_on_map(map_size.0, map_size.1) {
        antinodes.push(antinode_1);
    };

    let antinode_4 = Point {
        x: antenna_2.x + 2 * vector.x,
        y: antenna_2.y + 2 * vector.y,
    };
    if antinode_4.is_on_map(map_size.0, map_size.1) {
        antinodes.push(antinode_4);
    };
    antinodes
}

fn find_antinodes_b(antenna_1: &Point, antenna_2: &Point, map_size: (isize, isize)) -> Vec<Point> {
    let mut antinodes = vec![*antenna_1];

    let vector = Point {
        x: antenna_1.x - antenna_2.x,
        y: antenna_1.y - antenna_2.y,
    }
    .smallest_vector();

    let mut multiplier = 1;
    loop {
        let antinode = Point {
            x: antenna_1.x - multiplier * vector.x,
            y: antenna_1.y - multiplier * vector.y,
        };
        if !antinode.is_on_map(map_size.0, map_size.1) {
            break;
        }

        antinodes.push(antinode);
        multiplier += 1;
    }
    let mut multiplier = 1;
    loop {
        let antinode = Point {
            x: antenna_1.x + multiplier * vector.x,
            y: antenna_1.y + multiplier * vector.y,
        };
        if !antinode.is_on_map(map_size.0, map_size.1) {
            break;
        }

        antinodes.push(antinode);
        multiplier += 1;
    }
    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let (antennas, (x, y)) = parse_input(input);
        assert_eq!(x, 11);
        assert_eq!(y, 11);
        assert_eq!(
            antennas,
            HashMap::from([
                (
                    '0',
                    vec![
                        Point { x: 8, y: 1 },
                        Point { x: 5, y: 2 },
                        Point { x: 7, y: 3 },
                        Point { x: 4, y: 4 },
                    ]
                ),
                (
                    'A',
                    vec![
                        Point { x: 6, y: 5 },
                        Point { x: 8, y: 8 },
                        Point { x: 9, y: 9 },
                    ]
                )
            ])
        )
    }

    #[rstest]
    #[case((Point {x: 4, y: 3}, Point {x: 5, y: 5}), vec![Point {x: 3, y: 1}, Point {x: 6, y: 7}])]
    #[case((Point {x: 5, y: 0}, Point {x: 8, y: 0}), vec![Point {x: 2, y: 0}, Point {x: 6, y: 0}, Point {x: 7, y: 0}, Point {x: 11, y: 0}])]
    fn test_find_antinodes_a(#[case] antennas: (Point, Point), #[case] mut anti_nodes: Vec<Point>) {
        let mut found = find_antinodes_a(&antennas.0, &antennas.1, (11, 11));
        found.sort();
        anti_nodes.sort();
        assert_eq!(found, anti_nodes);
        let mut found_other_order = find_antinodes_a(&antennas.1, &antennas.0, (11, 11));
        found_other_order.sort();
        assert_eq!(found, found_other_order);
    }

    #[rstest]
    #[case((Point {x: 4, y: 3}, Point {x: 5, y: 5}), vec![
        Point {x: 3, y: 1}, Point {x: 4, y: 3}, Point {x: 5, y: 5},
        Point {x: 6, y: 7}, Point {x: 7, y: 9}, Point {x: 8, y: 11}
    ])]
    #[case((Point {x: 0, y: 0}, Point {x: 3, y: 0}), vec![
        Point {x: 0, y: 0}, Point {x: 1, y: 0}, Point {x: 2, y: 0},
        Point {x: 3, y: 0}, Point {x: 4, y: 0}, Point {x: 5, y: 0},
        Point {x: 6, y: 0}, Point {x: 7, y: 0}, Point {x: 8, y: 0},
        Point {x: 9, y: 0}, Point {x: 10, y: 0}, Point {x: 11, y: 0},
    ])]
    fn test_find_antinodes_b(#[case] antennas: (Point, Point), #[case] mut anti_nodes: Vec<Point>) {
        let mut found = find_antinodes_b(&antennas.0, &antennas.1, (11, 11));
        found.sort();
        anti_nodes.sort();
        assert_eq!(found, anti_nodes);
        let mut found_other_order = find_antinodes_b(&antennas.1, &antennas.0, (11, 11));
        found_other_order.sort();
        assert_eq!(found, found_other_order);
    }

    #[rstest]
    fn test_part_a() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part_a(input), 14)
    }

    #[rstest]
    #[case(1, 1, 1)]
    #[case(1071, 462, 21)]
    #[case(9, 6, 3)]
    #[case(2 * 2 * 3, 2 * 2 * 7, 4)]
    #[case(10, 0, 10)]
    fn test_gcd(#[case] a: usize, #[case] b: usize, #[case] div: usize) {
        assert_eq!(gcd(a, b), div);
        assert_eq!(gcd(b, a), div);
    }

    #[rstest]
    fn test_part_b() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part_b(input), 34)
    }
}
