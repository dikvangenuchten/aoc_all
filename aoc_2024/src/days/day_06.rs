use std::{collections::HashSet, str::FromStr};

pub fn solve_day(input_file: String) -> (u32, u32) {
    let map = Map::from_str(&input_file).unwrap();
    let a = part_a(&map);
    let b = part_b(&map);
    (a, b)
}

fn part_a(map: &Map) -> u32 {
    // Find current guard spot
    let visited = map.get_visited();
    visited.len() as u32
}

fn part_b(map: &Map) -> u32 {
    // It only makes sense to place obstacles on walkable path
    let visited = map.get_visited();

    visited
        .iter()
        .filter(|obs| obs != &&map.guard.pos)
        .filter(|obs| map.check_loop(obs))
        .count() as u32
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<MapPart>>,
    guard: Guard,
    oob: MapPart,
}

#[derive(Debug, PartialEq, Eq)]
enum MapPart {
    Obstacle,
    Empty,
    Out,
}

impl FromStr for MapPart {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char = s.chars().next();
        match char {
            Some('.') => Ok(MapPart::Empty),
            Some('#') => Ok(MapPart::Obstacle),
            Some('^') => Ok(MapPart::Empty),
            Some(_) => todo!(),
            None => todo!(),
        }
    }
}

impl MapPart {
    fn from_char(char: &char) -> Result<Self, ParseError> {
        match char {
            '.' => Ok(MapPart::Empty),
            '#' => Ok(MapPart::Obstacle),
            '^' => Ok(MapPart::Empty),
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Guard {
    pos: Point,
    dir: Dir,
}
impl Guard {
    fn next_pos(&self) -> Point {
        let mut pos = self.pos;
        pos.add(&self.dir);
        pos
    }
}

impl Point {
    fn add(&mut self, dir: &Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

impl Map {
    fn get(&self, point: Point) -> &MapPart {
        if point.y < 0 || point.x < 0 {
            return &self.oob;
        }
        let x: usize = point.x.try_into().expect("Checked before");
        let y: usize = point.y.try_into().expect("Checked before");

        if let Some(row) = self.map.get(y) {
            if let Some(spot) = row.get(x) {
                return spot;
            }
        }
        &self.oob
    }

    fn get_visited(&self) -> HashSet<Point> {
        // Find current guard spot
        let mut visited = HashSet::new();
        let mut guard = self.guard;
        loop {
            match self.get(guard.next_pos()) {
                MapPart::Obstacle => {
                    guard.dir = guard.dir.next();
                }
                MapPart::Empty => {
                    guard.pos.add(&guard.dir);
                    visited.insert(guard.pos);
                }
                MapPart::Out => return visited,
            };
        }
    }

    fn get_obs(&self, point: Point, obs: &Point) -> &MapPart {
        if &point == obs {
            return &MapPart::Obstacle;
        }
        return self.get(point);
    }

    fn check_loop(&self, extra_obs: &Point) -> bool {
        let mut visited = HashSet::new();
        let mut guard = self.guard;
        loop {
            match self.get_obs(guard.next_pos(), extra_obs) {
                MapPart::Obstacle => {
                    guard.dir = guard.dir.next();
                }
                MapPart::Empty => {
                    guard.pos.add(&guard.dir);
                    if !visited.insert((guard.pos, guard.dir)) {
                        return true;
                    };
                }
                MapPart::Out => return false,
            };
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .trim()
            .split("\n")
            .map(|line| {
                line.chars()
                    .map(|s| MapPart::from_char(&s).unwrap())
                    .collect()
            })
            .collect();

        let mut guard_opt = None;
        for (y, line) in s.trim().split("\n").enumerate() {
            for (x, c) in line.char_indices() {
                if c == '^' {
                    guard_opt = Some(Guard {
                        pos: Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        dir: Dir::Up,
                    });
                }
            }
        }

        Ok(Map {
            map,
            guard: guard_opt.unwrap(),
            oob: MapPart::Out,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...", Map { map: vec![
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Obstacle,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Obstacle],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Obstacle,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Obstacle,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Obstacle,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Obstacle,MapPart::Empty],
vec![MapPart::Obstacle,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty],
vec![MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Obstacle,MapPart::Empty,MapPart::Empty,MapPart::Empty],
], guard: Guard { pos: Point {x: 4, y: 6}, dir: Dir::Up }, oob: MapPart::Out,}
    )]
    fn test_parse(#[case] input_file: &str, #[case] map: Map) {
        assert_eq!(Map::from_str(input_file).unwrap(), map)
    }

    #[fixture]
    fn example_map() -> Map {
        let input_file = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        Map::from_str(input_file).unwrap()
    }

    #[rstest]
    #[case(Point {x: 0, y: 0}, MapPart::Empty)]
    #[case(Point {x: 4, y: 0}, MapPart::Obstacle)]
    #[case(Point {x: 0, y: 4}, MapPart::Empty)]
    fn test_map_get(example_map: Map, #[case] pos: Point, #[case] expected_part: MapPart) {
        assert_eq!(example_map.get(pos), &expected_part);
    }

    #[rstest]
    fn test_part_a(example_map: Map) {
        assert_eq!(part_a(&example_map), 41)
    }

    #[rstest]
    #[case(Point {x: 0, y: 0}, false)]
    #[case(Point {x: 3, y: 6}, true)]
    #[case(Point {x: 6, y: 7}, true)]
    fn test_check_loop(example_map: Map, #[case] obstacle_point: Point, #[case] is_loop: bool) {
        assert_eq!(example_map.check_loop(&obstacle_point), is_loop)
    }

    #[rstest]
    fn test_part_b(example_map: Map) {
        assert_eq!(part_b(&example_map), 6)
    }
}
