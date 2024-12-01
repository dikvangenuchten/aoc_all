pub fn solve_day(input: &str) -> (u64, u64) {
    let grid = Grid::from(input);
    (part_a(grid.clone()), part_b(grid))
}

fn part_a(mut grid: Grid) -> u64 {
    let beam = LightBeam {
        dir: Direction::East,
        location: (0, 0),
    };
    grid.shine_light(beam);
    grid.count_energized()
}

fn part_b(grid: Grid) -> u64 {
    let top_beams: u64 = (0..grid.grid[0].len())
        .map(|y| LightBeam {
            dir: Direction::North,
            location: (0, y),
        })
        .map(|beam| {
            let mut g = grid.clone();
            g.shine_light(beam);
            g.count_energized()
        })
        .max()
        .unwrap();
    let bot_beams = (0..grid.grid[0].len())
        .map(|y| LightBeam {
            dir: Direction::South,
            location: (grid.grid.len() - 1, y),
        })
        .map(|beam| {
            let mut g = grid.clone();
            g.shine_light(beam);
            g.count_energized()
        })
        .max()
        .unwrap();
    let left_beams = (0..grid.grid.len())
        .map(|x| LightBeam {
            dir: Direction::East,
            location: (x, 0),
        })
        .map(|beam| {
            let mut g = grid.clone();
            g.shine_light(beam);
            g.count_energized()
        })
        .max()
        .unwrap();
    let right_beams = (0..grid.grid.len())
        .map(|x| LightBeam {
            dir: Direction::West,
            location: (x, grid.grid[0].len() - 1),
        })
        .map(|beam| {
            let mut g = grid.clone();
            g.shine_light(beam);
            g.count_energized()
        })
        .max()
        .unwrap();

    top_beams.max(left_beams).max(bot_beams).max(right_beams)
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Empty,
    Left,
    Right,
    Horizontal,
    Vertical,
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        match value {
            '.' => Mirror::Empty,
            '\\' => Mirror::Left,
            '/' => Mirror::Right,
            '-' => Mirror::Horizontal,
            '|' => Mirror::Vertical,
            _ => unreachable!(),
        }
    }
}

impl From<Mirror> for char {
    fn from(value: Mirror) -> Self {
        match value {
            Mirror::Empty => '.',
            Mirror::Left => '\\',
            Mirror::Right => '/',
            Mirror::Horizontal => '-',
            Mirror::Vertical => '|',
        }
    }
}

impl Mirror {
    fn scatter(self, direction: Direction) -> Vec<Direction> {
        match self {
            Mirror::Empty => vec![direction],
            Mirror::Left => match direction {
                Direction::North => vec![Direction::East],
                Direction::East => vec![Direction::North],
                Direction::South => vec![Direction::West],
                Direction::West => vec![Direction::South],
            },
            Mirror::Right => match direction {
                Direction::North => vec![Direction::West],
                Direction::East => vec![Direction::South],
                Direction::South => vec![Direction::East],
                Direction::West => vec![Direction::North],
            },
            Mirror::Horizontal => match direction {
                Direction::North | Direction::South => vec![Direction::East, Direction::West],
                Direction::East | Direction::West => vec![direction],
            },
            Mirror::Vertical => match direction {
                Direction::North | Direction::South => vec![direction],
                Direction::East | Direction::West => vec![Direction::North, Direction::South],
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    mirror: Mirror,
    north_light: bool,
    west_light: bool,
    south_light: bool,
    east_light: bool,
}

impl Tile {
    fn new(mirror: Mirror) -> Self {
        Tile {
            mirror,
            north_light: false,
            west_light: false,
            south_light: false,
            east_light: false,
        }
    }

    fn is_energized(&self) -> u64 {
        if self.north_light | self.west_light | self.south_light | self.east_light {
            return 1;
        }
        0
    }

    fn shine_light(&mut self, beam: LightBeam) -> Vec<LightBeam> {
        match beam.dir {
            Direction::North => {
                if self.north_light {
                    return vec![];
                } else {
                    self.north_light = true;
                }
            }
            Direction::East => {
                if self.east_light {
                    return vec![];
                } else {
                    self.east_light = true;
                }
            }
            Direction::South => {
                if self.south_light {
                    return vec![];
                } else {
                    self.south_light = true;
                }
            }
            Direction::West => {
                if self.west_light {
                    return vec![];
                } else {
                    self.west_light = true;
                }
            }
        }

        self.mirror
            .scatter(beam.dir)
            .iter()
            .map(|d| (d, d.as_offset()))
            .filter_map(|(dir, (x, y))| {
                let location = (
                    (beam.location.0 as isize + x).try_into().ok()?,
                    (beam.location.1 as isize + y).try_into().ok()?,
                );
                Some(LightBeam {
                    dir: *dir,
                    location,
                })
            })
            .collect()
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self::new(Mirror::from(value))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    grid: Vec<Vec<Tile>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (1, 0),
            Direction::East => (0, 1),
            Direction::South => (-1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug)]
struct LightBeam {
    dir: Direction,
    location: (usize, usize),
}

impl Grid {
    fn shine_light(&mut self, start_beam: LightBeam) {
        let mut stack = vec![start_beam];

        while let Some(beam) = stack.pop() {
            let loc = beam.location;
            if let Some(row) = self.grid.get_mut(loc.0) {
                if let Some(tile) = row.get_mut(loc.1) {
                    let beams = tile.shine_light(beam);
                    for b in beams {
                        stack.insert(0, b);
                    }
                }
            }

            if stack.len() > self.grid.len() * self.grid[0].len() {
                panic!()
            }
        }
    }

    fn count_energized(&self) -> u64 {
        let x = self
            .grid
            .iter()
            .map(|r| r.iter().map(|t| t.is_energized()).sum::<u64>())
            .sum();
        x
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let grid = value.trim().split('\n').map(parse_row).collect();
        Grid { grid }
    }
}

fn parse_row(row: &str) -> Vec<Tile> {
    row.chars().map(Tile::from).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
    }

    #[fixture]
    fn example_grid(example_input: &str) -> Grid {
        Grid::from(example_input)
    }

    #[rstest]
    #[case(".\\/-|", vec![Tile::new(Mirror::Empty), Tile::new(Mirror::Left), Tile::new(Mirror::Right), Tile::new(Mirror::Horizontal), Tile::new(Mirror::Vertical)])]
    fn test_parse_row(#[case] row_str: &str, #[case] expected: Vec<Tile>) {
        assert_eq!(parse_row(row_str), expected)
    }

    #[rstest]
    fn test_parse_grid(example_input: &str) {
        let expected = Grid {
            grid: vec![
                parse_row(".|...\\...."),
                parse_row("|.-.\\....."),
                parse_row(".....|-..."),
                parse_row("........|."),
                parse_row(".........."),
                parse_row(".........\\"),
                parse_row("..../.\\\\.."),
                parse_row(".-.-/..|.."),
                parse_row(".|....-|.\\"),
                parse_row("..//.|...."),
            ],
        };
        assert_eq!(Grid::from(example_input), expected)
    }

    #[rstest]
    fn test_energize(mut example_grid: Grid) {
        let beam = LightBeam {
            dir: Direction::East,
            location: (0, 0),
        };
        example_grid.shine_light(beam);

        assert_eq!(example_grid.count_energized(), 46)
    }

    #[rstest]
    fn test_part_a(example_grid: Grid) {
        assert_eq!(part_a(example_grid), 46);
    }

    #[rstest]
    fn test_part_b(example_grid: Grid) {
        assert_eq!(part_b(example_grid), 51);
    }
}
