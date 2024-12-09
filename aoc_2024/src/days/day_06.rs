use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let map = Map::from_str(input_file).unwrap();
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

    let efficient_map = EfficientMap::from_map(map);

    visited
        .par_iter()
        .filter(|obs| obs != &&map.guard.pos)
        .filter(|obs| efficient_map.check_loop(&map.guard, obs))
        .count() as u32
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<MapPart>>,
    guard: Guard,
    oob: MapPart,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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

    fn sub(&self, dir: &Dir) -> Self {
        let Point { mut x, mut y } = self;
        match dir {
            Dir::Up => y += 1,
            Dir::Down => y -= 1,
            Dir::Left => x += 1,
            Dir::Right => x -= 1,
        };
        Point { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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

    fn get_size(&self) -> (usize, usize) {
        let y = self.map.len();
        let x = self.map[0].len();
        (x, y)
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

#[derive(Debug)]
struct EfficientMap {
    obs_xy: Vec<Vec<Point>>,
    obs_yx: Vec<Vec<Point>>,
}

impl EfficientMap {
    fn from_map(map: &Map) -> Self {
        let (x, y) = map.get_size();

        let obs_xy = vec![vec![]; x];
        let obs_yx = vec![vec![]; y];

        let mut e_map = EfficientMap { obs_xy, obs_yx };

        for (y_i, row) in map.map.iter().enumerate() {
            for (x_i, part) in row.iter().enumerate() {
                if part == &MapPart::Obstacle {
                    let (x, y) = (x_i as i32, y_i as i32);
                    let obstacle = Point { x, y };
                    e_map.insert_point(obstacle);
                }
            }
        }
        e_map
    }

    fn insert_point(&mut self, point: Point) {
        let (x, y): (usize, usize) = (point.x.try_into().unwrap(), point.y.try_into().unwrap());
        if let Err(idx) = self.obs_xy[x].binary_search(&point) {
            self.obs_xy[x].insert(idx, point);
        }
        if let Err(idx) = self.obs_yx[y].binary_search(&point) {
            self.obs_yx[y].insert(idx, point);
        }
    }

    fn check_loop(&self, guard: &Guard, obstacle: &Point) -> bool {
        let mut visited = BTreeSet::new();
        let mut guard = *guard;
        let mut is_loop = false;

        while let Some(obstacle) = self.get_next_obs_with_extra_obs(&guard, obstacle) {
            guard.pos = obstacle.sub(&guard.dir);
            guard.dir = guard.dir.next();
            if !visited.insert(guard) {
                is_loop = true;
                break;
            }
        }

        is_loop
    }

    fn get_next_obs(&self, guard: &Guard) -> Option<Point> {
        let (x, y): (usize, usize) = (
            guard.pos.x.try_into().unwrap(),
            guard.pos.y.try_into().unwrap(),
        );
        let point = match guard.dir {
            Dir::Up => {
                let all_obs = &self.obs_xy[x];
                let idx = all_obs.binary_search(&guard.pos).err().unwrap();
                all_obs.get(idx.wrapping_sub(1))
            }
            Dir::Down => {
                let all_obs = &self.obs_xy[x];
                let idx = all_obs.binary_search(&guard.pos).err().unwrap();
                all_obs.get(idx)
            }
            Dir::Left => {
                let all_obs = &self.obs_yx[y];
                let idx = all_obs.binary_search(&guard.pos).err().unwrap();
                all_obs.get(idx.wrapping_sub(1))
            }
            Dir::Right => {
                let all_obs = &self.obs_yx[y];
                let idx = all_obs.binary_search(&guard.pos).err().unwrap();
                all_obs.get(idx)
            }
        };
        point.copied()
    }

    fn get_next_obs_with_extra_obs(&self, guard: &Guard, extra_obs: &Point) -> Option<Point> {
        if let Some(obs) = self.get_next_obs(guard) {
            match guard.dir {
                Dir::Up => {
                    if extra_obs.x == obs.x && extra_obs.y > obs.y && extra_obs.y < guard.pos.y {
                        Some(*extra_obs)
                    } else {
                        Some(obs)
                    }
                }
                Dir::Down => {
                    if extra_obs.x == obs.x && extra_obs.y < obs.y && extra_obs.y > guard.pos.y {
                        Some(*extra_obs)
                    } else {
                        Some(obs)
                    }
                }
                Dir::Left => {
                    if extra_obs.y == obs.y && extra_obs.x > obs.x && extra_obs.x < guard.pos.x {
                        Some(*extra_obs)
                    } else {
                        Some(obs)
                    }
                }
                Dir::Right => {
                    if extra_obs.y == obs.y && extra_obs.x < obs.x && extra_obs.x > guard.pos.x {
                        Some(*extra_obs)
                    } else {
                        Some(obs)
                    }
                }
            }
        } else {
            match guard.dir {
                Dir::Up => {
                    if extra_obs.x == guard.pos.x && extra_obs.y < guard.pos.y {
                        Some(*extra_obs)
                    } else {
                        None
                    }
                }
                Dir::Down => {
                    if extra_obs.x == guard.pos.x && extra_obs.y > guard.pos.y {
                        Some(*extra_obs)
                    } else {
                        None
                    }
                }
                Dir::Left => {
                    if extra_obs.y == guard.pos.y && extra_obs.x < guard.pos.x {
                        Some(*extra_obs)
                    } else {
                        None
                    }
                }
                Dir::Right => {
                    if extra_obs.y == guard.pos.y && extra_obs.x > guard.pos.x {
                        Some(*extra_obs)
                    } else {
                        None
                    }
                }
            }
        }
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
    fn test_check_loop_efficient(
        example_map: Map,
        #[case] obstacle_point: Point,
        #[case] is_loop: bool,
    ) {
        let map = EfficientMap::from_map(&example_map);
        assert_eq!(map.check_loop(&example_map.guard, &obstacle_point), is_loop)
    }

    #[rstest]
    fn test_part_b(example_map: Map) {
        assert_eq!(part_b(&example_map), 6)
    }

    #[rstest]
    #[case( Guard { pos: Point {x: 4, y: 6}, dir: Dir::Up }, Some(Point { x: 4, y: 0 }))]
    #[case( Guard { pos: Point {x: 8, y: 6}, dir: Dir::Up }, None)]
    #[case( Guard { pos: Point {x: 8, y: 6}, dir: Dir::Down }, Some(Point { x: 8, y: 7 }))]
    #[case( Guard { pos: Point {x: 4, y: 6}, dir: Dir::Down }, None)]
    #[case( Guard { pos: Point {x: 8, y: 0}, dir: Dir::Left }, Some(Point { x: 4, y: 0 }))]
    #[case( Guard { pos: Point {x: 3, y: 0}, dir: Dir::Left }, None)]
    #[case( Guard { pos: Point {x: 3, y: 0}, dir: Dir::Right }, Some(Point { x: 4, y: 0 }))]
    #[case( Guard { pos: Point {x: 8, y: 0}, dir: Dir::Right }, None)]
    fn test_efficient_next_pos(
        example_map: Map,
        #[case] guard: Guard,
        #[case] expected: Option<Point>,
    ) {
        let map = EfficientMap::from_map(&example_map);
        assert_eq!(map.get_next_obs(&guard), expected)
    }

    #[rstest]
    #[case(vec![Point {x: 1, y: 0}, Point {x: 0, y: 0}], vec![Point {x: 0, y: 0}, Point {x: 1, y: 0}])]
    #[case(vec![Point {x: 0, y: 0}, Point {x: 1, y: 0}], vec![Point {x: 0, y: 0}, Point {x: 1, y: 0}])]
    #[case(vec![Point {x: 0, y: 1}, Point {x: 0, y: 0}], vec![Point {x: 0, y: 0}, Point {x: 0, y: 1}])]
    #[case(vec![Point {x: 0, y: 0}, Point {x: 0, y: 1}], vec![Point {x: 0, y: 0}, Point {x: 0, y: 1}])]
    fn test_point_sorting(#[case] mut unsorted: Vec<Point>, #[case] sorted: Vec<Point>) {
        unsorted.sort();
        assert_eq!(unsorted, sorted)
    }
}
