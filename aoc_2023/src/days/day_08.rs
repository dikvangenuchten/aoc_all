pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

#[derive(Debug, PartialEq, Eq)]

struct QuickGraph {
    nodes: Vec<QuickNode>,
}
#[derive(Debug, PartialEq, Eq)]

struct QuickNode {
    label: [char; 3],
    left_idx: usize,
    right_idx: usize,
}

impl QuickNode {
    fn get(&self, direction: &Direction) -> usize {
        match direction {
            Direction::Left => self.left_idx,
            Direction::Right => self.right_idx,
        }
    }

    fn is_label(&self, label: &str) -> bool {
        label.chars().zip(self.label).all(|(l, r)| l == r)
    }
}

impl QuickGraph {
    fn new(input: &str) -> Self {
        let nodes: Vec<Node> = input
            .trim()
            .split('\n')
            .map(Node::from)
            // .map(|n| ((&n.label).into(), n))
            .collect();
        let labels: Vec<[char; 3]> = nodes
            .iter()
            .map(|n| {
                n.label
                    .chars()
                    .take(3)
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap()
            })
            .collect();
        let left: Vec<usize> = nodes
            .iter()
            .map(|n| {
                n.left
                    .chars()
                    .take(3)
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap()
            })
            .map(|label: [char; 3]| labels.iter().position(|l| l == &label).unwrap())
            .collect();
        let right: Vec<usize> = nodes
            .iter()
            .map(|n| {
                (*n.right.chars().collect::<Vec<char>>())
                    .try_into()
                    .unwrap()
            })
            .map(|label: [char; 3]| labels.iter().position(|l| l == &label).unwrap())
            .collect();

        let nodes = labels
            .into_iter()
            .zip(left)
            .zip(right)
            .map(|((label, left_idx), right_idx)| QuickNode {
                label,
                left_idx,
                right_idx,
            })
            .collect::<Vec<QuickNode>>();

        QuickGraph { nodes }
    }

    fn follow_path(&self, start: String, end: String, path: Vec<Direction>) -> u64 {
        let mut count = 0;
        let mut cur = self.to_idx(&start);
        let end = self.to_idx(&end);
        while cur != end {
            let dir = &path[count % path.len()];
            cur = self.get(cur, dir);
            count += 1;
        }
        count as u64
    }

    fn start_nodes(&self) -> Vec<usize> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.label[2] == 'A')
            .map(|(i, _)| i)
            .collect()
    }

    fn find_loops(&self, start: usize, directions: &Vec<Direction>) -> usize {
        let mut first_visited = vec![0; self.nodes.len()];
        let mut count: usize = 0;
        let mut cur = start;
        loop {
            for direction in directions {
                cur = self.get(cur, direction);
            }
            count += 1;

            if first_visited[cur] == 0 {
                first_visited[cur] = count;
            } else {
                let path_length = count - first_visited[cur];

                return path_length;
            }
        }
    }

    fn to_idx(&self, label: &str) -> usize {
        self.nodes.iter().position(|n| n.is_label(label)).unwrap()
    }

    fn get(&self, cur: usize, dir: &Direction) -> usize {
        self.nodes[cur].get(dir)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let (label, connections) = value.split_once('=').unwrap();
        let (left, right) = connections.split_once(',').unwrap();
        Node {
            label: label.trim().into(),
            left: left.trim().strip_prefix('(').unwrap().into(),
            right: right.trim().strip_suffix(')').unwrap().into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<&char> for Direction {
    fn from(value: &char) -> Self {
        match value {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!("Invalid direction character: {value}"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Direction>, QuickGraph) {
    let (directions, nodes) = input.split_once('\n').unwrap();
    let directions = directions
        .trim()
        .chars()
        .map(|c| Direction::from(&c))
        .collect();
    let graph = QuickGraph::new(nodes);
    (directions, graph)
}

fn part_a(input: &str) -> u64 {
    let (directions, graph) = parse_input(input);
    graph.follow_path("AAA".into(), "ZZZ".into(), directions)
}

fn part_b(input: &str) -> u64 {
    let (directions, graph) = parse_input(input);

    let mut total_loop_length = directions.len();

    for n in graph.start_nodes() {
        let loop_length = graph.find_loops(n, &directions);
        total_loop_length *= loop_length;
    }

    total_loop_length as u64
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::days::read_day_input;

    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)";
    }

    #[fixture]
    fn input() -> &'static str {
        return read_day_input("day_08").leak();
    }

    #[rstest]
    #[timeout(Duration::from_secs(1))]
    fn test_part_a(example_input: &str) {
        let (directions, graph) = parse_input(example_input);
        assert_eq!(graph.follow_path("AAA".into(), "ZZZ".into(), directions), 2)
    }

    #[rstest]
    #[timeout(Duration::from_secs(1))]
    fn test_part_a_full(input: &str) {
        let (directions, graph) = parse_input(input);
        assert_eq!(
            graph.follow_path("AAA".into(), "ZZZ".into(), directions),
            14257
        )
    }

    #[rstest]
    #[timeout(Duration::from_secs(1))]
    fn test_part_a2() {
        let example_input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_a(example_input), 6);
    }

    #[rstest]
    #[timeout(Duration::from_secs(1))]
    fn test_part_b() {
        let example_input = "LR\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        assert_eq!(part_b(example_input), 6);
    }

    #[rstest]
    #[timeout(Duration::from_secs(1))]
    fn test_find_loops() {
        let example_input = "LR\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (directions, graph) = parse_input(example_input);
        assert_eq!(graph.find_loops(graph.to_idx("11A"), &directions), 1);
        assert_eq!(graph.find_loops(graph.to_idx("22A"), &directions), 3);
    }
}
