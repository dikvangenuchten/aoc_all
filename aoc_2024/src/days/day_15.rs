use std::{
    fmt::{Debug, Display, Write},
    str::FromStr,
};

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let a = part_a(input_file);
    let b = part_b(input_file);
    (a, b)
}

fn parse_input(input_file: &str) -> (MapA, Vec<Move>) {
    let (map_s, move_s) = input_file.split_once("\n\n").expect("Invalid Input");
    (
        map_s.parse().expect("Invalid input while parsing map"),
        parse_moves(move_s),
    )
}

fn part_a(input_file: &str) -> u32 {
    let (map, moves) = parse_input(input_file);
    let final_map = moves.iter().fold(map, |mut map, move_| {
        map.apply_move(move_);
        map
    });
    final_map.caculate_gps()
}

fn part_b(input_file: &str) -> u32 {
    let (map_s, move_s) = input_file.split_once("\n\n").expect("Invalid Input");
    let (map, moves): (MapB, Vec<Move>) =
        (map_s.parse().expect("Valid input"), parse_moves(move_s));
    let final_map = moves.iter().fold(map, |mut map, move_| {
        map.apply_move(move_);
        map
    });
    final_map.caculate_gps()
}

#[derive(Debug, PartialEq, Eq)]
struct MapA {
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

impl MapA {
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
            self.robot_pos = self.robot_pos.add_move(move_);
            self.set(&pos, MapPart::Empty);
        };
    }

    fn apply_move_if_possible(&mut self, coord: &Coord, move_: &Move) -> bool {
        let new_pos = coord.add_move(move_);
        match self.get(&new_pos) {
            MapPart::Robot => unreachable!("Only one robot is allowed"),
            MapPart::Box => {
                if self.apply_move_if_possible(&new_pos, move_) {
                    let old_part = self.get(coord);
                    self.set(&new_pos, *old_part);
                    true
                } else {
                    false
                }
            }
            MapPart::Empty => {
                let old_part = self.get(coord);
                self.set(&new_pos, *old_part);
                true
            }
            MapPart::Wall => false,
            _ => unreachable!(),
        }
    }

    fn get(&self, coord: &Coord) -> &MapPart {
        self.map.get(coord.y).unwrap().get(coord.x).unwrap()
    }

    fn set(&mut self, coord: &Coord, part: MapPart) {
        self.map[coord.y][coord.x] = part;
    }
}

#[derive(PartialEq, Eq)]
struct MapB {
    map: Vec<Vec<MapPart>>,
    robot_pos: Coord,
}

