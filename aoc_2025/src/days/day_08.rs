use anyhow::{Result, anyhow};
use std::{
    collections::{BinaryHeap, HashSet},
    str::FromStr,
    vec,
};

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let boxes = parse(input_file);
    let a = part_a(&boxes, 1_000);
    let b = part_b(&boxes);
    (a, b)
}

fn parse(input_file: &str) -> Vec<Box> {
    input_file
        .trim()
        .lines()
        .filter_map(|line| Box::from_str(line).ok())
        .collect()
}

fn part_a(boxes: &[Box], n: usize) -> u64 {
    let mut distances = boxes
        .iter()
        .flat_map(|b1| boxes.iter().map(|b2| (*b1, *b2)))
        .filter(|(b1, b2)| b1 < b2)
        .map(|(b1, b2)| BoxDistance {
            distance: b1.distance(&b2),
            box1: b1,
            box2: b2,
        })
        .collect::<BinaryHeap<BoxDistance>>();

    let mut circuits: Vec<Circuit> = vec![];
    for _ in 0..n {
        let box_distance = distances
            .pop()
            .expect("There should be more distances then boxes");

        let mut in_circuits = vec![];
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.boxes.contains(&box_distance.box1)
                || circuit.boxes.contains(&box_distance.box2)
            {
                in_circuits.push(i);
            }
        }
        if in_circuits.is_empty() {
            circuits.push(Circuit::new(vec![box_distance.box1, box_distance.box2]));
        } else if in_circuits.len() == 1 {
            let circuit = &mut circuits[in_circuits[0]];
            circuit.boxes.insert(box_distance.box1);
            circuit.boxes.insert(box_distance.box2);
        } else {
            let mut merged_circuit = Circuit::new(vec![]);
            for &i in in_circuits.iter().rev() {
                let circuit = circuits.remove(i);
                merged_circuit.merge(&circuit);
            }
            merged_circuit.boxes.insert(box_distance.box1);
            merged_circuit.boxes.insert(box_distance.box2);
            circuits.push(merged_circuit);
        }
    }

    let mut heap: BinaryHeap<usize> = circuits.iter().map(|c| c.len()).collect();
    let mut prod = 1;
    for _ in 0..3 {
        prod *= heap.pop().unwrap() as u64;
    }
    prod
}

fn part_b(boxes: &[Box]) -> u64 {
    let mut distances = boxes
        .iter()
        .flat_map(|b1| boxes.iter().map(|b2| (*b1, *b2)))
        .filter(|(b1, b2)| b1 < b2)
        .map(|(b1, b2)| BoxDistance {
            distance: b1.distance(&b2),
            box1: b1,
            box2: b2,
        })
        .collect::<BinaryHeap<BoxDistance>>();

    let mut circuits: Vec<Circuit> = boxes.iter().map(|b| Circuit::new(vec![*b])).collect();

    loop {
        let box_distance = distances
            .pop()
            .expect("There should be more distances then boxes");

        let mut in_circuits = vec![];
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.boxes.contains(&box_distance.box1)
                || circuit.boxes.contains(&box_distance.box2)
            {
                in_circuits.push(i);
            }
        }
        if in_circuits.len() == 1 {
            let circuit = &mut circuits[in_circuits[0]];
            circuit.boxes.insert(box_distance.box1);
            circuit.boxes.insert(box_distance.box2);
        } else {
            let mut merged_circuit = Circuit::new(vec![]);
            for &i in in_circuits.iter().rev() {
                let circuit = circuits.remove(i);
                merged_circuit.merge(&circuit);
            }
            merged_circuit.boxes.insert(box_distance.box1);
            merged_circuit.boxes.insert(box_distance.box2);
            circuits.push(merged_circuit);
        }
        if circuits.len() == 1 {
            return (box_distance.box1.x * box_distance.box2.x) as u64;
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Box {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct BoxDistance {
    distance: f32,
    box1: Box,
    box2: Box,
}
#[derive(Debug, Clone)]
struct Circuit {
    boxes: HashSet<Box>,
}

impl Circuit {
    fn new(boxes: Vec<Box>) -> Self {
        Self {
            boxes: boxes.into_iter().collect(),
        }
    }

    fn merge(&mut self, other: &Circuit) {
        for b in other.boxes.iter() {
            self.boxes.insert(*b);
        }
    }

    fn len(&self) -> usize {
        self.boxes.len()
    }
}

impl Eq for BoxDistance {}

impl PartialOrd for BoxDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoxDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

impl Box {
    fn distance(&self, other: &Box) -> f32 {
        ((self.x as f32 - other.x as f32).powi(2)
            + (self.y as f32 - other.y as f32).powi(2)
            + (self.z as f32 - other.z as f32).powi(2))
        .sqrt()
    }
}

impl FromStr for Box {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid input format"));
        }
        let x = parts[0].parse::<u32>()?;
        let y = parts[1].parse::<u32>()?;
        let z = parts[2].parse::<u32>()?;
        Ok(Box { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::read_test_day_input;
    use rstest::rstest;

    #[rstest]
    #[case("0,0,0", Box { x: 0, y: 0, z: 0 })]
    #[case("10,11,12", Box { x: 10, y: 11, z: 12 })]
    fn test_parse_box(#[case] input: &str, #[case] expected: Box) {
        let parsed = Box::from_str(input);
        assert_eq!(parsed.unwrap(), expected);
    }

    #[rstest]
    #[case("0,0,0", "1,0,0", 1.0)]
    #[case("0,0,0", "0,1,0", 1.0)]
    #[case("0,0,0", "0,0,1", 1.0)]
    #[case("0,0,0", "1,1,1", (3.0f32).sqrt())]
    #[case("0,0,0", "2,2,2", (12.0f32).sqrt())]
    fn test_calc_distance(#[case] box1: Box, #[case] box2: Box, #[case] expected_distance: f32) {
        let distance = box1.distance(&box2);
        assert_eq!(distance, expected_distance);
        let reverse_distance = box2.distance(&box1);
        assert_eq!(reverse_distance, expected_distance);
    }

    #[rstest]
    fn test_part_a() {
        let input = read_test_day_input("08");
        let boxes: Vec<Box> = input
            .lines()
            .map(|line| Box::from_str(line).unwrap())
            .collect();
        let result = part_a(&boxes, 10);
        assert_eq!(result, 40);
    }

    #[rstest]
    fn test_part_b() {
        let input = read_test_day_input("08");
        let boxes: Vec<Box> = input
            .lines()
            .map(|line| Box::from_str(line).unwrap())
            .collect();
        let result = part_b(&boxes);
        assert_eq!(result, 25272);
    }
}
