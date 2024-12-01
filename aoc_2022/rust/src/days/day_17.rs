use std::iter::repeat;
use std::{str::FromStr, string::ParseError};

pub fn solve(input: &str) -> (u32, u32) {
    let jets = parse_input(input);
    let part_1 = solve_part_1(&jets);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn solve_part_1(jets: &Vec<Jet>) -> u32 {
    simulation(jets, 2022)
}

fn solve_part_2(input_str: &str) -> u32 {
    0
}

fn parse_input(input_str: &str) -> Vec<Jet> {
    input_str
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn simulation(jets: &Vec<Jet>, num_rounds: usize) -> u32 {
    let rocks = vec![Rock::Plus, Rock::Minus, Rock::L, Rock::Stick, Rock::Block];
    let jets_iter = jets.iter().cycle();
    let mut rows = Vec::new();
    for rock in rocks.iter().cycle().take(num_rounds) {
        println!("{:?}", rock);
        rows.push(rock);
    }

    // Loop:
    // Start falling
    // Jet push
    // Rock drops 1
    // Jet push
    0
}

// Rock order
// minus-sign
// plus-sign
// L-shape
// i-shape
// block

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Plus,
    Minus,
    L,
    Stick,
    Block,
}

impl Rock {
    fn as_offsets(&self) -> Vec<(u32, u32)> {
        match self {
            Rock::Plus => vec![(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Rock::Minus => vec![(0, 1), (0, 2), (0, 3), (0, 4)],
            Rock::L => vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)],
            Rock::Stick => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Block => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("<>", vec![Jet::Left, Jet::Right])]
    #[case(">>", vec![Jet::Right, Jet::Right])]
    #[case("<<", vec![Jet::Left, Jet::Left])]
    fn test_parse_input(#[case] example_input_str: &str, #[case] expected: Vec<Jet>) {
        assert_eq!(parse_input(example_input_str), expected)
    }

    #[rstest]
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1, 1)] // Minus
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2, 4)] // Plus
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 3, 6)] // L
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 4, 7)] // I
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 5, 9)] // Block
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 6, 10)] // Minus
    #[case(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022, 3068)] // Given example
    fn test_simulation(
        #[case] jets_str: &str,
        #[case] rounds: usize,
        #[case] expected_height: u32,
    ) {
        let jets = parse_input(jets_str);
        assert_eq!(simulation(&jets, rounds), expected_height)
    }
}