impl MapB {
    fn caculate_gps(&self) -> u32 {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, s)| {
                        if s == &MapPart::LeftBox {
                            y * 100 + x
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum::<usize>() as u32
    }

    fn _from_debug_str(input_file: &str) -> Self {
        let mut r_pos = (0, 0);

        let map = input_file
            .trim()
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.char_indices()
                    .map(|(x, c)| match c {
                        '#' => MapPart::Wall,
                        'O' => unreachable!(),
                        '@' => {
                            r_pos = (x, y);
                            MapPart::Robot
                        }
                        '.' => MapPart::Empty,
                        '[' => MapPart::LeftBox,
                        ']' => MapPart::RightBox,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        assert_ne!(r_pos, (0, 0));

        MapB {
            map,
            robot_pos: r_pos.into(),
        }
    }

    fn apply_move(&mut self, move_: &Move) {
        match move_ {
            Move::Up | Move::Down => self.apply_move_ver_if_possible(move_),
            Move::Left | Move::Right => self.apply_move_hor_if_possible(move_),
        }
    }

    fn apply_move_ver_if_possible(&mut self, move_: &Move) {
        let pos = self.robot_pos;
        if self.check_move_if_possible_ver(&pos, move_) {
            self.apply_move_ver(&pos, move_);
            self.robot_pos = self.robot_pos.add_move(move_);
            self.set(&pos, MapPart::Empty);
        };
    }

    fn check_move_if_possible_ver(&mut self, coord: &Coord, move_: &Move) -> bool {
        match self.get(coord) {
            MapPart::Robot => self.check_vertical_can_become_empty(coord, move_),
            MapPart::LeftBox => {
                self.check_move_if_possible_ver(coord, move_)
                    && self.check_move_if_possible_ver(&coord.add_move(&Move::Right), move_)
            }
            MapPart::RightBox => {
                self.check_move_if_possible_ver(coord, move_)
                    && self.check_move_if_possible_ver(&coord.add_move(&Move::Left), move_)
            }
            MapPart::Empty => true,
            MapPart::Wall => false,
            MapPart::Box => unreachable!(),
        }
    }

    fn check_vertical_can_become_empty(&mut self, coord: &Coord, move_: &Move) -> bool {
        let new_pos = coord.add_move(move_);
        match self.get(&new_pos) {
            MapPart::LeftBox => {
                self.check_vertical_can_become_empty(&new_pos, move_)
                    && self.check_vertical_can_become_empty(&new_pos.add_move(&Move::Right), move_)
            }
            MapPart::RightBox => {
                self.check_vertical_can_become_empty(&new_pos, move_)
                    && self.check_vertical_can_become_empty(&new_pos.add_move(&Move::Left), move_)
            }
            MapPart::Empty => true,
            MapPart::Wall => false,
            MapPart::Robot | MapPart::Box => todo!(),
        }
    }

    fn apply_move_ver(&mut self, coord: &Coord, move_: &Move) {
        let new_pos = &coord.add_move(move_);
        match self.get(coord) {
            MapPart::Robot => {
                self.apply_move_ver(new_pos, move_);
                self.set(new_pos, MapPart::Robot);
                self.set(coord, MapPart::Empty);
            }
            MapPart::LeftBox => {
                self.apply_move_ver(new_pos, move_);
                self.apply_move_ver(&new_pos.add_move(&Move::Right), move_);
                self.set(new_pos, MapPart::LeftBox);
                self.set(&new_pos.add_move(&Move::Right), MapPart::RightBox);
                self.set(coord, MapPart::Empty);
                self.set(&coord.add_move(&Move::Right), MapPart::Empty);
            }
            MapPart::RightBox => {
                self.apply_move_ver(new_pos, move_);
                self.apply_move_ver(&new_pos.add_move(&Move::Left), move_);
                self.set(new_pos, MapPart::RightBox);
                self.set(&new_pos.add_move(&Move::Left), MapPart::LeftBox);
                self.set(coord, MapPart::Empty);
                self.set(&coord.add_move(&Move::Left), MapPart::Empty);
            }
            MapPart::Empty => (),
            MapPart::Box => unreachable!("No small box allowed"),
            MapPart::Wall => unreachable!("Should not be possible"),
        }
    }

    fn apply_move_hor_if_possible(&mut self, move_: &Move) {
        let pos = self.robot_pos;
        if self.apply_move_if_possible_hor(&pos, move_) {
            self.robot_pos = self.robot_pos.add_move(move_);
            self.set(&pos, MapPart::Empty);
        };
    }

    fn apply_move_if_possible_hor(&mut self, coord: &Coord, move_: &Move) -> bool {
        // Note the caller is responsible for properly
        let new_pos = coord.add_move(move_);
        match self.get(&new_pos) {
            MapPart::Robot => unreachable!("Only one robot is allowed"),
            MapPart::Box => unreachable!("No small boxes allowed"),
            MapPart::Empty => {
                let old_part = self.get(coord);
                self.set(&new_pos, *old_part);
                true
            }
            MapPart::Wall => false,
            MapPart::LeftBox | MapPart::RightBox => {
                if self.apply_move_if_possible_hor(&new_pos, move_) {
                    let old_part = self.get(coord);
                    self.set(&new_pos, *old_part);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get(&self, coord: &Coord) -> &MapPart {
        self.map.get(coord.y).unwrap().get(coord.x).unwrap()
    }

    fn set(&mut self, coord: &Coord, part: MapPart) {
        self.map[coord.y][coord.x] = part;
    }
}

impl FromStr for MapB {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut r_pos = (0, 0);

        let map = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.char_indices()
                    .flat_map(|(x, c)| match c {
                        '#' => vec![MapPart::Wall, MapPart::Wall],
                        'O' => vec![MapPart::LeftBox, MapPart::RightBox],
                        '@' => {
                            r_pos = (2 * x, y);
                            vec![MapPart::Robot, MapPart::Empty]
                        }
                        '.' => vec![MapPart::Empty, MapPart::Empty],
                        c => unreachable!("Encountered unknown value: {c}"),
                    })
                    .collect()
            })
            .collect();

        assert_ne!(r_pos, (0, 0));

        Ok(MapB {
            map,
            robot_pos: r_pos.into(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapPart {
    Robot,
    Box,
    LeftBox,
    RightBox,
    Empty,
    Wall,
}

impl MapPart {
    fn as_char(&self) -> char {
        match self {
            MapPart::Robot => '@',
            MapPart::Box => 'O',
            MapPart::LeftBox => '[',
            MapPart::RightBox => ']',
            MapPart::Empty => '.',
            MapPart::Wall => '#',
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for MapA {
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

        Ok(MapA {
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

impl Debug for MapB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())?;
        Ok(())
    }
}

impl Display for MapB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        self.map.iter().try_fold(f, |f, row| {
            let f = row.iter().try_fold(f, |f, part| {
                f.write_char(part.as_char())?;
                Ok(f)
            })?;
            f.write_char('\n')?;
            Ok(f)
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########", MapA { map: 
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
    fn test_parse(#[case] input: &str, #[case] map: MapA) {
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
    fn test_apply_move(#[case] mut map: MapA, #[case] move_: Move, #[case] map_after: MapA) {
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
"
    )]
    fn test_part_a(#[case] input_file: &str) {
        let out = part_a(input_file);
        assert_eq!(out, 10092)
    }

    #[rstest]
    #[case("#######\n#@..O..\n#......", 104)]
    #[case(
        "##########\n#.O.O.OOO#\n#........#\n#OO......#\n#OO@.....#\n#O#.....O#\n#O.....OO#\n#O.....OO#\n#OO....OO#\n##########",
        10092
    )]
    fn test_calculate_gps(#[case] map: MapA, #[case] sum: u32) {
        assert_eq!(map.caculate_gps(), sum)
    }

    #[rstest]
    #[case("########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########", MapB { map: 
        vec![
            vec![MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Robot,MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::LeftBox, MapPart::RightBox,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Empty, MapPart::Empty,MapPart::Wall, MapPart::Wall,],
            vec![MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,MapPart::Wall, MapPart::Wall,],
        ], robot_pos: (4, 2).into()
        })]
    fn test_parse_map_b(#[case] input_file: &str, #[case] map: MapB) {
        assert_eq!(input_file.parse(), Ok(map))
    }

    #[rstest]
    #[case(
        "##############\n##......##..##\n##..........##\n##....[][]@.##\n##....[]....##\n##..........##\n##############",
        Move::Left,
        "##############\n##......##..##\n##..........##\n##...[][]@..##\n##....[]....##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##..........##\n##...[][]@..##\n##....[]....##\n##..........##\n##############",
        Move::Down,
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[].@..##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[].@..##\n##..........##\n##############",
        Move::Down,
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##.......@..##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##.......@..##\n##############",
        Move::Left,
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##......@...##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##......@...##\n##############",
        Move::Left,
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##.....@....##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##..........##\n##...[][]...##\n##....[]....##\n##.....@....##\n##############",
        Move::Up,
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##.....@....##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##.....@....##\n##..........##\n##############",
        Move::Up,
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##.....@....##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##.....@....##\n##..........##\n##############",
        Move::Left,
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##....@.....##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##....@.....##\n##..........##\n##############",
        Move::Left,
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##...@......##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##...[][]...##\n##....[]....##\n##...@......##\n##..........##\n##############",
        Move::Up,
        "##############\n##......##..##\n##...[][]...##\n##...@[]....##\n##..........##\n##..........##\n##############"
    )]
    #[case(
        "##############\n##......##..##\n##...[][]...##\n##...@[]....##\n##..........##\n##..........##\n##############",
        Move::Up,
        "##############\n##...[].##..##\n##...@.[]...##\n##....[]....##\n##..........##\n##..........##\n##############"
    )]
    fn test_apply_move_b(#[case] init_map: &str, #[case] move_: Move, #[case] map_after: &str) {
        dbg!(&move_);
        let mut init_map = MapB::_from_debug_str(init_map);
        let map_after = MapB::_from_debug_str(map_after);
        init_map.apply_move(&move_);

        assert_eq!(init_map, map_after)
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
"
    )]
    fn test_part_b(#[case] input_file: &str) {
        let out = part_b(input_file);
        assert_eq!(out, 9021)
    }
}
