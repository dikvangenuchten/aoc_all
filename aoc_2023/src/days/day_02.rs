use anyhow::Result;
use lazy_static::lazy_static;
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: u64,
    revealed: Vec<Set>,
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Set {
    r: u64,
    g: u64,
    b: u64,
}

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

impl Game {
    fn is_possible(&self) -> bool {
        self.revealed.iter().all(|s| s.is_possible())
    }

    fn power(&self) -> u64 {
        self.revealed
            .iter()
            .fold(Set::default(), |s, other| s.merge(other))
            .power()
    }
}

impl Set {
    fn is_possible(&self) -> bool {
        if self.r > 12 {
            return false;
        }
        if self.g > 13 {
            return false;
        }
        if self.b > 14 {
            return false;
        }
        true
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }

    fn power(&self) -> u64 {
        self.r * self.g * self.b
    }
}

lazy_static! {
    static ref RE_GAME_ID: Regex = Regex::new(r"Game (\d+):").expect("regex should be valid");
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let game_id = RE_GAME_ID
            .captures(s)
            .unwrap_or_else(|| panic!("Input did not contain game id: {s}"))
            .get(1)
            .unwrap_or_else(|| panic!("Input could not match game id: {s}"))
            .as_str()
            .parse()
            .unwrap_or_else(|_| panic!("Could not parse game id: {s}"));

        let sets = s.split(';').map(Set::from).collect();
        Ok(Game {
            id: game_id,
            revealed: sets,
        })
    }
}

lazy_static! {
    static ref RE_RED: Regex = Regex::new(r"(\d+) red").expect("regex should be valid");
}
lazy_static! {
    static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").expect("regex should be valid");
}
lazy_static! {
    static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").expect("regex should be valid");
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let mut r = 0;
        if let Some(match_) = RE_RED.captures(value) {
            r = match_.get(1).unwrap().as_str().parse().unwrap();
        }
        let mut g: u64 = 0;
        if let Some(match_) = RE_GREEN.captures(value) {
            g = match_.get(1).unwrap().as_str().parse().unwrap();
        }
        let mut b = 0;
        if let Some(match_) = RE_BLUE.captures(value) {
            b = match_.get(1).unwrap().as_str().parse().unwrap();
        }

        Self { r, g, b }
    }
}

fn part_a(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|l| Game::from_str(l).unwrap())
        .filter(|g| g.is_possible())
        .map(|g| g.id)
        .sum()
}

fn part_b(input: &str) -> u64 {
    input
        .trim()
        .split('\n')
        .map(|l| Game::from_str(l).unwrap().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {id: 1, revealed: vec![Set{r: 4, g: 0, b: 3}, Set{r: 1, g: 2, b: 6}, Set{r: 0, g:2, b:0}]})]
    fn parse_line(#[case] line: &str, #[case] expected: Game) {
        assert_eq!(Game::from_str(line).unwrap(), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_game_possible(#[case] game: Game, #[case] is_possible: bool) {
        assert_eq!(game.is_possible(), is_possible);
    }

    #[fixture]
    fn test_input() -> &'static str {
        return "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    }

    #[rstest]
    fn test_part_a(test_input: &str) {
        assert_eq!(part_a(test_input), 8)
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_power(#[case] game: Game, #[case] power: u64) {
        assert_eq!(game.power(), power);
    }

    #[rstest]
    fn test_part_b(test_input: &str) {
        assert_eq!(part_b(test_input), 2286)
    }
}
