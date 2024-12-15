use std::{collections::HashMap, str::FromStr};

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let mut stones = Stones::from_str(input_file).expect("Invalid input");
    let a = part_a(&mut stones);
    let b = part_b(&mut stones);
    (a, b)
}

fn part_a(stones: &mut Stones) -> u64 {
    blink_counter(&stones.stones, 25) as u64
}

fn part_b(stones: &mut Stones) -> u64 {
    blink_counter(&stones.stones, 75) as u64
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Stones {
    stones: Vec<u64>,
    cache: HashMap<(u64, u64), usize>,
}
#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Stones {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones: Result<_, Self::Err> = s
            .trim()
            .split(" ")
            .map(|n| n.parse().map_err(|_| ParseError))
            .collect();
        Ok(Stones {
            stones: stones?,
            cache: HashMap::new(),
        })
    }
}

fn split_stone(n: &u64) -> (u64, u64) {
    let num_digits = n.ilog10() + 1;
    let sword = 10_u64.pow(num_digits / 2);
    let lhs = n / sword;
    let rhs = n % sword;
    (lhs, rhs)
}

fn _blink_recursive_cached(
    cache: &mut HashMap<(u64, u64), usize>,
    stone: u64,
    n_blinks: u64,
) -> usize {
    let key = (stone, n_blinks);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }
    let result = if n_blinks == 0 {
        1
    } else if stone == 0 {
        _blink_recursive_cached(cache, 1, n_blinks - 1)
    } else if ((stone).ilog10() + 1) % 2 == 0 {
        let (left_stone, right_stone) = split_stone(&stone);
        _blink_recursive_cached(cache, left_stone, n_blinks - 1)
            + _blink_recursive_cached(cache, right_stone, n_blinks - 1)
    } else {
        _blink_recursive_cached(cache, stone * 2024, n_blinks - 1)
    };
    cache.insert(key, result);
    result
}

fn blink_counter(stones: &[u64], n_blinks: u64) -> usize {
    let mut counter = stones.iter().fold(HashMap::new(), |mut c, stone| {
        c.entry(*stone)
            .and_modify(|c| *c += 1)
            .or_insert(1_usize);
        c
    });

    for _ in 0..n_blinks {
        counter = counter
            .drain()
            .fold(HashMap::new(), |mut c, (stone, count)| {
                if stone == 0 {
                    c.entry(1).and_modify(|c| *c += count).or_insert(count);
                } else if ((stone).ilog10() + 1) % 2 == 0 {
                    let (lhs, rhs) = split_stone(&stone);
                    c.entry(lhs).and_modify(|c| *c += count).or_insert(count);
                    c.entry(rhs).and_modify(|c| *c += count).or_insert(count);
                } else {
                    c.entry(stone * 2024)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                }
                c
            });
    }
    counter.values().sum()
}

impl Stones {
    // Used to verify cached implementation in tests
    fn _blink(mut self, n: u64) -> Self {
        for i in 0..n {
            self.stones = self
                .stones
                .iter()
                .flat_map(|stone| {
                    if stone == &0 {
                        vec![1]
                    } else if ((stone).ilog10() + 1) % 2 == 0 {
                        let val = split_stone(stone);
                        vec![val.0, val.1]
                    } else {
                        vec![stone * 2024]
                    }
                })
                .collect();
            println!("iteration: {}, len: {}", i, self._len())
        }
        self
    }
    // Used to verify cached implementation in tests
    fn _len(&self) -> usize {
        self.stones.len()
    }

    fn _length_after_n_blink(&mut self, n: u64) -> usize {
        let mut sum = 0;
        for stone in &self.stones {
            sum += _blink_recursive_cached(&mut self.cache, *stone, n);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("125 17", Ok(Stones { stones: [125, 17].into(), cache: HashMap::new() }))]
    fn test_parse(#[case] input: &str, #[case] stones: Result<Stones, ParseError>) {
        assert_eq!(Stones::from_str(input), stones)
    }

    #[rstest]
    #[case("125 17", "253000 1 7")]
    #[case("253000 1 7", "253 0 2024 14168")]
    #[case("253 0 2024 14168", "512072 1 20 24 28676032")]
    #[case("512072 1 20 24 28676032", "512 72 2024 2 0 2 4 2867 6032")]
    #[case(
        "512 72 2024 2 0 2 4 2867 6032",
        "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
    )]
    #[case(
        "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
    )]
    fn test_blink(#[case] start: Stones, #[case] after_blink: Stones) {
        assert_eq!(start._blink(1), after_blink)
    }

    #[rstest]
    #[case(10, (1, 0))]
    #[case(2024, (20, 24))]
    fn test_split_stone(#[case] number: u64, #[case] split: (u64, u64)) {
        assert_eq!(split_stone(&number), split)
    }

    #[rstest]
    #[case(0, 1)]
    #[case(0, 2)]
    #[case(0, 3)]
    #[case(0, 4)]
    #[case(0, 5)]
    #[case(0, 6)]
    #[case(125, 1)]
    #[case(125, 2)]
    #[case(125, 3)]
    #[case(125, 4)]
    #[case(125, 5)]
    #[case(125, 6)]
    #[case(125, 25)]
    fn test_blink_recursive_vs_slow(#[case] stone: u64, #[case] blinks: u64) {
        let mut stones = Stones {
            stones: vec![stone],
            cache: HashMap::new(),
        };
        assert_eq!(
            stones._length_after_n_blink(blinks),
            stones._blink(blinks)._len()
        )
    }

    #[rstest]
    #[case(0, 10)]
    fn test_blink_loop(#[case] stone: u64, #[case] blinks: u64) {
        let mut recu_cache = HashMap::new();
        assert_eq!(
            blink_counter(&[stone], blinks),
            _blink_recursive_cached(&mut recu_cache, stone, blinks)
        )
    }

    #[rstest]
    #[case("125 17", 55312)]
    fn test_part_a(#[case] mut start: Stones, #[case] num_stones: u64) {
        assert_eq!(part_a(&mut start), num_stones)
    }
}
