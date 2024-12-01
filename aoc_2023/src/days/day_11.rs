use itertools::Itertools;

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxy {
    position: Position,
}

impl Galaxy {
    fn distance(&self, other: &Self) -> usize {
        self.position.distance(&other.position)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Galaxies {
    galaxies: Vec<Galaxy>,
}

impl Galaxies {
    fn dimensions(&self) -> Position {
        let dim = self.galaxies.iter().fold((0, 0), |mut dim, galaxy| {
            dim.0 = galaxy.position.x.max(dim.0);
            dim.1 = galaxy.position.y.max(dim.1);
            dim
        });
        Position { x: dim.0, y: dim.1 }
    }

    fn total_distance(&self) -> usize {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a.distance(b))
            .sum()
    }

    fn expand(&mut self, factor: usize) {
        let empty_x = self.find_empty_cols();
        let empty_y = self.find_empty_rows();

        for galaxy in &mut self.galaxies {
            // The err in binary seach is given when the specific item is not found
            // In our case that is expected as a galaxy is by definition never on an
            // empty row (or col).
            let n_empty_x = empty_x.binary_search(&galaxy.position.x).err().unwrap();
            let n_empty_y = empty_y.binary_search(&galaxy.position.y).err().unwrap();
            galaxy.position.x += (factor - 1) * n_empty_x;
            galaxy.position.y += (factor - 1) * n_empty_y;
        }
    }

    fn find_empty_rows(&self) -> Vec<usize> {
        let empty = vec![true; self.dimensions().y + 1];
        self.galaxies
            .iter()
            .fold(empty, |mut empty, galaxy| {
                empty[galaxy.position.y] = false;
                empty
            })
            .iter()
            .enumerate()
            .filter(|(_, is_empty)| **is_empty)
            .map(|(idx, _)| idx)
            .collect()
    }

    fn find_empty_cols(&self) -> Vec<usize> {
        let empty = vec![true; self.dimensions().x + 1];
        self.galaxies
            .iter()
            .fold(empty, |mut empty, galaxy| {
                empty[galaxy.position.x] = false;
                empty
            })
            .iter()
            .enumerate()
            .filter(|(_, is_empty)| **is_empty)
            .map(|(idx, _)| idx)
            .collect()
    }
}

fn parse_input(input: &str) -> Galaxies {
    Galaxies {
        galaxies: input
            .trim()
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| parse_line(line, y))
            .collect(),
    }
}

fn parse_line(line: &str, y: usize) -> Vec<Galaxy> {
    line.chars()
        .enumerate()
        .filter(|(_, char)| char == &'#')
        .map(|(x, _)| Galaxy {
            position: Position { x, y },
        })
        .collect()
}

fn part_a(input: &str) -> u64 {
    calculate(input, 2)
}

fn part_b(input: &str) -> u64 {
    calculate(input, 1_000_000)
}

fn calculate(input: &str, factor: usize) -> u64 {
    let mut galaxies = parse_input(input);
    galaxies.expand(factor);
    galaxies.total_distance().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....\n";
    }

    #[fixture]
    fn parsed_example(example_input: &str) -> Galaxies {
        return parse_input(example_input);
    }

    #[rstest]
    #[case("...#......", 0, vec![Galaxy{ position: Position { x: 3, y: 0}}])]
    #[case(".......#..", 1, vec![Galaxy{ position: Position { x: 7, y: 1}}])]
    #[case("#.........", 2, vec![Galaxy{ position: Position { x: 0, y: 2}}])]
    #[case("..........", 3, vec![])]
    #[case("......#...", 4, vec![Galaxy{ position: Position { x: 6, y: 4}}])]
    #[case(".#........", 5, vec![Galaxy{ position: Position { x: 1, y: 5}}])]
    #[case(".........#", 6, vec![Galaxy{ position: Position { x: 9, y: 6}}])]
    #[case("..........", 7, vec![])]
    #[case(".......#..", 8, vec![Galaxy{ position: Position { x: 7, y: 8}}])]
    #[case("#...#.....", 9, vec![Galaxy{ position: Position { x: 0, y: 9}}, Galaxy{ position: Position { x: 4, y: 9}}])]
    fn test_parse_line(#[case] line: &str, #[case] y_idx: usize, #[case] expected: Vec<Galaxy>) {
        assert_eq!(parse_line(line, y_idx), expected)
    }

    #[rstest]
    fn test_parse_input(parsed_example: Galaxies) {
        assert_eq!(
            parsed_example,
            Galaxies {
                galaxies: vec![
                    Galaxy {
                        position: Position { x: 3, y: 0 }
                    },
                    Galaxy {
                        position: Position { x: 7, y: 1 }
                    },
                    Galaxy {
                        position: Position { x: 0, y: 2 }
                    },
                    Galaxy {
                        position: Position { x: 6, y: 4 }
                    },
                    Galaxy {
                        position: Position { x: 1, y: 5 }
                    },
                    Galaxy {
                        position: Position { x: 9, y: 6 }
                    },
                    Galaxy {
                        position: Position { x: 7, y: 8 }
                    },
                    Galaxy {
                        position: Position { x: 0, y: 9 }
                    },
                    Galaxy {
                        position: Position { x: 4, y: 9 }
                    },
                ]
            }
        );
    }

    #[rstest]
    fn test_find_empty_rows(parsed_example: Galaxies) {
        assert_eq!(parsed_example.find_empty_rows(), vec![3, 7])
    }

    #[rstest]
    fn test_find_empty_cols(parsed_example: Galaxies) {
        assert_eq!(parsed_example.find_empty_cols(), vec![2, 5, 8])
    }

    #[fixture]
    fn example_expanded_galaxy(mut parsed_example: Galaxies) -> Galaxies {
        parsed_example.expand(2);
        return parsed_example;
    }

    #[rstest]
    fn test_expand_galaxy(example_expanded_galaxy: Galaxies) {
        assert_eq!(
            example_expanded_galaxy,
            parse_input("....#........\n.........#...\n#............\n.............\n.............\n........#....\n.#...........\n............#\n.............\n.............\n.........#...\n#....#.......")
        )
    }

    #[rstest]
    #[case(1, 1, 0)]
    #[case(5, 9, 9)]
    #[case(1, 7, 15)]
    #[case(8, 9, 5)]
    fn test_distance(
        example_expanded_galaxy: Galaxies,
        #[case] idx_1: usize,
        #[case] idx_2: usize,
        #[case] dist: usize,
    ) {
        let g1 = example_expanded_galaxy.galaxies.get(idx_1 - 1).unwrap();
        let g2 = example_expanded_galaxy.galaxies.get(idx_2 - 1).unwrap();

        assert_eq!(g1.distance(g2), dist);
    }

    #[rstest]
    fn total_distance(example_expanded_galaxy: Galaxies) {
        assert_eq!(example_expanded_galaxy.total_distance(), 374)
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        assert_eq!(part_a(example_input), 374)
    }

    #[rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn test_expand_distance(
        mut parsed_example: Galaxies,
        #[case] factor: usize,
        #[case] total_distance: usize,
    ) {
        parsed_example.expand(factor);
        assert_eq!(parsed_example.total_distance(), total_distance)
    }

    #[rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    #[case(1_000_000, 82000210)]
    fn test_calculate(example_input: &str, #[case] factor: usize, #[case] total_distance: u64) {
        assert_eq!(calculate(example_input, factor), total_distance)
    }
}
