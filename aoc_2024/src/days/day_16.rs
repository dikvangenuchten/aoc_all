use std::{
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet},
    str::FromStr,
    u32,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let map: Map = input_file.parse().expect("Valid input");
    let (a, b) = map.solve_map();
    // let a = part_a(input_file);
    // println!("Finished A");
    // let b = part_b(input_file, a);
    (a, b)
}

fn part_a(input_file: &str) -> u32 {
    let map: Map = input_file.parse().expect("Valid input");
    map.solve_map().0
}

fn part_b(input_file: &str, max_cost: u32) -> u32 {
    let map: Map = input_file.parse().expect("Valid input");
    map.solve_map().1
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
struct StateC {
    cost: i32,
    pos: Coord,
    dir: Direction,
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

    fn count_paths_with_max_cost(&self, max_cost: u32) -> u32 {
        let start_state = (self.start, Direction::Right);
        let mut possible_tiles = HashMap::new();
        self.count_paths_rec(&mut possible_tiles, &start_state, max_cost);
        let mut tiles = possible_tiles
            .keys()
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>();

        let mut tiles: Vec<Coord> = tiles.drain().copied().collect();
        tiles.sort();
        dbg!(&tiles);
        println!("Tiles: {:?}", &tiles);
        tiles.len() as u32
    }

    fn count_paths_rec(
        &self,
        possible_tiles: &mut HashMap<(Coord, Direction), u32>,
        cur_state: &(Coord, Direction),
        budget: u32,
    ) -> bool {
        if cur_state.0 == self.end {
            assert!(budget == 0);
            possible_tiles.insert(*cur_state, budget);
            return true;
        }
        let mut reachable = false;
        if budget >= 1 {
            // Try forward
            let forward_pos = cur_state.0.add(&cur_state.1);
            let forward_state = (forward_pos, cur_state.1);
            if let Some(future_budget) = possible_tiles.get(&forward_state) {
                match future_budget.cmp(&(budget - 1)) {
                    std::cmp::Ordering::Less => panic!("Found a shorter route: {:?}", forward_pos),
                    std::cmp::Ordering::Equal => {
                        reachable = true;
                    }
                    std::cmp::Ordering::Greater => println!("More expensive path"),
                }
            } else if self.get(&forward_state.0) == &Part::Empty && self.count_paths_rec(possible_tiles, &forward_state, budget - 1) {
                reachable = true;
            }
        }
        if budget > 1000 {
            // Check clock wise
            let new_dir = cur_state.1.clock();
            // Only allow turns when that turn leads to an empty straight
            let future_state = (cur_state.0, new_dir);
            let future_pos = cur_state.0.add(&new_dir);
            if let Some(future_budget) = possible_tiles.get(&future_state) {
                match future_budget.cmp(&(budget - 1000)) {
                    std::cmp::Ordering::Less => panic!("Found a shorter route: {:?}", future_pos),
                    std::cmp::Ordering::Equal => {
                        reachable = true;
                    }
                    std::cmp::Ordering::Greater => println!("More expensive path"),
                }
            } else if self.get(&future_pos) == &Part::Empty && self.count_paths_rec(possible_tiles, &future_state, budget - 1000) {
                reachable = true;
            }

            // Check counter clock wise
            let new_dir = cur_state.1.counter();
            // Only allow turns when that turn leads to an empty straight
            let future_state = (cur_state.0, new_dir);
            let future_pos = cur_state.0.add(&new_dir);
            if let Some(future_budget) = possible_tiles.get(&future_state) {
                match future_budget.cmp(&(budget - 1000)) {
                    std::cmp::Ordering::Less => panic!("Found a shorter route"),
                    std::cmp::Ordering::Equal => {
                        reachable = true;
                    }
                    std::cmp::Ordering::Greater => println!("More expensive path"),
                }
            } else if self.get(&future_pos) == &Part::Empty && self.count_paths_rec(possible_tiles, &(cur_state.0, new_dir), budget - 1000) {
                reachable = true;
            }
        }
        if reachable {
            possible_tiles.insert(*cur_state, budget);
        }
        if cur_state.0 == (Coord { x: 3, y: 10 }) {
            println!("Is_reachable: {:?}", reachable);
        }

        reachable
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
    #[case("###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############", 7036)]
    fn test_part_a(#[case] map: &str, #[case] cost: u32) {
        assert_eq!(part_a(map), cost)
    }

    #[rstest]
    #[case("###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############", 45)]
    fn test_part_b(#[case] map: &str, #[case] num_tiles: u32) {
        let a = part_a(map);
        assert_eq!(part_b(map, a), num_tiles)
    }
}
