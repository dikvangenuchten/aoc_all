use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_day(input: &str) -> (u64, u64) {
    let (part_numbers, symbols) = parse_input(input);
    (
        part_a(&part_numbers, &symbols),
        part_b(&part_numbers, &symbols),
    )
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Location {
    x: u8,
    y: u8,
}

impl From<(u8, u8)> for Location {
    fn from(value: (u8, u8)) -> Self {
        Location {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PartNumber {
    value: u64,
    start_loc: Location,
    end_loc: Location,
}

impl PartNumber {
    fn neighbores_symbol(&self, symbols: &[Vec<Option<Symbol>>]) -> bool {
        for n in self.get_neighbores() {
            if let Some(row) = symbols.get(n.y as usize) {
                if let Some(Some(_)) = row.get(n.x as usize) {
                    return true;
                }
            }
        }
        false
    }

    fn is_possible_gear(&self, symbols: &[Vec<Option<Symbol>>]) -> bool {
        for n in self.get_neighbores() {
            if let Some(row) = symbols.get(n.y as usize) {
                if let Some(Some(loc)) = row.get(n.x as usize) {
                    if loc.value == '*' {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_neighbor_of_location(&self, loc: &Location) -> bool {
        if (self.start_loc.x.max(1) - 1) <= loc.x
            && loc.x <= (self.end_loc.x + 1)
            && (self.start_loc.y.max(1) - 1) <= loc.y
            && loc.y <= (self.end_loc.y + 1)
        {
            return true;
        }
        false
    }

    fn get_neighbores(&self) -> Vec<Location> {
        let y = self.start_loc.y;
        ((self.start_loc.x.max(1) - 1)..=(self.end_loc.x + 1))
            .flat_map(|x| {
                [
                    Location {
                        x,
                        y: (y.max(1) - 1).max(0),
                    },
                    Location { x, y },
                    Location {
                        x,
                        y: (y + 1).max(0),
                    },
                ]
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Symbol {
    value: char,
}

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"(\d+)").expect("regex should be valid");
}

fn parse_input(input: &str) -> (Vec<PartNumber>, Vec<Vec<Option<Symbol>>>) {
    let part_numbers = input
        .split('\n')
        .enumerate()
        .flat_map(|(i, line)| parse_line_part_number(line, i as u8))
        .collect::<Vec<PartNumber>>();
    let symbols = parse_symbols(input);
    (part_numbers, symbols)
}

fn parse_line_part_number(line: &'_ str, y_idx: u8) -> impl Iterator<Item = PartNumber> + '_ {
    return RE_NUMBER.captures_iter(line).map(move |c| PartNumber {
        value: c.get(0).unwrap().as_str().parse::<u64>().unwrap(),
        start_loc: (c.get(0).unwrap().range().start as u8, y_idx).into(),
        end_loc: (c.get(0).unwrap().range().end as u8 - 1, y_idx).into(),
    });
}

fn parse_symbols(input: &str) -> Vec<Vec<Option<Symbol>>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    '0'..='9' => None,
                    _ => Some(Symbol { value: c }),
                })
                .collect()
        })
        .collect()
}

fn part_a(part_numbers: &[PartNumber], symbols: &[Vec<Option<Symbol>>]) -> u64 {
    part_numbers
        .iter()
        .filter(|p| p.neighbores_symbol(symbols))
        .fold(0, |sum, p| sum + p.value)
}

struct Gear {
    p1: u64,
    p2: u64,
}

impl Gear {
    fn ratio(&self) -> u64 {
        self.p1 * self.p2
    }
}

fn get_locs_of_symbol(symbols: &[Vec<Option<Symbol>>], val: char) -> Vec<Location> {
    symbols
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, s)| {
                if let Some(s) = s {
                    if s.value == val {
                        return Some((x as u8, y as u8).into());
                    }
                }
                None
            })
        })
        .collect::<Vec<Location>>()
}

fn part_b(part_numbers: &[PartNumber], symbols: &[Vec<Option<Symbol>>]) -> u64 {
    let possible_gears = part_numbers
        .iter()
        .filter(|p| p.is_possible_gear(symbols))
        .collect::<Vec<&PartNumber>>();

    let possbile_gear_locs = get_locs_of_symbol(symbols, '*');

    possbile_gear_locs
        .iter()
        .filter_map(|gear_loc: &Location| {
            let neighbors = &possible_gears
                .iter()
                .filter(|p| p.is_neighbor_of_location(gear_loc))
                .collect::<Vec<&&PartNumber>>();
            if neighbors.len() == 2 {
                Some(Gear {
                    p1: neighbors.get(0).unwrap().value,
                    p2: neighbors.get(1).unwrap().value,
                })
            } else {
                None
            }
        })
        .map(|g| g.ratio())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn example_input() -> &'static str {
        return "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    }

    #[rstest]
    fn test_is_part_number(example_input: &str) {
        let (part_numbers, symbols) = parse_input(example_input);
        let first_part = part_numbers.get(0).unwrap();
        assert_eq!(first_part.start_loc, (0, 0).into());
        assert_eq!(first_part.end_loc, (2, 0).into());

        assert_eq!(
            symbols
                .get(1)
                .expect("Could not get row")
                .get(3)
                .expect("Could not get index")
                .expect("Value is None"),
            Symbol { value: '*' }
        );
        assert!(part_numbers.get(0).unwrap().neighbores_symbol(&symbols));
        assert!(!part_numbers.get(1).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(2).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(3).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(4).unwrap().neighbores_symbol(&symbols));
        assert!(!part_numbers.get(5).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(6).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(7).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(8).unwrap().neighbores_symbol(&symbols));
        assert!(part_numbers.get(9).unwrap().neighbores_symbol(&symbols));
        assert_eq!(part_numbers.get(10), None);
    }

    #[rstest]
    fn test_is_possible_gear(example_input: &str) {
        let (part_numbers, symbols) = parse_input(example_input);
        assert!(part_numbers.get(0).unwrap().is_possible_gear(&symbols));
        assert!(!part_numbers.get(1).unwrap().is_possible_gear(&symbols));
        assert!(part_numbers.get(2).unwrap().is_possible_gear(&symbols));
        assert!(!part_numbers.get(3).unwrap().is_possible_gear(&symbols));
        assert!(part_numbers.get(4).unwrap().is_possible_gear(&symbols));
        assert!(!part_numbers.get(5).unwrap().is_possible_gear(&symbols));
        assert!(!part_numbers.get(6).unwrap().is_possible_gear(&symbols));
        assert!(part_numbers.get(7).unwrap().is_possible_gear(&symbols));
        assert!(!part_numbers.get(8).unwrap().is_possible_gear(&symbols));
        assert!(part_numbers.get(9).unwrap().is_possible_gear(&symbols));
        assert_eq!(part_numbers.get(10), None);
    }

    #[rstest]
    fn test_is_neighbor(example_input: &str) {
        let (part_numbers, _symbols) = parse_input(example_input);

        for p in part_numbers {
            for loc in p.get_neighbores() {
                assert!(p.is_neighbor_of_location(&loc))
            }
        }
    }

    #[rstest]
    fn test_symbol_locations(example_input: &str) {
        let (_part_numbers, symbols) = parse_input(example_input);
        let locations = get_locs_of_symbol(&symbols, '*');
        assert!(locations.contains(&Location { x: 3, y: 1 }));
        assert!(locations.contains(&Location { x: 3, y: 4 }));
        assert!(locations.contains(&Location { x: 5, y: 8 }));
        assert_eq!(locations.len(), 3);
    }

    #[rstest]
    fn test_parse_input() {
        assert_eq!(
            parse_line_part_number("467..114..", 0).collect::<Vec<PartNumber>>(),
            vec![
                PartNumber {
                    value: 467,
                    start_loc: (0, 0).into(),
                    end_loc: (2, 0).into()
                },
                PartNumber {
                    value: 114,
                    start_loc: (5, 0).into(),
                    end_loc: (7, 0).into()
                }
            ]
        )
    }

    #[rstest]
    fn test_part_a(example_input: &str) {
        let (part_numbers, symbols) = parse_input(example_input);
        assert_eq!(part_a(&part_numbers, &symbols), 4361);
    }

    #[rstest]
    fn test_part_b(example_input: &str) {
        let (part_numbers, symbols) = parse_input(example_input);
        assert_eq!(part_b(&part_numbers, &symbols), 467835);
    }
}
