use std::fmt::Display;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let map = parse(input_file);
    let (a, b) = part_a(map.clone());
    // let b = part_b(&map);
    (a, b)
}

pub fn part_a(map: Vec<Vec<Tile>>) -> (u64, u64) {
    let mut new_map = Vec::with_capacity(map.len());
    let mut split_count = 0;
    new_map.push(map[0].clone());
    for row_idx in 0..map.len() {
        let new_row = vec![Tile::Empty; map[row_idx].len()];
        new_map.push(new_row);
        for tile_idx in 0..map[row_idx].len() {
            match map[row_idx][tile_idx] {
                Tile::Start => {
                    // Launch beam
                    new_map[row_idx + 1][tile_idx] = Tile::Beam(1);
                }
                Tile::Splitter => {
                    if let Tile::Beam(x) = new_map[row_idx - 1][tile_idx] {
                        split_count += 1;
                        if tile_idx > 0
                            && let Some(left_tile) = new_map[row_idx].get_mut(tile_idx - 1)
                        {
                            left_tile.add_tile(Tile::Beam(x));
                        }
                        if let Some(right_tile) = new_map[row_idx].get_mut(tile_idx + 1) {
                            right_tile.add_tile(Tile::Beam(x));
                        }
                    }
                    new_map[row_idx][tile_idx] = Tile::Splitter;
                }
                Tile::Empty => {
                    if row_idx > 0
                        && let Tile::Beam(x) = new_map[row_idx - 1][tile_idx]
                    {
                        new_map[row_idx][tile_idx].add_tile(Tile::Beam(x));
                    }
                }
                Tile::Beam(_) => {}
            }
        }
    }
    new_map.pop(); // An extra row is added for simplicity, but is invalid.
    let total_beams = new_map
        .pop()
        .expect("There should be a row")
        .iter()
        .fold(
            0,
            |acc, t| {
                if let Tile::Beam(x) = t { acc + x } else { acc }
            },
        );
    (split_count, total_beams)
}

fn parse(input_file: &str) -> Vec<Vec<Tile>> {
    let mut map = Vec::new();
    for line in input_file.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                'S' => Tile::Start,
                '^' => Tile::Splitter,
                _ => panic!("Invalid tile character: {}", c),
            })
            .collect::<Vec<Tile>>();
        map.push(row);
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Start,
    Splitter,
    Beam(u64),
}

impl Tile {
    fn add_tile(&mut self, other: Self) {
        let count = match other {
            Tile::Beam(x) => x,
            _ => 0,
        };
        match self {
            Tile::Beam(x) => *x += count,
            Tile::Empty => *self = Tile::Beam(count),
            _ => {}
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Start => 'S',
            Tile::Splitter => '^',
            Tile::Beam(x) => format!("{x:X}").chars().next().unwrap_or('|'),
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(".S.\n.^.\n...", vec![
        vec![Tile::Empty, Tile::Start, Tile::Empty],
        vec![Tile::Empty, Tile::Splitter, Tile::Empty],
        vec![Tile::Empty, Tile::Empty, Tile::Empty],
    ])]
    fn test_parse(#[case] input: &str, #[case] expected: Vec<Vec<Tile>>) {
        let result = parse(input);
        // Compare the parsed result with the expected value
        for (r1, r2) in result.iter().zip(expected.iter()) {
            for (t1, t2) in r1.iter().zip(r2.iter()) {
                match (t1, t2) {
                    (Tile::Empty, Tile::Empty) => {}
                    (Tile::Start, Tile::Start) => {}
                    (Tile::Splitter, Tile::Splitter) => {}
                    _ => panic!("Tiles do not match"),
                }
            }
        }
    }

    #[rstest]
    #[case(".S.\n...\n.^.", (1, 2))]
    #[case(".S..\n....\n.^..\n....\n^...\n.^.^\n....", (3, 3))]
    fn test_part_a_simple(#[case] input: &str, #[case] expected: (u64, u64)) {
        let map = parse(input);
        let result = part_a(map);
        assert_eq!(result, expected);
    }

    #[rstest]
    fn test_part_a() {
        let input_file = read_test_day_input("07");
        let map = parse(&input_file);
        let result = part_a(map);
        assert_eq!(result, (21, 40));
    }
}
