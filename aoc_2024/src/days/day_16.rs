use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let map: Map = input_file.parse().expect("Valid input");
    let (a, b) = map.solve_map();

    (a, b)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq)]
enum Part {
    Wall,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<Part>>,
    start: Coord,
    end: Coord,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn clock(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn counter(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Right => Self::Up,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
        }
    }
}

impl Coord {
    fn add(&self, dir: &Direction) -> Coord {
        match dir {
            Direction::Up => (self.x, self.y - 1).into(),
            Direction::Right => (self.x + 1, self.y).into(),
            Direction::Down => (self.x, self.y + 1).into(),
            Direction::Left => (self.x - 1, self.y).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (usize::MAX, usize::MAX);
        let mut end = (usize::MAX, usize::MAX);

        let map = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.char_indices()
                    .map(|(x, part)| match part {
                        '#' => Part::Wall,
                        '.' => Part::Empty,
                        'S' => {
                            start = (x, y);
                            Part::Empty
                        }
                        'E' => {
                            end = (x, y);
                            Part::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            map,
            start: start.into(),
            end: end.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct State {
    pos: Coord,
    dir: Direction,
}

impl Map {
    fn solve_map(&self) -> (u32, u32) {
        let start = (
            0,
            State {
                pos: self.start,
                dir: Direction::Right,
            },
        );
        let mut todo = BinaryHeap::from([start]);

        let mut dist = HashMap::<State, i32>::from([(start.1, 0)]);
        let mut prev = HashMap::<State, BTreeSet<State>>::new();
        let mut visited = HashMap::<(Coord, Direction), State>::new();
        let mut end_nodes = vec![];
        let mut end_cost = i32::MIN;

        while let Some((cost, node)) = todo.pop() {
            if node.pos == self.end && cost > end_cost {
                end_cost = cost;
                // There might be multiple directions in which the reindeer can end.
                match dist.get(&node).expect("Must be inserted").cmp(&cost) {
                    std::cmp::Ordering::Less => end_nodes = vec![node],
                    std::cmp::Ordering::Equal => end_nodes.push(node),
                    std::cmp::Ordering::Greater => (),
                }
            }
            visited.insert((node.pos, node.dir), node);

            let straight = node.pos.add(&node.dir);
            if self.get(&straight) == &Part::Empty {
                let new_state = State {
                    pos: straight,
                    dir: node.dir,
                };
                if dist
                    .get(&new_state)
                    .is_none_or(|prev_dist| *prev_dist < cost)
                {
                    todo.push((cost - 1, new_state));
                    if dist
                        .get(&new_state)
                        .is_some_and(|prev_dist| *prev_dist == cost - 1)
                    {
                        prev.get_mut(&new_state).unwrap().insert(node);
                    } else {
                        dist.insert(new_state, cost - 1);
                        prev.insert(new_state, BTreeSet::from([node]));
                    }
                }
            }

            let clock_state = State {
                pos: node.pos,
                dir: node.dir.clock(),
            };
            if dist
                .get(&clock_state)
                .is_none_or(|prev_dist| *prev_dist <= cost - 1000)
            {
                todo.push((cost - 1000, clock_state));
                if dist
                    .get(&clock_state)
                    .is_some_and(|prev_dist| *prev_dist == cost - 1000)
                {
                    prev.get_mut(&clock_state).unwrap().insert(node);
                } else {
                    dist.insert(clock_state, cost - 1000);
                    prev.insert(clock_state, BTreeSet::from([node]));
                }
            }

            let counter_state = State {
                pos: node.pos,
                dir: node.dir.counter(),
            };
            if dist
                .get(&counter_state)
                .is_none_or(|prev_dist| *prev_dist <= cost - 1000)
            {
                todo.push((cost - 1000, counter_state));
                if dist
                    .get(&counter_state)
                    .is_some_and(|prev_dist| *prev_dist == cost - 1000)
                {
                    prev.get_mut(&counter_state).unwrap().insert(node);
                } else {
                    dist.insert(counter_state, cost - 1000);
                    prev.insert(counter_state, BTreeSet::from([node]));
                }
            }
        }

        let max_cost = dist.get(end_nodes.first().unwrap()).unwrap().unsigned_abs();
        let mut visited = HashSet::<Coord>::from([self.start]);

        while let Some(node) = end_nodes.pop() {
            visited.insert(node.pos);
            if let Some(prevs) = prev.get(&node) {
                assert!(prevs.len() <= 2);
                end_nodes.extend(prevs);
            }
        }

        (max_cost, visited.len() as u32)
    }

    fn get(&self, coord: &Coord) -> &Part {
        self.map.get(coord.y).unwrap().get(coord.x).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############",
        Map { map: vec![
            vec![Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Wall,Part::Wall,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Empty,Part::Wall,Part::Empty,Part::Empty,Part::Empty,Part::Wall],
            vec![Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall,Part::Wall],
        ], start: Coord {x: 1, y: 13},
        end: Coord {x: 13, y: 1},}
    )]
    fn test_parse(#[case] input: &str, #[case] map: Map) {
        assert_eq!(input.parse(), Ok(map))
    }

    #[rstest]
    #[case("###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############", 7036)]
    fn test_solve_map(#[case] map: Map, #[case] cost: u32) {
        assert_eq!(map.solve_map().0, cost)
    }

    #[rstest]
    #[case("###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############", 7036, 45)]
    fn test_day(#[case] map: &str, #[case] cost: u32, #[case] num_tiles: u32) {
        assert_eq!(solve_day(map), (cost, num_tiles))
    }
}
