use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u64, u64) {
    let ops = parse(input_file);
    let a = part_a(&ops);
    let b = part_b(input_file);
    (a, b)
}

pub fn part_a(ops: &[Vec<CephalapodMath>]) -> u64 {
    let mut sum = 0;
    for column in ops {
        let mut stack = vec![];
        for val in column {
            match val {
                CephalapodMath::Number(x) => stack.push(*x),
                CephalapodMath::Add => {
                    let col_val: u64 = stack.iter().sum();
                    sum += col_val;
                }
                CephalapodMath::Mul => sum += stack.iter().product::<u64>(),
                CephalapodMath::None => unreachable!("Should not happen in part 1"),
            }
        }
    }
    sum
}

pub fn part_b(input: &str) -> u64 {
    let mut columns: Vec<Vec<CephalapodMath>> = vec![];
    for (i, line) in input.lines().enumerate() {
        for c in line.chars() {
            match columns.get_mut(i) {
                Some(col) => col.push(CephalapodMath::from_str(c.to_string().as_str()).unwrap()),
                None => columns.insert(
                    i,
                    vec![CephalapodMath::from_str(c.to_string().as_str()).unwrap()],
                ),
            }
        }
    }
    let rows = columns.len() - 1;

    let numbers = &columns[0..rows];
    let ops = &columns[rows];

    let mut sum = 0;

    let mut offsets = ops
        .iter()
        .enumerate()
        .filter(|(_, op)| op != &&CephalapodMath::None)
        .peekable();

    while let Some((offset, op)) = offsets.next() {
        let next = offsets.peek();
        let next_idx;
        if let Some((n, _)) = next {
            next_idx = *n
        } else {
            next_idx = columns[0].len();
        }
        let mut acc = op.init_acc();
        for i in offset..next_idx {
            let mut val = 0;
            for row in numbers {
                if let Some(n) = row[i].get_number()
                    && n != 0
                {
                    val = val * 10 + n
                }
            }
            if val == 0 {
                continue;
            }
            acc = op.apply(acc, val);
        }
        sum += acc;
    }

    sum
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CephalapodMath {
    Add,
    Mul,
    None,
    Number(u64),
}

impl CephalapodMath {
    fn get_number(&self) -> Option<u64> {
        match self {
            CephalapodMath::Add => None,
            CephalapodMath::Mul => None,
            CephalapodMath::None => None,
            CephalapodMath::Number(x) => Some(*x),
        }
    }

    fn init_acc(&self) -> u64 {
        match self {
            CephalapodMath::Add => 0,
            CephalapodMath::Mul => 1,
            CephalapodMath::None => 0,
            CephalapodMath::Number(_) => todo!(),
        }
    }

    fn apply(&self, acc: u64, val: u64) -> u64 {
        match self {
            CephalapodMath::Add => acc + val,
            CephalapodMath::Mul => acc * val,
            CephalapodMath::None => 0,
            CephalapodMath::Number(_) => todo!(),
        }
    }
}

impl FromStr for CephalapodMath {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(CephalapodMath::Add),
            "*" => Ok(CephalapodMath::Mul),
            "" => Ok(CephalapodMath::None),
            n => Ok(CephalapodMath::Number(n.parse()?)),
        }
    }
}

fn parse(input_file: &str) -> Vec<Vec<CephalapodMath>> {
    let mut columns = vec![];
    for (i, line) in input_file.trim().lines().enumerate() {
        for (j, value) in line
            .trim()
            .split(' ')
            .filter(|s| s != &" " && !s.is_empty())
            .enumerate()
        {
            if i == 0 {
                columns.push(vec![]);
            }
            columns[j].push(CephalapodMath::from_str(value).unwrap());
        }
    }
    columns
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let input = read_test_day_input("06");
        let ops = parse(&input);
        assert_eq!(
            ops,
            vec![
                vec![
                    CephalapodMath::Number(123),
                    CephalapodMath::Number(45),
                    CephalapodMath::Number(6),
                    CephalapodMath::Mul
                ],
                vec![
                    CephalapodMath::Number(328),
                    CephalapodMath::Number(64),
                    CephalapodMath::Number(98),
                    CephalapodMath::Add
                ],
                vec![
                    CephalapodMath::Number(51),
                    CephalapodMath::Number(387),
                    CephalapodMath::Number(215),
                    CephalapodMath::Mul
                ],
                vec![
                    CephalapodMath::Number(64),
                    CephalapodMath::Number(23),
                    CephalapodMath::Number(314),
                    CephalapodMath::Add
                ],
            ]
        );
    }

    #[rstest]
    fn test_part_a() {
        let input = read_test_day_input("06");
        let ops = parse(&input);
        let result = part_a(&ops);
        assert_eq!(result, 4277556)
    }
    #[rstest]
    fn test_part_b() {
        let input = read_test_day_input("06");
        let result = part_b(&input);
        assert_eq!(result, 3263827)
    }
}
