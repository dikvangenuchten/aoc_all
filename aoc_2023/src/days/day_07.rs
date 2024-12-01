use std::str::FromStr;

pub fn solve_day(input: &str) -> (u64, u64) {
    (part_a(input), part_b(input))
}

fn part_a(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input
        .trim()
        .split('\n')
        .map(|l| Hand::from_str(l).unwrap())
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank as u64 + 1) * h.bid)
        .sum()
}

fn part_b(input: &str) -> u64 {
    let mut hands: Vec<Hand> = input
        .trim()
        .split('\n')
        .map(|l| Hand::from_2(l).unwrap())
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank as u64 + 1) * h.bid)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

impl From<&[Card; 5]> for Type {
    fn from(value: &[Card; 5]) -> Self {
        let mut counts = value.iter().fold([0; 14], |mut acc, card| {
            acc[(card.val - 1) as usize] += 1;
            acc
        });
        let jokers = counts[0];
        counts[0] = 0;
        counts.sort_unstable();
        if counts[13] == (5 - jokers) {
            return Type::FiveOfaKind;
        }
        if counts[13] == (4 - jokers) {
            return Type::FourOfaKind;
        }

        if counts[13] == (3 - jokers) {
            if counts[12] == 2 {
                return Type::FullHouse;
            }
            return Type::ThreeOfaKind;
        }
        if counts[13] == (2 - jokers) {
            if counts[12] == 2 {
                return Type::TwoPair;
            }
            return Type::Pair;
        }
        Type::HighCard
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl Hand {
    fn type_(&self) -> Type {
        Type::from(&self.cards)
    }

    fn from_2(s: &str) -> anyhow::Result<Self> {
        let (cards, bid) = s.split_at(5);
        let cards: [Card; 5] = cards
            .chars()
            .map(|c| Card::from_2(&c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let bid = bid.trim().parse()?;
        Ok(Hand { cards, bid })
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (cards, bid) = s.split_at(5);
        let cards: [Card; 5] = cards
            .chars()
            .map(Card::from)
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let bid = bid.trim().parse()?;
        Ok(Hand { cards, bid })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_().cmp(&other.type_()) {
            core::cmp::Ordering::Equal => {
                match self.cards.cmp(&other.cards) {
                    core::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
                unreachable!()
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    val: u8,
}

impl Card {
    fn from_2(value: &char) -> Self {
        if value.is_ascii_digit() {
            let val = value.to_digit(10).unwrap();
            return Card { val: val as u8 };
        }
        match value {
            'T' => Card { val: 10 },
            'J' => Card { val: 1 },
            'Q' => Card { val: 11 },
            'K' => Card { val: 12 },
            'A' => Card { val: 13 },
            _ => unreachable!(),
        }
    }
}

impl From<&char> for Card {
    fn from(value: &char) -> Self {
        if value.is_ascii_digit() {
            let val = value.to_digit(10).unwrap();
            return Card { val: val as u8 };
        }
        match value {
            'T' => Card { val: 10 },
            'J' => Card { val: 11 },
            'Q' => Card { val: 12 },
            'K' => Card { val: 13 },
            'A' => Card { val: 14 },
            _ => unreachable!(),
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card::from(&value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    }

    #[rstest]
    #[case('2', Card { val: 2 })]
    #[case('3', Card { val: 3 })]
    #[case('4', Card { val: 4 })]
    #[case('5', Card { val: 5 })]
    #[case('6', Card { val: 6 })]
    #[case('7', Card { val: 7 })]
    #[case('8', Card { val: 8 })]
    #[case('9', Card { val: 9 })]
    #[case('T', Card { val: 10 })]
    #[case('J', Card { val: 11 })]
    #[case('Q', Card { val: 12 })]
    #[case('K', Card { val: 13 })]
    #[case('A', Card { val: 14 })]
    fn parse_card(#[case] input: char, #[case] expected: Card) {
        assert_eq!(Card::from(&input), expected)
    }

    #[rstest]
    #[case('J', Card { val: 1 })]
    #[case('2', Card { val: 2 })]
    #[case('3', Card { val: 3 })]
    #[case('4', Card { val: 4 })]
    #[case('5', Card { val: 5 })]
    #[case('6', Card { val: 6 })]
    #[case('7', Card { val: 7 })]
    #[case('8', Card { val: 8 })]
    #[case('9', Card { val: 9 })]
    #[case('T', Card { val: 10 })]
    #[case('Q', Card { val: 11 })]
    #[case('K', Card { val: 12 })]
    #[case('A', Card { val: 13 })]
    fn parse_card_2(#[case] input: char, #[case] expected: Card) {
        assert_eq!(Card::from_2(&input), expected)
    }

    #[rstest]
    #[case("32T3K 765", Hand {cards: [Card::from('3'), Card::from('2'), Card::from('T'), Card::from('3'), Card::from('K')], bid: 765})]
    #[case("T55J5 684", Hand {cards: [Card::from('T'), Card::from('5'), Card::from('5'), Card::from('J'), Card::from('5')], bid: 684})]
    #[case("KK677 28", Hand {cards: [Card::from('K'), Card::from('K'), Card::from('6'), Card::from('7'), Card::from('7')], bid: 28})]
    #[case("KTJJT 220", Hand {cards: [Card::from('K'), Card::from('T'), Card::from('J'), Card::from('J'), Card::from('T')], bid: 220})]
    #[case("QQQJA 483", Hand {cards: [Card::from('Q'), Card::from('Q'), Card::from('Q'), Card::from('J'), Card::from('A')], bid: 483})]
    fn parse_hand(#[case] input: &str, #[case] expected: Hand) {
        assert_eq!(Hand::from_str(input).unwrap(), expected);
    }

    #[rstest]
    #[case("32T3K 765", Hand {cards: [Card::from_2(&'3'), Card::from_2(&'2'), Card::from_2(&'T'), Card::from_2(&'3'), Card::from_2(&'K')], bid: 765})]
    #[case("T55J5 684", Hand {cards: [Card::from_2(&'T'), Card::from_2(&'5'), Card::from_2(&'5'), Card::from_2(&'J'), Card::from_2(&'5')], bid: 684})]
    #[case("KK677 28", Hand {cards: [Card::from_2(&'K'), Card::from_2(&'K'), Card::from_2(&'6'), Card::from_2(&'7'), Card::from_2(&'7')], bid: 28})]
    #[case("KTJJT 220", Hand {cards: [Card::from_2(&'K'), Card::from_2(&'T'), Card::from_2(&'J'), Card::from_2(&'J'), Card::from_2(&'T')], bid: 220})]
    #[case("QQQJA 483", Hand {cards: [Card::from_2(&'Q'), Card::from_2(&'Q'), Card::from_2(&'Q'), Card::from_2(&'J'), Card::from_2(&'A')], bid: 483})]
    fn parse_hand_2(#[case] input: &str, #[case] expected: Hand) {
        assert_eq!(Hand::from_2(input).unwrap(), expected);
    }

    #[rstest]
    #[case(Hand::from_str("32T3K 0").unwrap(), Type::Pair)]
    #[case(Hand::from_str("KK677 0").unwrap(), Type::TwoPair)]
    #[case(Hand::from_str("KTJJT 0").unwrap(), Type::TwoPair)]
    #[case(Hand::from_str("T55J5 0").unwrap(), Type::ThreeOfaKind)]
    #[case(Hand::from_str("QQQJA 0").unwrap(), Type::ThreeOfaKind)]
    #[case(Hand::from_str("QQQJJ 0").unwrap(), Type::FullHouse)]
    #[case(Hand::from_str("QQQQA 0").unwrap(), Type::FourOfaKind)]
    #[case(Hand::from_str("QQQQQ 0").unwrap(), Type::FiveOfaKind)]
    fn test_type(#[case] hand: Hand, #[case] type_: Type) {
        assert_eq!(hand.type_(), type_);
    }

    #[rstest]
    #[case(Hand::from_str("KK677 0").unwrap(), Hand::from_str("32T3K 0").unwrap())]
    #[case(Hand::from_str("KK677 0").unwrap(), Hand::from_str("KTJJT 0").unwrap())]
    #[case(Hand::from_str("T55J5 0").unwrap(), Hand::from_str("KK677 0").unwrap())]
    #[case(Hand::from_str("QQQJA 0").unwrap(), Hand::from_str("T55J5 0").unwrap())]
    fn test_order(#[case] gt: Hand, #[case] lt: Hand) {
        assert_eq!(gt.partial_cmp(&lt), Some(std::cmp::Ordering::Greater))
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        assert_eq!(part_a(example_input), 6440)
    }

    #[rstest]
    #[case(Hand::from_2("32T5J 0").unwrap(), Type::Pair)]
    #[case(Hand::from_2("KK67J 0").unwrap(), Type::ThreeOfaKind)]
    #[case(Hand::from_2("KTJJT 0").unwrap(), Type::FourOfaKind)]
    #[case(Hand::from_2("T55J5 0").unwrap(), Type::FourOfaKind)]
    #[case(Hand::from_2("QQQJA 0").unwrap(), Type::FourOfaKind)]
    #[case(Hand::from_2("QQJAA 0").unwrap(), Type::FullHouse)]
    #[case(Hand::from_2("QQQJA 0").unwrap(), Type::FourOfaKind)]
    #[case(Hand::from_2("QJJJJ 0").unwrap(), Type::FiveOfaKind)]
    fn test_type_2(#[case] hand: Hand, #[case] expected: Type) {
        assert_eq!(hand.type_(), expected)
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        assert_eq!(part_b(example_input), 5905)
    }
}
