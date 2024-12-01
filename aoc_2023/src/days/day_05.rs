use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_day(input: &str) -> (u64, u64) {
    let (seeds, maps) = parse_input(input);
    (part_a(&seeds, &maps), part_b(&seeds, &maps))
}

#[derive(Debug, PartialEq, Eq)]
struct Seeds {
    seeds: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct SeedRange {
    start: u64,
    length: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    converters: Vec<Converter>,
}

impl Map {
    fn convert(&self, x: u64) -> u64 {
        for c in &self.converters {
            if c.is_input(x) {
                return c.convert(x);
            }
        }
        x
    }

    fn convert_range(&self, ranges: Vec<SeedRange>) -> Vec<SeedRange> {
        let not_converted = ranges;
        let (mut converted, not_converted) = self.converters.iter().fold(
            (vec![], not_converted),
            |(mut converted, not_converted), converter| {
                let (c, not_converted) = converter.convert_ranges(not_converted);
                converted.extend(c);
                (converted, not_converted)
            },
        );
        converted.extend(not_converted);
        converted
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Converter {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

impl Converter {
    fn is_input(&self, x: u64) -> bool {
        (self.src_start <= x) && (x < self.src_start + self.length)
    }

    fn convert(&self, x: u64) -> u64 {
        assert!(self.is_input(x));
        (self.dst_start + x) - self.src_start
    }

    fn convert_ranges(&self, ranges: Vec<SeedRange>) -> (Vec<SeedRange>, Vec<SeedRange>) {
        let mut converted = Vec::with_capacity(ranges.len());
        let mut not_converted = Vec::with_capacity(ranges.len());

        for mut range in ranges {
            let left_bound = range.start;
            let right_bound = range.start + (range.length - 1);
            if self.left_bound() > right_bound || self.right_bound() < left_bound {
                not_converted.push(range);
                continue;
            }
            // Split up the range in 3 (or less) parts
            if left_bound < self.left_bound() {
                not_converted.push(SeedRange {
                    start: range.start,
                    length: self.left_bound() - range.start,
                });
                range = SeedRange {
                    start: self.left_bound(),
                    length: range.length - (self.left_bound() - range.start),
                }
            }
            if right_bound > self.right_bound() {
                not_converted.push(SeedRange {
                    start: self.right_bound(),
                    length: right_bound - self.right_bound(),
                });

                range = SeedRange {
                    start: range.start,
                    length: range.length - (right_bound - self.right_bound()),
                }
            }
            converted.push(SeedRange {
                start: self.dst_start + (range.start - self.src_start),
                length: range.length,
            });
        }
        (converted, not_converted)
    }

    fn left_bound(&self) -> u64 {
        self.src_start
    }

    fn right_bound(&self) -> u64 {
        self.src_start + self.length
    }
}

impl From<[u64; 3]> for Converter {
    fn from(value: [u64; 3]) -> Self {
        Converter {
            dst_start: value[0],
            src_start: value[1],
            length: value[2],
        }
    }
}

impl From<[u64; 2]> for SeedRange {
    fn from(value: [u64; 2]) -> SeedRange {
        SeedRange {
            start: value[0],
            length: value[1],
        }
    }
}

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(\d+)").unwrap();
}

fn extract_digits_from_line(line: &str) -> Vec<u64> {
    RE_DIGITS
        .captures_iter(line)
        .map(|x| x.get(0).unwrap().as_str().parse::<u64>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Seeds, Vec<Map>) {
    let (seeds_line, conversion_maps) = input.split_once("\n\n").expect("Invalid inpt");

    let seeds = Seeds {
        seeds: extract_digits_from_line(seeds_line),
    };

    let mut maps = vec![];
    for map_str in conversion_maps.split("\n\n") {
        let mut converters: Vec<Converter> = Vec::with_capacity(6);
        for conversion_chunk in extract_digits_from_line(map_str).chunks(3) {
            let converter =
                Converter::from(<[u64; 3]>::try_from(conversion_chunk).expect("Chunks"));
            converters.push(converter);
        }
        maps.push(Map { converters })
    }

    (seeds, maps)
}

fn full_convert(maps: &[Map], seed: u64) -> u64 {
    maps.iter().fold(seed, |val, map| map.convert(val))
}

fn full_convert_range(maps: &[Map], seedranges: Vec<SeedRange>) -> Vec<SeedRange> {
    maps.iter()
        .fold(seedranges, |ranges, map| map.convert_range(ranges))
}

fn part_a(seeds: &Seeds, maps: &[Map]) -> u64 {
    seeds
        .seeds
        .iter()
        .map(|s| full_convert(maps, *s))
        .min()
        .unwrap()
}

fn parse_seeds_part_2(seeds: &Seeds) -> Vec<SeedRange> {
    seeds
        .seeds
        .chunks(2)
        .map(|x| SeedRange {
            start: x[0],
            length: x[1],
        })
        .collect()
}

fn part_b(seeds: &Seeds, maps: &[Map]) -> u64 {
    let seeds = parse_seeds_part_2(seeds);

    full_convert_range(maps, seeds)
        .iter()
        .map(|s| s.start)
        .min()
        .unwrap() as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::read_test_day_input;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        read_test_day_input("day_05").leak()
    }

    #[rstest]
    fn test_parse_input(example_input: &str, first_map: Map) {
        let (seeds, maps) = parse_input(example_input);
        assert_eq!(
            seeds,
            Seeds {
                seeds: vec![79, 14, 55, 13]
            }
        );

        assert_eq!(maps[0], first_map);

        assert_eq!(
            maps[1],
            Map {
                converters: vec![
                    Converter::from([0, 15, 37]),
                    Converter::from([37, 52, 2]),
                    Converter::from([39, 0, 15])
                ]
            }
        );

        assert_eq!(
            maps[6],
            Map {
                converters: vec![Converter::from([60, 56, 37]), Converter::from([56, 93, 4])]
            }
        );
    }

    #[rstest]
    fn test_converter_from_slice() {
        assert_eq!(
            Converter::from([50, 98, 2]),
            Converter {
                src_start: 98,
                dst_start: 50,
                length: 2,
            }
        )
    }

    #[fixture]
    fn first_map() -> Map {
        Map {
            converters: vec![Converter::from([50, 98, 2]), Converter::from([52, 50, 48])],
        }
    }

    #[rstest]
    fn test_single_conversion(first_map: Map) {
        let input_range = 0..100;
        let expected_output_range: Vec<u64> = (0..50).chain(52..100).chain(50..52).collect();
        let actual_output_range: Vec<u64> = input_range.map(|x| first_map.convert(x)).collect();

        for (actual, expected) in actual_output_range.iter().zip(expected_output_range) {
            assert_eq!(actual, &expected);
        }
    }

    #[fixture]
    fn all_maps(example_input: &str) -> Vec<Map> {
        let (_, maps) = parse_input(example_input);
        maps
    }

    #[fixture]
    fn seeds_part_1(example_input: &str) -> Seeds {
        let (seeds, _) = parse_input(example_input);
        seeds
    }

    #[rstest]
    #[case(79, 82)]
    #[case(14, 43)]
    #[case(55, 86)]
    #[case(13, 35)]
    fn test_full_convert(all_maps: Vec<Map>, #[case] seed: u64, #[case] location: u64) {
        assert_eq!(full_convert(&all_maps, seed), location)
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        let (seeds, maps) = parse_input(example_input);
        assert_eq!(part_a(&seeds, &maps), 35)
    }

    #[rstest]
    fn test_converter() {
        let converter = Converter::from([50, 98, 2]);

        assert_eq!(converter.convert(98), 50);
        assert_eq!(converter.convert(99), 51);
    }

    #[rstest]
    fn test_parse_seeds_part_2(seeds_part_1: Seeds) {
        assert_eq!(
            parse_seeds_part_2(&seeds_part_1),
            vec![
                SeedRange {
                    start: 79,
                    length: 14
                },
                SeedRange {
                    start: 55,
                    length: 13
                }
            ]
        )
    }

    #[fixture]
    fn seeds_part_2(seeds_part_1: Seeds) -> Vec<SeedRange> {
        parse_seeds_part_2(&seeds_part_1)
    }

    #[rstest]
    #[case(vec![SeedRange::from([0, 50])], vec![SeedRange::from([0, 50])])]
    #[case(vec![SeedRange::from([0, 51])], vec![SeedRange::from([0, 50]), SeedRange::from([52, 1])])]
    #[case(vec![SeedRange::from([0, 100])], vec![SeedRange::from([0, 50]), SeedRange::from([52, 48]), SeedRange::from([50, 2])])]
    #[case(vec![SeedRange::from([82, 1])], vec![SeedRange::from([84, 1])])]
    fn test_convert_range(
        first_map: Map,
        #[case] input_range: Vec<SeedRange>,
        #[case] mut expected_range: Vec<SeedRange>,
    ) {
        let mut calculated = first_map.convert_range(input_range);
        calculated.sort();
        expected_range.sort();
        assert_eq!(calculated, expected_range)
    }

    #[rstest]
    #[case(vec![SeedRange::from([50, 10])], vec![SeedRange::from([100, 10])])]
    #[case(vec![SeedRange::from([40, 20])], vec![SeedRange::from([100, 10]), SeedRange::from([40, 10])])]
    #[case(vec![SeedRange::from([50, 20])], vec![SeedRange::from([100, 11]), SeedRange::from([60, 9])])]
    #[case(vec![SeedRange::from([40, 30])], vec![SeedRange::from([100, 11]), SeedRange::from([60, 9]), SeedRange::from([40, 10])])]
    #[case(vec![SeedRange::from([51, 1])], vec![SeedRange::from([101, 1])])]
    #[case(vec![SeedRange::from([52, 1])], vec![SeedRange::from([102, 1])])]
    fn test_converter_convert_range(
        #[case] input_ranges: Vec<SeedRange>,
        #[case] mut expected_range: Vec<SeedRange>,
    ) {
        let converter = Converter::from([100, 50, 10]);
        let (mut c, n) = converter.convert_ranges(input_ranges);
        c.extend(n);
        c.sort();
        expected_range.sort();
        assert_eq!(c, expected_range)
    }

    #[rstest]
    fn test_part_b_extra(all_maps: Vec<Map>) {
        let seeds: Vec<SeedRange> = vec![SeedRange::from([82, 1])];
        let seeds = all_maps[0].convert_range(seeds);
        assert_eq!(seeds, vec![SeedRange::from([84, 1])]);
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        let (seeds, maps) = parse_input(example_input);
        assert_eq!(part_b(&seeds, &maps), 46)
    }
}
