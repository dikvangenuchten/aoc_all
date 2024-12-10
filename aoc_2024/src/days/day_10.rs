use std::{collections::HashSet, str::FromStr};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let a = part_a(input_file);
    let b = part_b(input_file);
    (a, b)
}

fn part_a(input_file: &str) -> u32 {
    let map = Map::from_str(input_file).unwrap();
    map.count_all_arrow_heads_unique()
}

fn part_b(input_file: &str) -> u32 {
    let map = Map::from_str(input_file).unwrap();
    map.count_all_arrow_heads_distinct()
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;
impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<u32>> = s
            .trim()
            .split("\n")
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap_or(16)).collect())
            .collect();
        let x = map[0].len();
        let y = map.len();
        let size = Coord { x, y };
        Ok(Map { map, size })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Coord {
            x: value.0,
            y: value.1,
        }
    }
}

impl Coord {
    fn neighbours(&self, size: &Coord) -> [Option<Coord>; 4] {
        // Left
        let left = {
            if self.x > 0 {
                Some(Coord {
                    x: self.x - 1,
                    y: self.y,
                })
            } else {
                None
            }
        };
        let right = {
            if self.x < (size.x - 1) {
                Some(Coord {
                    x: self.x + 1,
                    y: self.y,
                })
            } else {
                None
            }
        };
        let up = {
            if self.y > 0 {
                Some(Coord {
                    x: self.x,
                    y: self.y - 1,
                })
            } else {
                None
            }
        };
        let down = {
            if self.y < (size.y - 1) {
                Some(Coord {
                    x: self.x,
                    y: self.y + 1,
                })
            } else {
                None
            }
        };
        [left, up, right, down]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<u32>>,
    size: Coord,
}

impl Map {
    fn count_all_arrow_heads_unique(&self) -> u32 {
        let mut sum = 0;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                sum += self.count_arrow_heads_unique_at((x, y).into());
            }
        }
        sum
    }

    fn count_arrow_heads_unique_at(&self, coord: Coord) -> u32 {
        if self.get_coord(&coord) != 0 {
            return 0;
        }
        self.recursive_find_paths(&coord, 0).len() as u32
    }

    fn recursive_find_paths(&self, coord: &Coord, cur_height: u32) -> HashSet<Coord> {
        let mut unique = HashSet::new();

        for next in coord.neighbours(&self.size).into_iter().flatten() {
            if self.get_coord(&next) == cur_height + 1 {
                if cur_height + 1 == 9 {
                    unique.insert(next);
                } else {
                    unique.extend(&self.recursive_find_paths(&next, cur_height + 1));
                }
            }
        }
        unique
    }

    fn count_all_arrow_heads_distinct(&self) -> u32 {
        let mut sum = 0;
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                sum += self.count_arrow_heads_distinct_at((x, y).into());
            }
        }
        sum
    }

    fn count_arrow_heads_distinct_at(&self, coord: Coord) -> u32 {
        if self.get_coord(&coord) != 0 {
            return 0;
        }
        self.recursive_find_distinct_paths(&coord, 0)
    }

    fn recursive_find_distinct_paths(&self, coord: &Coord, cur_height: u32) -> u32 {
        let mut sum = 0;

        for next in coord.neighbours(&self.size).into_iter().flatten() {
            if self.get_coord(&next) == cur_height + 1 {
                if cur_height + 1 == 9 {
                    sum += 1;
                } else {
                    sum += self.recursive_find_distinct_paths(&next, cur_height + 1);
                }
            }
        }
        sum
    }

    fn get_coord(&self, coord: &Coord) -> u32 {
        self.map[coord.y][coord.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0123\n1234\n8765\n9876", Map { map: vec![
        vec![0,1,2,3,],
        vec![1,2,3,4,],
        vec![8,7,6,5,],
        vec![9,8,7,6,],
    ], size: Coord { x: 4, y: 4 }})]
    fn test_parse(#[case] input_file: &str, #[case] map: Map) {
        assert_eq!(Map::from_str(input_file), Ok(map))
    }

    #[rstest]
    #[case("...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9", (3, 1), 0)]
    #[case("...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9", (3, 0), 2)]
    #[case("..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....", (3, 0), 4)]
    fn test_count_single_arrow_head(
        #[case] map: Map,
        #[case] point: (usize, usize),
        #[case] num_heads: u32,
    ) {
        assert_eq!(map.count_arrow_heads_unique_at(point.into()), num_heads)
    }

    #[rstest]
    #[case("...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9", 2)]
    #[case("..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....", 4)]
    #[case(
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732",
        36
    )]
    fn test_count_all_arrow_head(#[case] map: Map, #[case] num_heads: u32) {
        assert_eq!(map.count_all_arrow_heads_unique(), num_heads)
    }

    #[rstest]
    #[case("...0...\n...1...\n...2...\n6543456\n7.....7\n8.....8\n9.....9", 2)]
    #[case("..90..9\n...1.98\n...2..7\n6543456\n765.987\n876....\n987....", 4)]
    #[case(
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732",
        36
    )]
    fn test_part_a(#[case] map: &str, #[case] num_heads: u32) {
        assert_eq!(part_a(map), num_heads)
    }

    #[rstest]
    #[case(
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732",
        81
    )]
    fn test_part_b(#[case] map: &str, #[case] num_heads: u32) {
        assert_eq!(part_b(map), num_heads)
    }
}
