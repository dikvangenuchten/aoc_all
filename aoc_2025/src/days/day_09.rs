use anyhow::{Result, anyhow};
use std::{collections::BinaryHeap, str::FromStr};

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let coords = parse(input_file);
    let a = part_a(&coords);
    let b = part_b(&coords);
    (a, b)
}

fn part_a(coords: &[Coord]) -> u64 {
    coords
        .iter()
        .flat_map(|c1| coords.iter().map(move |c2| c1.square(c2)))
        .max()
        .unwrap_or(0)
}

fn part_b(coords: &[Coord]) -> u64 {
    let mut squares = BinaryHeap::with_capacity(coords.len() * (coords.len() - 1) / 2);
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            squares.push((coords[i].square(&coords[j]), coords[i], coords[j]));
        }
    }

    let borders: Vec<_> = (0..coords.len())
        .map(|i| Line::new(coords[i], coords[(i + 1) % coords.len()]))
        .collect();

    while let Some((square, c1, c2)) = squares.pop() {
        if check_valid_square(&c1, &c2, &borders, coords) {
            return square;
        }
    }
    panic!("No valid square found");
}

fn check_valid_square(c1: &Coord, c2: &Coord, borders: &[Line], polygon: &[Coord]) -> bool {
    let min_x = c1.x.min(c2.x);
    let max_x = c1.x.max(c2.x);
    let min_y = c1.y.min(c2.y);
    let max_y = c1.y.max(c2.y);

    let corners = [
        Coord::from((min_x, min_y)),
        Coord::from((max_x, min_y)),
        Coord::from((max_x, max_y)),
        Coord::from((min_x, max_y)),
    ];

    // Check if all corners are inside the polygon
    if !corners.iter().all(|c| is_point_inside_polygon(c, polygon)) {
        return false;
    }

    let square_lines = [
        Line::new(corners[0], corners[1]),
        Line::new(corners[1], corners[2]),
        Line::new(corners[2], corners[3]),
        Line::new(corners[3], corners[0]),
    ];

    borders.iter().all(|border| {
        square_lines
            .iter()
            .all(|line| !border.crosses_out_bounds(line))
    })
}

fn is_point_inside_polygon(point: &Coord, polygon: &[Coord]) -> bool {
    // First check if point is on a polygon edge
    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let p1 = &polygon[i];
        let p2 = &polygon[j];

        // Check if point is on the edge between p1 and p2
        if is_point_on_segment(point, p1, p2) {
            return true;
        }
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let xi = polygon[i].x as f64;
        let yi = polygon[i].y as f64;
        let xj = polygon[j].x as f64;
        let yj = polygon[j].y as f64;
        let x = point.x as f64;
        let y = point.y as f64;

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn is_point_on_segment(point: &Coord, p1: &Coord, p2: &Coord) -> bool {
    // Check if it's on a horizontal or vertical line within bounds
    if p1.x == p2.x {
        point.x == p1.x && point.y >= p1.y.min(p2.y) && point.y <= p1.y.max(p2.y)
    } else if p1.y == p2.y {
        point.y == p1.y && point.x >= p1.x.min(p2.x) && point.x <= p1.x.max(p2.x)
    } else {
        false
    }
}

fn parse(input_file: &str) -> Vec<Coord> {
    input_file
        .trim()
        .lines()
        .filter_map(|line| Coord::from_str(line).ok())
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn new(start: Coord, end: Coord) -> Self {
        let line = Line {
            start: start.min(end),
            end: start.max(end),
        };
        assert!(
            line.is_horizontal() || line.is_vertical(),
            "Line must be horizontal or vertical"
        );
        line
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_parallel(&self, other: &Line) -> bool {
        (self.is_horizontal() && other.is_horizontal())
            || (self.is_vertical() && other.is_vertical())
    }

    fn crosses_out_bounds(&self, other: &Line) -> bool {
        if self.is_parallel(other) {
            self.crosses_parallel(other)
        } else if self.is_horizontal() {
            self.start.x < other.start.x
                && self.end.x > other.start.x
                && other.start.y < self.start.y
                && other.end.y > self.start.y
        } else if self.is_vertical() {
            other.start.x < self.start.x
                && other.end.x > self.start.x
                && self.start.y < other.start.y
                && self.end.y > other.start.y
        } else {
            panic!("Line is neither horizontal nor vertical: {:?}", self);
        }
    }

    fn crosses_parallel(&self, other: &Line) -> bool {
        if self.is_horizontal() {
            self.start.y == other.start.y
                && (self.start.x > other.end.x || self.end.x < other.start.x)
        } else {
            self.start.x == other.start.x
                && (self.start.y > other.end.y || self.end.y < other.start.y)
        }
    }
}

impl Coord {
    fn square(&self, other: &Coord) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .ok_or_else(|| anyhow!("Invalid coord format"))?;
        Ok(Coord {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl From<(u64, u64)> for Coord {
    fn from((x, y): (u64, u64)) -> Self {
        Coord { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("0,0", Coord { x: 0, y: 0 })]
    #[case("1,1", Coord { x: 1, y: 1 })]
    fn test_parse(#[case] input: &str, #[case] expected: Coord) {
        let result = Coord::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("7,1", "11,7", 35)]
    fn test_square(#[case] c1: Coord, #[case] c2: Coord, #[case] expected_square: u64) {
        let square = c1.square(&c2);
        assert_eq!(square, expected_square);
    }

    #[rstest]
    fn test_part_a() {
        let input = read_test_day_input("09");
        let coords = parse(&input);
        let result = part_a(&coords);
        assert_eq!(result, 50);
    }

    #[rstest]
    fn test_part_b() {
        let input = read_test_day_input("09");
        let coords = parse(&input);
        let result = part_b(&coords);
        assert_eq!(result, 24);
    }

    #[rstest]
    #[case("7,3", "11,1", true)]
    #[case("9,7", "9,5", true)]
    #[case("9,5", "2,3", true)]
    #[case("2,5", "11,1", false)]
    fn test_check_valid_square(#[case] c1: Coord, #[case] c2: Coord, #[case] expected: bool) {
        let input_file = &read_test_day_input("09");
        let coords = parse(input_file);
        let borders: Vec<_> = (0..coords.len())
            .map(|i| Line::new(coords[i], coords[(i + 1) % coords.len()]))
            .collect();
        let result = check_valid_square(&c1, &c2, &borders, &coords);
        assert_eq!(result, expected);
    }
}
