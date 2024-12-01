use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

#[derive(Debug, PartialEq, Eq)]
enum MirrorSpot {
    Horizontal(u64),
    Vertical(u64),
}

impl MirrorSpot {
    fn as_u64(&self) -> u64 {
        match self {
            MirrorSpot::Horizontal(x) => 100 * x,
            MirrorSpot::Vertical(x) => *x,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ash => f.write_char('.')?,
            Tile::Rock => f.write_char('#')?,
        }
        Ok(())
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Rock,
            '.' => Tile::Ash,
            _ => panic!("Invalid char :{c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    tiles: Vec<Tile>,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in &self.tiles {
            f.write_str(&format!("{t}"))?
        }
        Ok(())
    }
}

impl Line {
    fn diff(&self, other: &Self) -> u64 {
        let mut diff = 0;
        for (s, o) in self.tiles.iter().zip(other.tiles.iter()) {
            if s != o {
                diff += 1;
                if diff > 1 {
                    return diff;
                }
            }
        }
        diff
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line {
            tiles: s.chars().map(Tile::from).collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Field {
    lines: Vec<Line>,
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Field {
            lines: s.trim().split('\n').map(Line::from_str).try_collect()?,
        })
    }
}

impl Field {
    fn get_row(&self, idx: usize) -> &Line {
        self.lines.get(idx).unwrap()
    }

    fn get_col(&self, idx: usize) -> Line {
        Line {
            tiles: self.lines.iter().map(|l| l.tiles[idx]).collect(),
        }
    }

    fn find_spot(&self) -> MirrorSpot {
        for i in 0..(self.lines.len() - 1) {
            if self.is_horizontal_mirror_spot(i) {
                return MirrorSpot::Horizontal((i + 1) as u64);
            }
        }

        for i in 0..(self.lines[0].tiles.len() - 1) {
            if self.is_vertical_mirror_spot(i) {
                return MirrorSpot::Vertical((i + 1) as u64);
            }
        }
        unreachable!("There should be atleast 1 mirror spot :(.")
    }

    fn is_horizontal_mirror_spot(&self, i: usize) -> bool {
        let mut offset = 0;
        while i >= offset && i + 1 + offset < self.lines.len() {
            if self.get_row(i - offset) != self.get_row(i + 1 + offset) {
                return false;
            }
            offset += 1;
        }
        true
    }

    fn is_vertical_mirror_spot(&self, i: usize) -> bool {
        let mut offset = 0;
        while i >= offset && i + 1 + offset < self.lines[0].tiles.len() {
            if self.get_col(i - offset) != self.get_col(i + 1 + offset) {
                return false;
            }
            offset += 1;
        }
        true
    }

    fn find_spot_smudge(&self) -> MirrorSpot {
        for i in 0..(self.lines.len() - 1) {
            if self.is_horizontal_mirror_spot_smudge(i) {
                return MirrorSpot::Horizontal((i + 1) as u64);
            }
        }

        for i in 0..(self.lines[0].tiles.len() - 1) {
            if self.is_vertical_mirror_spot_smudge(i) {
                return MirrorSpot::Vertical((i + 1) as u64);
            }
        }
        unreachable!("There should be atleast 1 mirror spot :(.")
    }

    fn is_horizontal_mirror_spot_smudge(&self, i: usize) -> bool {
        let mut offset = 0;
        let mut tot_diff = 0;
        while i >= offset && i + 1 + offset < self.lines.len() {
            tot_diff += self.get_row(i - offset).diff(self.get_row(i + 1 + offset));
            if tot_diff > 1 {
                return false;
            }
            offset += 1;
        }
        tot_diff == 1
    }

    fn is_vertical_mirror_spot_smudge(&self, i: usize) -> bool {
        let mut offset = 0;
        let mut tot_diff = 0;
        while i >= offset && i + 1 + offset < self.lines[0].tiles.len() {
            tot_diff += self.get_col(i - offset).diff(&self.get_col(i + 1 + offset));
            if tot_diff > 1 {
                return false;
            }
            offset += 1;
        }
        tot_diff == 1
    }
}

fn part_a(input: &str) -> u64 {
    input
        .trim()
        .split("\n\n")
        .map(|field| Field::from_str(field).unwrap().find_spot().as_u64())
        .sum()
}

fn part_b(input: &str) -> u64 {
    input
        .trim()
        .split("\n\n")
        .map(|field| Field::from_str(field).unwrap().find_spot_smudge().as_u64())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#\n";
    }

    #[rstest]
    #[case("#.##..##.", Line { tiles: vec![
        Tile::Rock,
        Tile::Ash,
        Tile::Rock,
        Tile::Rock,
        Tile::Ash,
        Tile::Ash,
        Tile::Rock,
        Tile::Rock,
        Tile::Ash,
        ] } )]
    fn test_parse_line(#[case] line: &str, #[case] expected_line: Line) {
        assert_eq!(Line::from_str(line).unwrap(), expected_line)
    }

    #[rstest]
    #[case("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n", Field { lines: vec![
        Line::from_str("#.##..##.").unwrap(),
        Line::from_str("..#.##.#.").unwrap(),
        Line::from_str("##......#").unwrap(),
        Line::from_str("##......#").unwrap(),
        Line::from_str("..#.##.#.").unwrap(),
        Line::from_str("..##..##.").unwrap(),
        Line::from_str("#.#.##.#.").unwrap(),
    ] })]
    fn test_parse_field(#[case] field_str: &str, #[case] field: Field) {
        assert_eq!(Field::from_str(field_str).unwrap(), field)
    }

    #[rstest]
    #[case(
        "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n",
        MirrorSpot::Vertical(5)
    )]
    #[case(
        "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#",
        MirrorSpot::Horizontal(4)
    )]
    #[case(
        ".##..#..#.###\n.##.#...#..##\n..#.######.#.\n.......#.....\n.......#.....\n..#.######.#.\n.##.#...#..##\n.##..#..#.###\n####.#####.#.\n####.#####.#.\n.##.....#.###",
        MirrorSpot::Horizontal(4)
    )]
    #[case(
        "#.#..##..#.#..###\n#.##.##.##.#.####\n#.#......#.#.#...\n#..........#.####\n.##########..#.##\n#..######..####..\n##...###..##.####\n#..........######\n#.########.#..#..\n.#..#..#..#.###..\n.##..##..##..#...\n#.########.#..#..\n...#....#...#..##",
        MirrorSpot::Vertical(16)
    )]
    fn test_find_mirror_spot(#[case] field: Field, #[case] spot: MirrorSpot) {
        assert_eq!(field.find_spot(), spot)
    }

    #[rstest]
    #[case(
        "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n",
        MirrorSpot::Horizontal(3)
    )]
    #[case(
        "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#",
        MirrorSpot::Horizontal(1)
    )]
    fn test_find_mirror_spot_smudge(#[case] field: Field, #[case] spot: MirrorSpot) {
        assert_eq!(field.find_spot_smudge(), spot)
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        assert_eq!(part_a(example_input), 405)
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        assert_eq!(part_b(example_input), 400)
    }
}
