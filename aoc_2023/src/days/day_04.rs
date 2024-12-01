use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Card {
    game_id: u64,
    winning_numbers: Vec<u64>,
    numbers: Vec<u64>,
}

impl Card {
    fn points(&self) -> u64 {
        match self.n_winning_numbers() {
            0 => 0,
            x => 2_u64.pow((x - 1).try_into().unwrap()),
        }
    }

    fn n_winning_numbers(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u64
    }
}

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(\d+)").unwrap();
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((game_id, numbers)) = s.split_once(':') {
            let game_id = RE_DIGITS
                .captures_iter(game_id)
                .next()
                .expect("Card id should be present")
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Could not parse card id: {}", game_id));

            if let Some((winnig_numbers, numbers)) = numbers.split_once('|') {
                let winning_numbers: Vec<u64> = RE_DIGITS
                    .captures_iter(winnig_numbers)
                    .map(|x| {
                        x.get(0)
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .unwrap_or_else(|_| {
                                panic!(
                                    "Could not parse winning numbers: {}",
                                    x.get(0).unwrap().as_str()
                                )
                            })
                    })
                    .collect();
                let numbers: Vec<u64> = RE_DIGITS
                    .captures_iter(numbers)
                    .map(|x| {
                        x.get(0)
                            .unwrap()
                            .as_str()
                            .parse::<u64>()
                            .unwrap_or_else(|_| {
                                panic!("Could not parse numbers: {}", x.get(0).unwrap().as_str())
                            })
                    })
                    .collect();
                return Ok(Card {
                    game_id,
                    winning_numbers,
                    numbers,
                });
            }
        }
        todo!()
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        Card::from_str(value).unwrap()
    }
}

pub fn solve_day(input: &str) -> (u64, u64) {
    let cards = parse_input(input);
    (part_a(&cards), part_b(&cards))
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .trim()
        .split('\n')
        .map(|card| Card::from_str(card).unwrap())
        .collect::<Vec<Card>>()
}

fn part_a(cards: &[Card]) -> u64 {
    cards.iter().map(|card| card.points()).sum()
}

fn part_b(cards: &Vec<Card>) -> u64 {
    let mut weights = vec![1; cards.len()];

    for i in 0..cards.len() {
        let weight = weights[i];
        let n_wins = cards[i].n_winning_numbers() as usize;
        for next_card_weight in weights
            .iter_mut()
            .take((i + n_wins).min(cards.len() - 1) + 1)
            .skip(i + 1)
        {
            *next_card_weight += weight;
        }
    }
    weights.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", Card {game_id: 1, winning_numbers: [41,48,83,86,17].into(), numbers: [83,86,6,31,17,9,48,53].into()})]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", Card {game_id: 2, winning_numbers: [13, 32, 20, 16, 61].into(), numbers: [61, 30, 68, 82, 17, 32, 24, 19].into()})]
    fn test_parse_input(#[case] input: &str, #[case] expected: Card) {
        assert_eq!(Card::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into(), 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".into(), 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".into(), 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".into(), 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".into(), 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".into(), 0)]
    fn test_points(#[case] card: Card, #[case] expected: u64) {
        assert_eq!(card.points(), expected)
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        let cards = parse_input(example_input);
        assert_eq!(part_a(&cards), 13)
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        let cards = parse_input(example_input);
        assert_eq!(part_b(&cards), 30)
    }
}
