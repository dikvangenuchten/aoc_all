use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let map = input_file.parse().expect("Invalid input");
    let a = part_a(&map);
    let b = part_b(&map);
    (a, b)
}

fn part_a(map: &Map) -> u32 {
    map.calculate_cost_a()
}

fn part_b(map: &Map) -> u32 {
    map.calculate_cost_b()
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Ok(Map { map })
    }
}

impl Map {
    fn calculate_cost_a(&self) -> u32 {
        self.get_regions_a().iter().map(|r| r.cost()).sum()
    }

    fn get_regions_a(&self) -> Vec<Region> {
        let mut unassigned: BTreeSet<Coord> = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| Coord { x, y })
                    .collect::<Vec<Coord>>()
            })
            .collect();
        let size = (self.map[0].len(), self.map.len()).into();

        let mut regions = vec![];
        while let Some(coord) = unassigned.pop_first() {
            let mut stack = vec![coord];
            let cur_char = self.get(&coord);
            let mut area = 1;
            let mut perimeter = 0;

            while let Some(coord) = stack.pop() {
                for n in coord.neighbours(&size) {
                    match n {
                        Some(n) => {
                            if self.get(&n) == cur_char {
                                if unassigned.remove(&n) {
                                    stack.push(n);
                                    area += 1;
                                };
                            } else {
                                perimeter += 1;
                            }
                        }
                        None => perimeter += 1,
                    }
                }
            }

            regions.push(Region {
                area,
                perimeter,
                plant: cur_char,
            });
        }
        regions
    }

    fn calculate_cost_b(&self) -> u32 {
        self.get_regions_b().iter().map(|r| r.cost()).sum()
    }

    fn get_regions_b(&self) -> Vec<Region> {
        let mut unassigned: BTreeSet<Coord> = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| Coord { x, y })
                    .collect::<Vec<Coord>>()
            })
            .collect();
        let size = (self.map[0].len(), self.map.len()).into();

        let mut regions = vec![];
        while let Some(coord) = unassigned.pop_first() {
            let mut stack = vec![coord];
            let cur_char = self.get(&coord);
            let mut area = 1;

            let mut num_corners = 0;

            while let Some(coord) = stack.pop() {
                for n in coord.neighbours(&size) {
                    match n {
                        Some(n) => {
                            if self.get(&n) == cur_char {
                                if unassigned.remove(&n) {
                                    stack.push(n);
                                    area += 1;
                                };
                            } else {
                            }
                        }
                        None => {}
                    }
                }
                num_corners += self.count_corners(&coord, cur_char, &size);
            }

            regions.push(Region {
                area,
                perimeter: num_corners,
                plant: cur_char,
            });
        }
        regions
    }

    fn count_corners(&self, coord: &Coord, char: char, size: &Coord) -> u32 {
        // fn corners(
        // grid: &[Vec<char>],
        // row: usize,
        // col: usize,
        // x: char,
        // width: usize,
        // height: usize,
        // ) -> u8 {
        let check_up = coord.up().is_none_or(|c| self.get(&c) != char);
        let check_left = coord.left().is_none_or(|c| self.get(&c) != char);
        let check_right = coord.right(&size).is_none_or(|c| self.get(&c) != char);
        let check_down = coord.down(&size).is_none_or(|c| self.get(&c) != char);
        let check_up_left = coord
            .up()
            .is_none_or(|c| c.left().is_none_or(|c| self.get(&c) != char));
        let check_up_right = coord
            .up()
            .is_none_or(|c| c.right(&size).is_none_or(|c| self.get(&c) != char));
        let check_down_left = coord
            .down(&size)
            .is_none_or(|c| c.left().is_none_or(|c| self.get(&c) != char));
        let check_down_right = coord
            .down(&size)
            .is_none_or(|c| c.right(&size).is_none_or(|c| self.get(&c) != char));

        (check_up && check_left) as u32
            + (check_up && check_right) as u32
            + (check_down && check_left) as u32
            + (check_down && check_right) as u32
            + (!check_up && !check_left && check_up_left) as u32
            + (!check_up && !check_right && check_up_right) as u32
            + (!check_down && !check_left && check_down_left) as u32
            + (!check_down && !check_right && check_down_right) as u32
    }

    fn get(&self, coord: &Coord) -> char {
        self.map[coord.y][coord.x]
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Region {
    area: u32,
    perimeter: u32,
    plant: char,
}

impl Region {
    fn cost(&self) -> u32 {
        self.area * self.perimeter
    }
}

impl From<&(u32, u32, char)> for Region {
    fn from(value: &(u32, u32, char)) -> Self {
        Region {
            area: value.0,
            perimeter: value.1,
            plant: value.2,
        }
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
    fn up(&self) -> Option<Coord> {
        if self.y > 0 {
            Some(Coord::from((self.x, self.y - 1)))
        } else {
            None
        }
    }

    fn left(&self) -> Option<Coord> {
        if self.x > 0 {
            Some(Coord::from((self.x - 1, self.y)))
        } else {
            None
        }
    }

    fn right(&self, size: &Coord) -> Option<Coord> {
        if self.x < (size.x - 1) {
            Some(Coord::from((self.x + 1, self.y)))
        } else {
            None
        }
    }

    fn down(&self, size: &Coord) -> Option<Coord> {
        if self.y < (size.y - 1) {
            Some(Coord::from((self.x, self.y + 1)))
        } else {
            None
        }
    }

    fn neighbours(&self, size: &Coord) -> [Option<Coord>; 4] {
        let left = self.left();
        let right = self.right(size);
        let up = self.up();
        let down = self.down(size);
        [left, up, right, down]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";
        let map = Map {
            map: {
                vec![
                    vec!['A', 'A', 'A', 'A'],
                    vec!['B', 'B', 'C', 'D'],
                    vec!['B', 'B', 'C', 'C'],
                    vec!['E', 'E', 'E', 'C'],
                ]
            },
        };
        assert_eq!(Map::from_str(input), Ok(map))
    }

    #[rstest]
    #[case(Region { area: 4, perimeter: 8, plant: 'b'}, (4, 8, 'b'))]
    #[case(Region { area: 8, perimeter: 4, plant: 'b'}, (8, 4, 'b'))]
    fn test_region_from_tuple(#[case] region: Region, #[case] tuple: (u32, u32, char)) {
        assert_eq!(Region::from(&tuple), region)
    }

    #[rstest]
    #[case("AAAA\nBBCD\nBBCC\nEEEC", vec![(4, 10, 'A'), (4, 8, 'B'), (4, 10, 'C'), (1, 4, 'D'), (3, 8, 'E')])]
    fn test_get_regions(#[case] map: Map, #[case] regions: Vec<(u32, u32, char)>) {
        let regions: HashSet<Region> = regions.iter().map(|r| r.into()).collect();
        assert_eq!(
            map.get_regions_a().into_iter().collect::<HashSet<Region>>(),
            regions
        )
    }

    #[rstest]
    #[case("AAAA\nBBCD\nBBCC\nEEEC", vec![(4, 4, 'A'), (4, 4, 'B'), (4, 8, 'C'), (1, 4, 'D'), (3, 4, 'E')])]
    fn test_get_regions_b(#[case] map: Map, #[case] regions: Vec<(u32, u32, char)>) {
        let regions: HashSet<Region> = regions.iter().map(|r| r.into()).collect();
        assert_eq!(
            map.get_regions_b().into_iter().collect::<HashSet<Region>>(),
            regions
        )
    }

    #[rstest]
    #[case("AAAA\nBBCD\nBBCC\nEEEC", 140)]
    #[case("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO", 772)]
    #[case("RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE", 1930)]
    fn test_part_a(#[case] map: Map, #[case] cost: u32) {
        assert_eq!(part_a(&map), cost)
    }

    #[rstest]
    #[case("AAAA\nBBCD\nBBCC\nEEEC", 80)]
    #[case("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO", 436)]
    #[case("RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE", 1206)]
    fn test_part_b(#[case] map: Map, #[case] cost: u32) {
        assert_eq!(part_b(&map), cost)
    }
}
