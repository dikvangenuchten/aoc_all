use std::{fmt::Debug, rc::Rc};

pub fn solve_day(input: &str) -> (u32, u32) {
    (part_a(input), part_b(input))
}

fn part_a(input: &str) -> u32 {
    input.trim().split(',').map(hash).sum()
}

fn part_b(input: &str) -> u32 {
    let mut boxes = Boxes::default();
    for step in input.trim().split(',') {
        boxes.step(step);
    }
    boxes.total_focal_length()
}

#[derive(Debug, Clone)]
struct Lens {
    label: Rc<str>,
    focal_length: u32,
}

#[derive(Debug, Clone)]
struct Box {
    id: u32,
    lenses: Vec<Lens>,
}

impl PartialEq for Lens {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Box {
    fn new(id: u32) -> Self {
        Box { lenses: vec![], id }
    }

    fn insert(&mut self, lens: Lens) {
        if let Some(position) = self.lenses.iter().position(|l| l.label == lens.label) {
            self.lenses[position] = lens;
        } else {
            self.lenses.push(lens);
        }
    }

    fn remove(&mut self, label: &str) {
        self.lenses.retain_mut(|l| Rc::from(label) != l.label);
    }

    fn focal_length(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(slot, lens)| self.id * (slot as u32 + 1) * lens.focal_length)
            .sum()
    }
}

struct Boxes {
    boxes: [Box; 256],
}

impl Debug for Boxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Boxes:\n")?;
        for b in &self.boxes {
            if !b.lenses.is_empty() {
                f.write_str(&format!("{:?}\n", b))?;
            }
        }
        Ok(())
    }
}

impl Default for Boxes {
    fn default() -> Self {
        Self {
            boxes: std::array::from_fn(|i| Box::new((i + 1).try_into().unwrap())),
        }
    }
}

impl Boxes {
    fn step(&mut self, step: &str) {
        if let Some((label, focal_length)) = step.split_once('=') {
            self.boxes[hash(label) as usize].insert(Lens {
                focal_length: focal_length.parse::<u32>().unwrap(),
                label: Rc::from(label),
            });
        } else if let Some((label, _)) = step.split_once('-') {
            self.boxes[hash(label) as usize].remove(label);
        } else {
            unreachable!()
        }
    }

    fn total_focal_length(&self) -> u32 {
        self.boxes.iter().map(|b| b.focal_length()).sum()
    }
}

fn hash(input: &str) -> u32 {
    let mut val = 0;
    for c in input.chars() {
        val += c as u32;
        val *= 17;
        val %= 256;
    }
    val
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("H", 200)]
    #[case("HA", 153)]
    #[case("HAS", 172)]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_example_input(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(hash(input), expected)
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn test_part_a(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part_a(input), expected)
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145)]
    fn test_part_b(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part_b(input), expected)
    }
}
