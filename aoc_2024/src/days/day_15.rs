use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let (mut map, moves) = parse_input(input_file);
    let a = part_a(&mut map, &moves);
    let b = part_b(input_file);
    (a, b)
}

fn parse_input(input_file: &str) -> (Map, Vec<Move>) {
    let (map_s, move_s) = input_file.split_once("\n\n").expect("Invalid Input");
    (
        map_s.parse().expect("Invalid input while parsing map"),
        parse_moves(move_s),
    )
}

fn part_a(map: &mut Map, moves: &[Move]) -> u32 {
    let final_map = moves.iter().fold(map, |map, move_| {
        map.apply_move(move_);
        map
    });
    final_map.caculate_gps()
}

fn part_b(input_file: &str) -> u32 {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    map: Vec<Vec<MapPart>>,
    robot_pos: Coord,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    fn add_move(&self, move_: &Move) -> Self {
        // Due to the wall we do not need to take into account over/underflow
        match move_ {
            Move::Up => (self.x, self.y - 1).into(),
            Move::Left => (self.x - 1, self.y).into(),
            Move::Down => (self.x, self.y + 1).into(),
            Move::Right => (self.x + 1, self.y).into(),
        }
    }
}

impl Map {
    fn caculate_gps(&self) -> u32 {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, s)| if s == &MapPart::Box { y * 100 + x } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32
    }

    fn apply_move(&mut self, move_: &Move) {
        let pos = self.robot_pos;
        if self.apply_move_if_possible(&pos, move_) {
            self.robot_pos = self.robot_pos.add_move(move_)
        };
    }

    fn apply_move_if_possible(&mut self, coord: &Coord, move_: &Move) -> bool {
        // Note the caller is responsible for properly
        let new_pos = coord.add_move(move_);
        match self.get(&new_pos) {
            MapPart::Robot => unreachable!("Only one robot is allowed"),
            MapPart::Box => {
                if self.apply_move_if_possible(&new_pos, move_) {
                    let old_part = self.get(coord);
                    self.set(&new_pos, *old_part);
                    self.set(coord, MapPart::Empty);
                    true
                } else {
                    false
                }
            }
            MapPart::Empty => {
                let old_part = self.get(coord);
                self.set(&new_pos, *old_part);
                self.set(coord, MapPart::Empty);
                true
            }
            MapPart::Wall => false,
        }
    }

    fn get(&self, coord: &Coord) -> &MapPart {
        self.map.get(coord.y).unwrap().get(coord.x).unwrap()
    }

    fn set(&mut self, coord: &Coord, part: MapPart) {
        self.map[coord.y][coord.x] = part;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapPart {
    Robot,
    Box,
    Empty,
    Wall,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r_pos = (0, 0);

        let map = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.char_indices()
                    .map(|(x, c)| match c {
                        '#' => MapPart::Wall,
                        'O' => MapPart::Box,
                        '@' => {
                            r_pos = (x, y);
                            MapPart::Robot
                        }
                        '.' => MapPart::Empty,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        assert_ne!(r_pos, (0, 0));

        Ok(Map {
            map,
            robot_pos: r_pos.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Up,
    Left,
    Down,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '^' => Move::Up,
            '<' => Move::Left,
            'v' => Move::Down,
            '>' => Move::Right,
            _ => unreachable!(),
        }
    }
}

fn parse_moves(input_file: &str) -> Vec<Move> {
    input_file
        .chars()
        .filter(|c| c != &'\n')
        .map(|m| m.into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########", Map { map: 
    vec![
        vec![MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Empty,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Wall,MapPart::Robot,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Empty,MapPart::Wall,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Box,MapPart::Empty,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Empty,MapPart::Wall,],
        vec![MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,MapPart::Wall,],
    ], robot_pos: (2, 2).into()
    })]
    fn test_parse(#[case] input: &str, #[case] map: Map) {
        assert_eq!(input.parse(), Ok(map))
    }

    #[rstest]
    #[case("^", vec![Move::Up])]
    #[case("^>v<", vec![Move::Up, Move::Right, Move::Down, Move::Left])]
    #[case("^>v<\n", vec![Move::Up, Move::Right, Move::Down, Move::Left])]
    fn test_parse_moves(#[case] moves: &str, #[case] expected_moves: Vec<Move>) {
        assert_eq!(parse_moves(moves), expected_moves)
    }

    #[rstest]
    #[case(
        "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Up,
        "########\n#.@O.O.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Left,
        "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Up,
        "########\n#.@O.O.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#.@O.O.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Up,
        "########\n#.@O.O.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#.@O.O.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Right,
        "########\n#..@OO.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#..@OO.#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Right,
        "########\n#...@OO#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#...@OO#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Right,
        "########\n#...@OO#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########"
    )]
    #[case(
        "########\n#...@OO#\n##..O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########",
        Move::Down,
        "########\n#....OO#\n##..@..#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##..@..#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Down,
        "########\n#....OO#\n##..@..#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##..@..#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Left,
        "########\n#....OO#\n##.@...#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.@...#\n#...O..#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Down,
        "########\n#....OO#\n##.....#\n#..@O..#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.....#\n#..@O..#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Right,
        "########\n#....OO#\n##.....#\n#...@O.#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.....#\n#...@O.#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Right,
        "########\n#....OO#\n##.....#\n#....@O#\n#.#.O..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.....#\n#....@O#\n#.#.O..#\n#...O..#\n#...O..#\n########",
        Move::Down,
        "########\n#....OO#\n##.....#\n#.....O#\n#.#.O@.#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.....#\n#.....O#\n#.#.O@.#\n#...O..#\n#...O..#\n########",
        Move::Left,
        "########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########"
    )]
    #[case(
        "########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########",
        Move::Left,
        "########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########"
    )]
    fn test_apply_move(#[case] mut map: Map, #[case] move_: Move, #[case] map_after: Map) {
        map.apply_move(&move_);
        assert_eq!(map, map_after)
    }

    #[rstest]
    #[case(
        "
##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########\n\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
", "##########\n#.O.O.OOO#\n#........#\n#OO......#\n#OO@.....#\n#O#.....O#\n#O.....OO#\n#O.....OO#\n#OO....OO#\n##########"
    )]
    fn test_part_a(#[case] input_file: &str, #[case] final_map: Map) {
        let (mut map, moves) = parse_input(input_file);
        let out = part_a(&mut map, &moves);
        for (m, fm) in map.map.iter().zip(&final_map.map) {
            assert_eq!(m, fm);
        }
        assert_eq!(map.map, final_map.map);
        assert_eq!(out, 10092)
    }

    #[rstest]
    #[case("#######\n#@..O..\n#......", 104)]
    #[case(
        "##########\n#.O.O.OOO#\n#........#\n#OO......#\n#OO@.....#\n#O#.....O#\n#O.....OO#\n#O.....OO#\n#OO....OO#\n##########",
        10092
    )]
    fn test_calculate_gps(#[case] map: Map, #[case] sum: u32) {
        assert_eq!(map.caculate_gps(), sum)
    }
}
