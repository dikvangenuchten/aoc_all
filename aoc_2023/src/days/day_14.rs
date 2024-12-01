use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::FromStr,
};

pub fn solve_day(input: &str) -> (u64, u64) {
    let grid = Grid::from_str(input).unwrap();
    (part_a(grid.clone()), part_b(grid))
}

fn part_a(mut grid: Grid) -> u64 {
    grid.tilt_north();
    grid.count_weight()
}

fn part_b(mut grid: Grid) -> u64 {
    grid.run_n_cycles(1_000_000_000);
    grid.count_weight()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Round,
    Square,
    Empty,
}

impl From<char> for Tile {
    fn from(s: char) -> Self {
        match s {
            'O' => Tile::Round,
            '#' => Tile::Square,
            '.' => Tile::Empty,
            _ => panic!("Unkown input: {s}"),
        }
    }
}

fn parse_row(input: &str) -> Vec<Tile> {
    input.chars().map(Tile::from).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            tiles: s.trim().split('\n').map(parse_row).collect(),
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Round => f.write_char('O')?,
                    Tile::Square => f.write_char('#')?,
                    Tile::Empty => f.write_char('.')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Grid {
    fn count_weight(&self) -> u64 {
        let mut sum = 0;
        let height = self.tiles.len();
        for col in 0..self.tiles[0].len() {
            for row in 0..self.tiles.len() {
                match self.tiles[row][col] {
                    Tile::Round => sum += height - row,
                    Tile::Square => continue,
                    Tile::Empty => continue,
                };
            }
        }
        sum as u64
    }

    fn tilt_north(&mut self) {
        for col in 0..self.tiles[0].len() {
            let mut blocker = 0;
            for row in 0..self.tiles.len() {
                match self.tiles[row][col] {
                    Tile::Round => {
                        // Move to blocker
                        self.tiles[row][col] = Tile::Empty;
                        self.tiles[blocker][col] = Tile::Round;
                        // New blocker is incremented
                        blocker += 1
                    }
                    Tile::Square => blocker = row + 1,
                    Tile::Empty => continue,
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.tiles.len() {
            let mut blocker = 0;
            for col in 0..self.tiles[0].len() {
                match self.tiles[row][col] {
                    Tile::Round => {
                        // Move to blocker
                        self.tiles[row][col] = Tile::Empty;
                        self.tiles[row][blocker] = Tile::Round;
                        // New blocker is incremented
                        blocker += 1
                    }
                    Tile::Square => blocker = col + 1,
                    Tile::Empty => continue,
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.tiles.len() {
            let mut blocker = self.tiles[0].len() - 1;
            for col in (0..self.tiles[0].len()).rev() {
                match self.tiles[row][col] {
                    Tile::Round => {
                        // Move to blocker
                        self.tiles[row][col] = Tile::Empty;
                        self.tiles[row][blocker] = Tile::Round;
                        // New blocker is incremented
                        blocker = blocker.saturating_sub(1)
                    }
                    Tile::Square => blocker = col.saturating_sub(1),
                    Tile::Empty => continue,
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.tiles[0].len() {
            let mut blocker = self.tiles.len() - 1;
            for row in (0..self.tiles.len()).rev() {
                match self.tiles[row][col] {
                    Tile::Round => {
                        // Move to blocker
                        self.tiles[row][col] = Tile::Empty;
                        self.tiles[blocker][col] = Tile::Round;
                        // New blocker is incremented
                        blocker = blocker.saturating_sub(1)
                    }
                    Tile::Square => blocker = row.saturating_sub(1),
                    Tile::Empty => continue,
                }
            }
        }
    }

    fn run_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn run_n_cycles(&mut self, mut n: u64) {
        let mut history = HashMap::with_capacity(100);
        // let mut history_vec = vec![];
        let mut i = 0;
        while i < n {
            if let Some(prev) = history.get(self) {
                // Todo return the self from history
                let cycle_length = i - prev;
                let cycles_left = (n - i) / cycle_length;
                let cycles_todo = (n - i) - cycle_length * cycles_left;
                n = i + cycles_todo;
            } else {
                history.insert(self.clone(), i);
            }
            self.run_cycle();

            i += 1;
        }
    }
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
    #[case("O.#", vec![Tile::Round, Tile::Empty, Tile::Square])]
    fn test_parse_row(#[case] input: &str, #[case] expected: Vec<Tile>) {
        assert_eq!(parse_row(input), expected)
    }

    #[rstest]
    fn test_parse_example(example_input: &str) {
        let expected = Grid {
            tiles: vec![
                parse_row("O....#...."),
                parse_row("O.OO#....#"),
                parse_row(".....##..."),
                parse_row("OO.#O....O"),
                parse_row(".O.....O#."),
                parse_row("O.#..O.#.#"),
                parse_row("..O..#O..O"),
                parse_row(".......O.."),
                parse_row("#....###.."),
                parse_row("#OO..#...."),
            ],
        };
        assert_eq!(Grid::from_str(example_input).unwrap(), expected)
    }

    #[fixture]
    fn example_grid(example_input: &str) -> Grid {
        Grid::from_str(example_input).unwrap()
    }

    #[rstest]
    fn tilt_north(mut example_grid: Grid) {
        println!("{example_grid}");
        println!("...");
        example_grid.tilt_north();
        println!("{example_grid}");
        assert_eq!(
            example_grid,
            Grid::from_str("OOOO.#.O..\nOO..#....#\nOO..O##..O\nO..#.OO...\n........#.\n..#....#.#\n..O..#.O.O\n..O.......\n#....###..\n#....#....").unwrap()
        )
    }

    #[rstest]
    fn test_count_weight(mut example_grid: Grid) {
        example_grid.tilt_north();
        assert_eq!(example_grid.count_weight(), 136)
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
    fn test_tilt_west(#[case] mut pre_tilt: Grid, #[case] expected: Grid) {
        println!("pre_tilt:\n{pre_tilt}");
        pre_tilt.tilt_west();
        // Ensure order does not matter
        println!("post_tilt:\n{pre_tilt}");
        println!("expected:\n{expected}");

        assert_eq!(pre_tilt, expected)
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
    fn test_tilt_east(#[case] mut pre_cycle: Grid, #[case] expected: Grid) {
        println!("pre_cycle:\n{pre_cycle}");
        pre_cycle.tilt_east();
        // Ensure order does not matter
        println!("post_cycle:\n{pre_cycle}");
        println!("expected:\n{expected}");
        assert_eq!(pre_cycle, expected)
    }

    #[rstest]
    #[case("O..\n.O.\n..O", "...\n...\nOOO")]
    #[case("OO.\n.O.\nO.O", "...\nOO.\nOOO")]
    #[case("OOO\nOOO\nOOO", "OOO\nOOO\nOOO")]
    #[case("#O#\nO#O\nOOO", "#O#\nO#O\nOOO")]
    #[case("#\nO\nO", "#\nO\nO")]
    #[case("#\nO\n#", "#\nO\n#")]
    fn test_tilt_south(#[case] mut pre_tilt: Grid, #[case] expected: Grid) {
        println!("pre_tilt:\n{pre_tilt}");
        pre_tilt.tilt_south();
        // Ensure order does not matter
        println!("post_tilt:\n{pre_tilt}");
        println!("expected:\n{expected}");
        assert_eq!(pre_tilt, expected)
    }

    #[rstest]
    #[case("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", ".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....")]
    #[case(".....#....\n....#...O#\n...OO##...\n.OO#......\n.....OOO#.\n.O#...O#.#\n....O#....\n......OOOO\n#...O###..\n#..OO#....", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O")]
    #[case(".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#..OO###..\n#.OOO#...O", ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O")]
    fn test_cycle(#[case] mut before: Grid, #[case] after: Grid) {
        before.run_cycle();

        println!("Before:\n{before}");
        println!("expected:\n{after}");
        assert_eq!(before, after)
    }

    #[rstest]
    fn test_part_a(example_grid: Grid) {
        assert_eq!(part_a(example_grid), 136);
    }

    #[rstest]
    fn test_part_b(example_grid: Grid) {
        assert_eq!(part_b(example_grid), 64);
    }
}
