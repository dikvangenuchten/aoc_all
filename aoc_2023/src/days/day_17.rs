use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub fn solve_day(input: &str) -> (u64, u64) {
    let city_blocks = CityBlocks::from(input);
    (part_a(&city_blocks), part_b(&city_blocks))
}

fn part_a(city_blocks: &CityBlocks) -> u64 {
    let x = city_blocks.blocks.len() - 1;
    let y = city_blocks.blocks[0].len() - 1;
    city_blocks.a_star_part_1((0, 0), (x, y))
}

fn part_b(city_blocks: &CityBlocks) -> u64 {
    let x = city_blocks.blocks.len() - 1;
    let y = city_blocks.blocks[0].len() - 1;
    city_blocks.a_star_part_2((0, 0), (x, y))
}

fn parse_row(input: &str) -> Vec<CityBlock> {
    input.trim().chars().map(CityBlock::from).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    cost: u64,
    node: Node,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    straight_streak: u8,
    loc: (usize, usize),
    dir: Direction,
}

fn add_dir_to_loc(loc: (usize, usize), dir: Direction) -> anyhow::Result<(usize, usize)> {
    let offset = dir.as_offset();

    Ok((
        (loc.0 as isize + offset.0).try_into()?,
        (loc.1 as isize + offset.1).try_into()?,
    ))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    fn as_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CityBlock {
    heat_loss: u64,
}

impl From<char> for CityBlock {
    fn from(value: char) -> Self {
        CityBlock {
            heat_loss: value.to_digit(10).unwrap().into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CityBlocks {
    blocks: Vec<Vec<CityBlock>>,
}

impl From<&str> for CityBlocks {
    fn from(value: &str) -> Self {
        CityBlocks {
            blocks: value.trim().split('\n').map(parse_row).collect(),
        }
    }
}

impl FromStr for CityBlocks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CityBlocks::from(s))
    }
}

impl CityBlocks {
    fn a_star_part_2(&self, _start: (usize, usize), goal: (usize, usize)) -> u64 {
        let node = Node {
            dir: Direction::East,
            straight_streak: 0,
            loc: (0, 0),
        };
        let mut heap = BinaryHeap::from([State { cost: 0, node }]);
        let mut visited = HashMap::from([(node, 0)]);
        let mut came_from = HashMap::new();

        while let Some(current) = heap.pop() {
            let node = current.node;
            if node.loc == goal
                && node.dir == Direction::East
                && (node.straight_streak > 3 || node.straight_streak == 0)
            {
                // print_path(&came_from, &current);
                return current.cost;
            }

            for neighbor in self.get_neighbors_p2(current) {
                let prev_cost = visited.get(&neighbor.node).unwrap_or(&u64::MAX);
                if prev_cost > &neighbor.cost {
                    heap.push(neighbor);
                    came_from.insert(neighbor, current);
                    visited.insert(neighbor.node, neighbor.cost);
                }
            }
        }
        unreachable!()
    }

    fn get_neighbors_p2(&self, state: State) -> Vec<State> {
        let mut neighbors = Vec::with_capacity(3);
        let dir = state.node.dir;

        if state.node.straight_streak < 9 {
            if let Some(s) = self.create_state(&state, dir) {
                neighbors.push(s)
            }
        }

        if state.node.straight_streak > 2 {
            if let Some(s) = self.create_state(&state, dir.left()) {
                neighbors.push(s)
            }
            if let Some(s) = self.create_state(&state, dir.right()) {
                neighbors.push(s)
            }
        }

        neighbors
    }

    fn a_star_part_1(&self, _start: (usize, usize), goal: (usize, usize)) -> u64 {
        let node = Node {
            dir: Direction::East,
            straight_streak: 0,
            loc: (0, 0),
        };
        let mut heap = BinaryHeap::from([State { cost: 0, node }]);
        let mut visited = HashMap::from([(node, 0)]);

        while let Some(current) = heap.pop() {
            let node = current.node;
            if node.loc == goal && node.dir == Direction::East {
                return current.cost;
            }

            for neighbor in self.get_neighbors(current) {
                let prev_cost = visited.get(&neighbor.node).unwrap_or(&u64::MAX);
                if prev_cost > &neighbor.cost {
                    heap.push(neighbor);
                    visited.insert(neighbor.node, neighbor.cost);
                }
            }
        }
        unreachable!()
    }

    fn get_neighbors(&self, state: State) -> Vec<State> {
        let mut neighbors = Vec::with_capacity(3);
        let dir = state.node.dir;
        // Turn left
        if let Some(state) = self.create_state(&state, dir.left()) {
            neighbors.push(state)
        }

        // Turn right
        if let Some(s) = self.create_state(&state, dir.right()) {
            neighbors.push(s)
        }

        if state.node.straight_streak < 3 {
            // Go straight
            if let Some(s) = self.create_state(&state, dir) {
                neighbors.push(s)
            }
        }
        neighbors
    }

    fn create_state(&self, prev_state: &State, dir: Direction) -> Option<State> {
        let node = prev_state.node;
        let loc = add_dir_to_loc(node.loc, dir).ok()?;
        let cost = self.get_cost(loc)?;
        let straight_streak;
        if prev_state.node.dir == dir {
            straight_streak = prev_state.node.straight_streak;
        } else {
            straight_streak = 0;
        }

        return Some(State {
            cost: prev_state.cost + cost,
            node: Node {
                straight_streak: straight_streak + 1,
                loc,
                dir,
            },
        });
    }

    fn get_cost(&self, loc: (usize, usize)) -> Option<u64> {
        if let Some(row) = self.blocks.get(loc.0) {
            if let Some(block) = row.get(loc.1) {
                return Some(block.heat_loss);
            }
        }
        None
    }
}

fn print_path(came_from: &HashMap<State, State>, state: &State) {
    if let Some(prev_state) = came_from.get(state) {
        print_path(came_from, prev_state);
    }
    println!("{:?}", state);
}

#[cfg(test)]
mod test {
    use crate::days::read_day_input;

    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n";
    }

    #[rstest]
    #[case('1', CityBlock { heat_loss: 1})]
    #[case('2', CityBlock { heat_loss: 2})]
    #[case('3', CityBlock { heat_loss: 3})]
    #[case('4', CityBlock { heat_loss: 4})]
    fn test_parse_block(#[case] input: char, #[case] expected: CityBlock) {
        assert_eq!(CityBlock::from(input), expected)
    }

    #[fixture]
    fn example_city(example_input: &str) -> CityBlocks {
        CityBlocks::from(example_input)
    }

    #[rstest]
    #[case("1234", vec![CityBlock::from('1'), CityBlock::from('2'), CityBlock::from('3'), CityBlock::from('4')])]
    fn test_parse_row(#[case] input: &str, #[case] expected: Vec<CityBlock>) {
        assert_eq!(parse_row(input), expected)
    }

    #[rstest]
    fn test_parse_city(example_city: CityBlocks) {
        let expected = CityBlocks {
            blocks: vec![
                parse_row("2413432311323"),
                parse_row("3215453535623"),
                parse_row("3255245654254"),
                parse_row("3446585845452"),
                parse_row("4546657867536"),
                parse_row("1438598798454"),
                parse_row("4457876987766"),
                parse_row("3637877979653"),
                parse_row("4654967986887"),
                parse_row("4564679986453"),
                parse_row("1224686865563"),
                parse_row("2546548887735"),
                parse_row("4322674655533"),
            ],
        };
        assert_eq!(example_city, expected)
    }

    #[rstest]
    fn test_part_a(example_city: CityBlocks) {
        assert_eq!(part_a(&example_city), 102)
    }

    #[rstest]
    #[case("2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533\n", 94)]
    #[case(
        "111111111111\n999999999991\n999999999991\n999999999991\n999999999991",
        71
    )]

    fn test_part_b(#[case] city: CityBlocks, #[case] expected: u64) {
        assert_eq!(part_b(&city), expected)
    }

    #[rstest]
    #[case(read_day_input("day_17"), 988, 1084)]
    fn test_part_b_real(#[case] city_str: String, #[case] min_: u64, #[case] max_: u64) {
        let city = CityBlocks::from_str(&city_str).unwrap();
        let x = part_b(&city);
        assert!(x > min_);
        assert!(x < max_);
    }
}
