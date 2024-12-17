use std::{
    collections::{BTreeSet, BinaryHeap, HashMap},
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

#[derive(Debug, Default)]
struct DijkstraState {
    todo: BinaryHeap<(i32, State)>,
    dist: HashMap<State, i32>,
    prev: HashMap<State, Vec<State>>,
}

impl Map {
    fn solve_map(&self) -> (u32, u32) {
        let (dijkstra_state, mut end_nodes) = self.dijkstra();

        let max_cost = dijkstra_state
            .dist
            .get(end_nodes.first().unwrap())
            .unwrap()
            .unsigned_abs();

        let mut visited = BTreeSet::<Coord>::from([self.start]);
        while let Some(node) = end_nodes.pop() {
            visited.insert(node.pos);
            if let Some(prevs) = dijkstra_state.prev.get(&node) {
                assert!(prevs.len() <= 2, "{:?}", prevs);
                end_nodes.extend(prevs);
            }
        }

        (max_cost, visited.len() as u32)
    }

    fn dijkstra(&self) -> (DijkstraState, Vec<State>) {
        let start = (
            0,
            State {
                pos: self.start,
                dir: Direction::Right,
            },
        );
        let mut dijkstra_state = DijkstraState::default();
        dijkstra_state.todo.push(start);
        dijkstra_state.dist.insert(start.1, 0);

        let mut end_nodes = vec![];
        let mut end_cost = i32::MIN;

        while let Some((cost, node)) = dijkstra_state.todo.pop() {
            self.dijkstra_step(
                &mut dijkstra_state,
                &mut end_nodes,
                &mut end_cost,
                cost,
                node,
            );
        }
        (dijkstra_state, end_nodes)
    }

    fn dijkstra_step(
        &self,
        dijkstra_state: &mut DijkstraState,
        end_nodes: &mut Vec<State>,
        end_cost: &mut i32,
        cost: i32,
        node: State,
    ) {
        if node.pos == self.end && cost > *end_cost {
            *end_cost = cost;
            // There might be multiple directions in which the reindeer can end.
            match dijkstra_state
                .dist
                .get(&node)
                .expect("Must be inserted")
                .cmp(&cost)
            {
                std::cmp::Ordering::Less => *end_nodes = vec![node],
                std::cmp::Ordering::Equal => end_nodes.push(node),
                std::cmp::Ordering::Greater => (),
            }
        }

        if self.get_straight(&node) == &Part::Empty {
            let new_state = State {
                pos: node.pos.add(&node.dir),
                dir: node.dir,
            };
            check_insert_new(dijkstra_state, new_state, cost - 1, node);
        }

        let clock_state = State {
            pos: node.pos,
            dir: node.dir.clock(),
        };
        if self.get_straight(&clock_state) == &Part::Empty {
            check_insert_new(dijkstra_state, clock_state, cost - 1000, node);
        }

        let counter_state = State {
            pos: node.pos,
            dir: node.dir.counter(),
        };
        if self.get_straight(&counter_state) == &Part::Empty {
            check_insert_new(dijkstra_state, counter_state, cost - 1000, node);
        }
    }

    fn get(&self, coord: &Coord) -> &Part {
        self.map.get(coord.y).unwrap().get(coord.x).unwrap()
    }

    fn get_straight(&self, state: &State) -> &Part {
        let straight = state.pos.add(&state.dir);
        self.get(&straight)
    }
}

fn check_insert_new(dijkstra_state: &mut DijkstraState, next_state: State, cost: i32, node: State) {
    // todo: &mut BinaryHeap<(i32, State)>,
    // dist: &mut HashMap<State, i32>,
    // prev: &mut HashMap<State, Vec<State>>,
    dijkstra_state
        .dist
        .entry(next_state)
        .and_modify(|prev_cost| {
            if *prev_cost <= cost {
                dijkstra_state.todo.push((cost, next_state));
                if *prev_cost == cost {
                    let v = dijkstra_state
                        .prev
                        .get_mut(&next_state)
                        .expect("Must be present");
                    v.push(node);
                    // v.sort();
                    v.dedup();
                } else {
                    dijkstra_state.prev.insert(next_state, vec![node]);
                }
                *prev_cost = cost;
            }
        })
        .or_insert_with(|| {
            dijkstra_state.todo.push((cost, next_state));
            dijkstra_state.prev.insert(next_state, vec![node]);
            cost
        });
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
