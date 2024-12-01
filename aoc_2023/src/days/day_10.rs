use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::{Display, Write},
    str::FromStr,
    usize,
};

pub fn solve_day(input: &str) -> (u64, u64) {
    let network = parse_input(input);
    (part_a(&network), part_b(&network))
}

#[derive(Debug, PartialEq, Eq)]
struct PipeNetwork {
    nodes: Vec<Vec<PipeShape>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Corners {
    TL,
    TR,
    BL,
    BR,
}

impl Corners {
    fn as_offset(&self) -> (i32, i32) {
        match self {
            Corners::TL => (0, 0),
            Corners::TR => (0, -1),
            Corners::BL => (-1, 0),
            Corners::BR => (-1, -1),
        }
    }

    fn relative_to(&self, other: &Self) -> (i32, i32) {
        (
            other._relative_coord().0 - self._relative_coord().0,
            other._relative_coord().1 - self._relative_coord().1,
        )
    }

    fn _relative_coord(&self) -> (i32, i32) {
        match self {
            Corners::TL => (0, 0),
            Corners::TR => (0, 1),
            Corners::BL => (1, 0),
            Corners::BR => (1, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Loc {
    Inside,
    Outside,
}

#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PipeNetwork {
    fn get_start_pos(&self) -> (usize, usize) {
        for (i, row) in self.nodes.iter().enumerate() {
            for (j, pipe) in row.iter().enumerate() {
                if pipe == &PipeShape::Start {
                    return (i, j);
                }
            }
        }
        unreachable!()
    }

    fn create_dist_map(&self, start: (usize, usize)) -> Vec<Vec<u64>> {
        let mut dist = vec![vec![u64::MAX; self.nodes[0].len()]; self.nodes.len()];
        dist[start.0][start.1] = 0;

        let mut heap = BinaryHeap::new();

        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > dist[position.0][position.1] {
                continue;
            }

            for connected in self.get_connected(position) {
                let next = State {
                    cost: (cost + 1),
                    position: connected,
                };

                if next.cost < dist[connected.0][connected.1] {
                    dist[connected.0][connected.1] = next.cost;
                    heap.push(next);
                }
            }
        }
        dist
    }

    fn get_dist_map(&self) -> Vec<Vec<u64>> {
        let start = self.get_start_pos();
        self.create_dist_map(start)
    }

    fn dijkstra(&self) -> u64 {
        let dist = self.get_dist_map();

        // Find largest reachable place
        *dist
            .iter()
            .flatten()
            .filter(|x| x != &&u64::MAX)
            .max()
            .unwrap()
    }

    fn get_connected(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let shape = self.nodes[position.0][position.1];
        let max_x = self.nodes.len();
        let max_y = self.nodes[0].len();

        shape
            .offsets()
            .iter()
            .filter_map(|direction| {
                let pos = add_offset(position, &direction.as_offset(), &(max_x, max_y))?;
                if self.nodes[pos.0][pos.1].is_connectable_from(direction) {
                    return Some(pos);
                }
                None
            })
            .collect()
    }

    fn get_connected_corners(
        loop_nodes: &Vec<Vec<PipeShape>>,
        corner_pos: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut corners = Vec::with_capacity(8);
        for as_corner in [Corners::TL, Corners::TR, Corners::BL, Corners::BR] {
            if let Some(segment_pos) = add_offset(
                corner_pos,
                &as_corner.as_offset(),
                &(loop_nodes.len(), loop_nodes[0].len()),
            ) {
                let shape = loop_nodes[segment_pos.0][segment_pos.1];
                for reachable in shape
                    .reachable_corners_from_corner(&as_corner)
                    .iter()
                    .filter(|c| c != &&as_corner)
                {
                    if let Some(pos) = add_offset(
                        corner_pos,
                        &as_corner.relative_to(reachable),
                        &(loop_nodes.len() + 1, loop_nodes[0].len() + 1),
                    ) {
                        corners.push(pos);
                    }
                }
            }
        }
        corners.sort();
        corners.dedup();
        corners
    }

    fn get_loop_nodes(&self, dist: Vec<Vec<u64>>) -> Vec<Vec<PipeShape>> {
        self.nodes
            .iter()
            .zip(dist)
            .map(|(row, dist_row)| {
                row.iter()
                    .zip(dist_row)
                    .map(|(pipe, dist)| {
                        if dist == u64::MAX {
                            PipeShape::Ground
                        } else {
                            *pipe
                        }
                    })
                    .collect()
            })
            .collect()
    }

    fn calculate_outside(&self) -> u64 {
        let dist: Vec<Vec<u64>> = self.get_dist_map();
        let loop_nodes: Vec<Vec<PipeShape>> = self.get_loop_nodes(dist);

        let x_corner_max = self.nodes.len() + 1;
        let y_corner_max = self.nodes[0].len() + 1;
        let mut corners = vec![vec![Loc::Inside; y_corner_max]; x_corner_max];

        corners[0][0] = Loc::Outside;
        let mut to_be_visited = BinaryHeap::from([(0, 0)]);

        let mut visited = HashSet::new();

        while let Some(position) = to_be_visited.pop() {
            for connected_corner in Self::get_connected_corners(&loop_nodes, position) {
                if !visited.contains(&connected_corner) {
                    visited.insert(connected_corner);
                    to_be_visited.push(connected_corner);
                    corners[connected_corner.0][connected_corner.1] = Loc::Outside;
                }
            }
        }

        let mut sum = 0;
        for i in 0..loop_nodes.len() {
            for j in 0..loop_nodes[0].len() {
                if corners[i][j] == Loc::Inside && loop_nodes[i][j] == PipeShape::Ground {
                    sum += 1;
                }
            }
        }

        sum
    }
}

fn add_offset(
    position: (usize, usize),
    (x, y): &(i32, i32),
    (max_x, max_y): &(usize, usize),
) -> Option<(usize, usize)> {
    let p = (
        usize::try_from(isize::try_from(position.0).ok()? + *x as isize).ok()?,
        usize::try_from(isize::try_from(position.1).ok()? + *y as isize).ok()?,
    );
    if &p.0 < max_x && &p.1 < max_y {
        return Some(p);
    }
    None
}

impl FromStr for PipeNetwork {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<PipeNetwork, Self::Err> {
        let nodes = input
            .trim()
            .split('\n')
            .map(|line| line.chars().map(PipeShape::from).collect())
            .collect();
        Ok(PipeNetwork { nodes })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn complement(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PipeShape {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeShape {
    fn as_char(&self) -> char {
        match self {
            PipeShape::NorthSouth => '|',
            PipeShape::EastWest => '-',
            PipeShape::NorthEast => 'L',
            PipeShape::NorthWest => 'J',
            PipeShape::SouthWest => '7',
            PipeShape::SouthEast => 'F',
            PipeShape::Start => 'S',
            PipeShape::Ground => '.',
        }
    }

    fn offsets(&self) -> Vec<Direction> {
        match self {
            PipeShape::NorthSouth => {
                vec![Direction::North, Direction::South]
            }
            PipeShape::EastWest => vec![Direction::East, Direction::West],
            PipeShape::NorthEast => vec![Direction::North, Direction::East],
            PipeShape::NorthWest => vec![Direction::North, Direction::West],
            PipeShape::SouthWest => vec![Direction::South, Direction::West],
            PipeShape::SouthEast => vec![Direction::South, Direction::East],
            PipeShape::Start => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            PipeShape::Ground => vec![],
        }
    }

    fn is_connectable_from(&self, dir: &Direction) -> bool {
        self.offsets().contains(&dir.complement())
    }

    fn reachable_corners_from_corner(&self, corner: &Corners) -> Vec<Corners> {
        match corner {
            Corners::TL => match self {
                PipeShape::NorthSouth => vec![Corners::BL, Corners::TL],
                PipeShape::EastWest => vec![Corners::TR, Corners::TL],
                PipeShape::NorthEast => vec![Corners::BL, Corners::BR, Corners::TL],
                PipeShape::NorthWest => vec![Corners::TL],
                PipeShape::SouthWest => vec![Corners::TR, Corners::BR, Corners::TL],
                PipeShape::SouthEast => vec![Corners::BL, Corners::TR, Corners::TL],
                PipeShape::Ground => vec![Corners::TR, Corners::BR, Corners::BL, Corners::TL],
                PipeShape::Start => vec![Corners::TL],
            },
            Corners::TR => match self {
                PipeShape::NorthSouth => vec![Corners::TR, Corners::BR],
                PipeShape::EastWest => vec![Corners::TR, Corners::TL],
                PipeShape::NorthEast => vec![Corners::TR],
                PipeShape::NorthWest => vec![Corners::TR, Corners::BR, Corners::BL],
                PipeShape::SouthWest => vec![Corners::TR, Corners::TL, Corners::BR],
                PipeShape::SouthEast => vec![Corners::TR, Corners::TL, Corners::BL],
                PipeShape::Ground => vec![Corners::TR, Corners::TL, Corners::BR, Corners::BL],
                PipeShape::Start => vec![Corners::TR],
            },
            Corners::BL => match self {
                PipeShape::NorthSouth => vec![Corners::BL, Corners::TL],
                PipeShape::EastWest => vec![Corners::BL, Corners::BR],
                PipeShape::NorthEast => vec![Corners::BL, Corners::TL, Corners::BR],
                PipeShape::NorthWest => vec![Corners::BL, Corners::BR, Corners::TR],
                PipeShape::SouthWest => vec![Corners::BL],
                PipeShape::SouthEast => vec![Corners::BL, Corners::TL, Corners::TR],
                PipeShape::Ground => vec![Corners::BL, Corners::TL, Corners::BR, Corners::TR],
                PipeShape::Start => vec![Corners::BL],
            },
            Corners::BR => match self {
                PipeShape::NorthSouth => vec![Corners::BR, Corners::TR],
                PipeShape::EastWest => vec![Corners::BR, Corners::BL],
                PipeShape::NorthEast => vec![Corners::BR, Corners::TL, Corners::BL],
                PipeShape::NorthWest => vec![Corners::BR, Corners::TR, Corners::BL],
                PipeShape::SouthWest => vec![Corners::BR, Corners::TL, Corners::TR],
                PipeShape::SouthEast => vec![Corners::BR],
                PipeShape::Ground => vec![Corners::BR, Corners::TL, Corners::BL, Corners::TR],
                PipeShape::Start => vec![Corners::BR],
            },
        }
    }
}

impl Display for PipeShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl Display for PipeNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.nodes {
            for pipe in row {
                f.write_char(pipe.as_char())?
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl From<char> for PipeShape {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeShape::NorthSouth,
            '-' => PipeShape::EastWest,
            'L' => PipeShape::NorthEast,
            'J' => PipeShape::NorthWest,
            '7' => PipeShape::SouthWest,
            'F' => PipeShape::SouthEast,
            'S' => PipeShape::Start,
            '.' => PipeShape::Ground,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> PipeNetwork {
    PipeNetwork::from_str(input.trim()).unwrap()
}

fn part_a(network: &PipeNetwork) -> u64 {
    network.dijkstra()
}

fn part_b(network: &PipeNetwork) -> u64 {
    network.calculate_outside()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return ".....\n.S-7.\n.|.|.\n.L-J.\n.....\n";
    }

    #[fixture]
    fn example_network(example_input: &str) -> PipeNetwork {
        return parse_input(example_input);
    }

    #[rstest]
    #[case("|", PipeShape::NorthSouth)]
    #[case('-', PipeShape::EastWest)]
    #[case('L', PipeShape::NorthEast)]
    #[case('J', PipeShape::NorthWest)]
    #[case('7', PipeShape::SouthWest)]
    #[case('F', PipeShape::SouthEast)]
    #[case('S', PipeShape::Start)]
    #[case('.', PipeShape::Ground)]
    fn test_parse_pipeshape(#[case] letter: char, #[case] expected: PipeShape) {
        assert_eq!(PipeShape::from(letter), expected)
    }

    #[rstest]
    fn test_parse_input(example_input: &str) {
        let pipe = parse_input(example_input);
        assert_eq!(
            pipe,
            PipeNetwork {
                nodes: vec![
                    vec![PipeShape::Ground; 5],
                    vec![
                        PipeShape::Ground,
                        PipeShape::Start,
                        PipeShape::EastWest,
                        PipeShape::SouthWest,
                        PipeShape::Ground
                    ],
                    vec![
                        PipeShape::Ground,
                        PipeShape::NorthSouth,
                        PipeShape::Ground,
                        PipeShape::NorthSouth,
                        PipeShape::Ground,
                    ],
                    vec![
                        PipeShape::Ground,
                        PipeShape::NorthEast,
                        PipeShape::EastWest,
                        PipeShape::NorthWest,
                        PipeShape::Ground,
                    ],
                    vec![PipeShape::Ground; 5],
                ]
            }
        );
        assert_eq!(format!("{}", pipe), example_input)
    }

    #[rstest]
    fn test_find_start_pos(example_network: PipeNetwork) {
        assert_eq!(example_network.get_start_pos(), (1, 1))
    }

    #[rstest]
    #[case((0, 0), vec![])]
    #[case((1, 2), vec![(1,1), (1,3)])]
    #[case((1, 3), vec![(1,2), (2,3)])]
    #[case((2, 3), vec![(1,3), (3,3)])]
    #[case((3, 3), vec![(2,3), (3,2)])]
    #[case((3, 2), vec![(3,3), (3,1)])]
    #[case((3, 1), vec![(3,2), (2,1)])]
    #[case((2, 1), vec![(3,1), (1, 1)])]
    fn test_get_connected(
        example_network: PipeNetwork,
        #[case] position: (usize, usize),
        #[case] mut expected: Vec<(usize, usize)>,
    ) {
        let mut connected = example_network.get_connected(position);
        connected.sort();
        expected.sort();
        assert_eq!(connected, expected);
    }

    #[rstest]
    fn test_dijkstra(example_network: PipeNetwork) {
        assert_eq!(example_network.dijkstra(), 4)
    }

    #[rstest]
    fn test_part_a(example_network: PipeNetwork) {
        assert_eq!(part_a(&example_network), 4)
    }

    #[rstest]
    #[case("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...", 8)]
    fn test_part_a_more(#[case] maze: &str, #[case] expected: u64) {
        assert_eq!(part_a(&parse_input(maze)), expected)
    }

    #[rstest]
    #[case("..\n..", vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (2, 1), (2, 2), (1, 2)])]
    #[case("F7\nLJ", vec![])]
    #[case("F7\n||", vec![(2, 1)])]
    #[case("||\nLJ", vec![(0, 1)])]
    #[case("--\n--", vec![(1, 0), (1, 2)])]
    fn test_connected_corners(
        #[case] network: &str,
        #[case] mut connected_corners: Vec<(usize, usize)>,
    ) {
        let loop_nodes = network
            .trim()
            .split('\n')
            .map(|line| line.chars().map(PipeShape::from).collect())
            .collect();

        let mut connected = PipeNetwork::get_connected_corners(&loop_nodes, (1, 1));
        connected.sort();
        connected_corners.sort();
        assert_eq!(connected, connected_corners)
    }

    #[rstest]
    #[case(".....\n.S-7.\n.|.|.\n.L-J.\n.....\n", 1)]
    #[case("..........\n.S------7.\n.|F----7|.\n.||....||.\n.||....||.\n.|L-7F-J|.\n.|..||..|.\n.L--JL--J.\n..........", 4)]
    fn test_part_b(#[case] network: PipeNetwork, #[case] expected: u64) {
        assert_eq!(part_b(&network), expected)
    }

    #[rstest]
    #[case(PipeShape::NorthSouth)]
    #[case(PipeShape::EastWest)]
    #[case(PipeShape::NorthEast)]
    #[case(PipeShape::NorthWest)]
    #[case(PipeShape::SouthWest)]
    #[case(PipeShape::SouthEast)]
    #[case(PipeShape::Ground)]
    #[case(PipeShape::Start)]
    fn assert_consitency_shapes(#[case] shape: PipeShape) {
        for corner in [Corners::TL, Corners::BL, Corners::TR, Corners::BR] {
            println!("{shape:?} ({shape}) from {corner:?}");
            let mut reachable = shape.reachable_corners_from_corner(&corner);
            assert!(reachable.contains(&corner));
            println!("reachable: {:?}", reachable);
            let mut indirect_reachable: Vec<Corners> = reachable
                .iter()
                .flat_map(|c| shape.reachable_corners_from_corner(c))
                .collect();
            println!("indirect: {:?}", indirect_reachable);
            // Sort and dedup to make them equal
            indirect_reachable.extend(&reachable);
            indirect_reachable.sort();
            indirect_reachable.dedup();
            reachable.sort();

            assert_eq!(reachable, indirect_reachable)
        }
    }
}
