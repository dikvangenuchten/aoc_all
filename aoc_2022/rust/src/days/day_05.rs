use itertools::Itertools;
use regex::Regex;

type CrateStack = Vec<char>;
type CrateStacks = Vec<CrateStack>;
type Operation = (u32, u32, u32);
type Operations = Vec<Operation>;

pub fn solve(input_str: &str) -> (String, String) {
    let (crates, operations) = parse_input(input_str);
    let part_1 = solve_part_1(crates.clone(), &operations);
    let part_2 = solve_part_2(crates, &operations);
    (part_1, part_2)
}

fn solve_part_1(crates: CrateStacks, operations: &Operations) -> String {
    let crates = apply_operations_p1(crates, operations);
    let mut out = vec![];
    for mut _crate in crates {
        if let Some(_crate) = _crate.pop() {
            out.push(_crate);
        }
    }
    String::from_iter(out)
}

fn solve_part_2(crates: CrateStacks, operations: &Operations) -> String {
    let crates = apply_operations_p2(crates, operations);
    let mut out = vec![];
    for mut _crate in crates {
        if let Some(_crate) = _crate.pop() {
            out.push(_crate);
        }
    }
    String::from_iter(out)
}

fn parse_input(input_str: &str) -> (CrateStacks, Operations) {
    let crates = extract_crate_part(input_str);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let operations = re
        .captures_iter(input_str)
        .map(|operation| {
            (
                operation.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                operation.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                operation.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .collect_vec();
    (crates, operations)
}

fn extract_crate_part(input_str: &str) -> Vec<Vec<char>> {
    let mut i = 0;
    let mut crates: CrateStacks = vec![];
    // Parse crate stacks
    for _crate in input_str.chars().into_iter().chunks(4).into_iter() {
        match _crate.collect::<CrateStack>()[..] {
            ['[', x, ']', end_char] => {
                match crates.get_mut(i) {
                    Some(stack) => stack.insert(0, x),
                    None => {
                        if crates.len() <= i {
                            crates.resize(i + 1, vec![]);
                        }
                        crates[i] = vec![x]
                    }
                };
                if end_char == '\n' {
                    i = 0
                } else {
                    i += 1
                }
            }
            [' ', ' ', ' ', ' '] => i += 1,
            [' ', ' ', ' ', '\n'] => i = 0,
            _ => break,
        };
    }
    crates
}

fn apply_operations_p1(crates: CrateStacks, operations: &Operations) -> CrateStacks {
    operations.iter().fold(crates, |crates, operation| {
        apply_operation_p1(crates, operation)
    })
}

fn apply_operation_p1(mut crates: CrateStacks, operation: &Operation) -> CrateStacks {
    for _ in 0..operation.0 as usize {
        let _crate = crates[(operation.1 - 1) as usize].pop().unwrap();
        crates[(operation.2 - 1) as usize].push(_crate);
    }
    crates
}

fn apply_operations_p2(crates: CrateStacks, operations: &Operations) -> CrateStacks {
    operations.iter().fold(crates, |crates, operation| {
        apply_operation_p2(crates, operation)
    })
}

fn apply_operation_p2(mut crates: CrateStacks, operation: &Operation) -> CrateStacks {
    let mut intermediate_stack = vec![];
    for _ in 0..operation.0 as usize {
        let _crate = crates[(operation.1 - 1) as usize].pop().unwrap();
        intermediate_stack.push(_crate);
    }
    for _ in 0..operation.0 as usize {
        let _crate = intermediate_stack.pop().unwrap();
        crates[(operation.2 - 1) as usize].push(_crate);
    }

    crates
}

#[cfg(test)]
mod tests {

    use crate::days::read_day_input;

    use super::*;
    use rstest::*;

    #[fixture]
    fn input_str() -> String {
        read_day_input("test_day_05")
    }

    #[fixture]
    fn parsed_input() -> (CrateStacks, Operations) {
        (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
        )
    }

    #[rstest]
    fn test_parse_input(input_str: String, parsed_input: (CrateStacks, Operations)) {
        assert_eq!(parse_input(&input_str), parsed_input)
    }

    #[rstest]
    #[case(
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        (1, 2, 1),
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
    )]
    #[case(
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
        (3, 1, 3),
        vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
    )]
    #[case(
        vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']],
        (2, 2, 1),
        vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']],

    )]
    #[case(
        vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']],
        (1, 1, 2),
        vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],

    )]
    fn test_apply_operation_p1(
        #[case] crates: CrateStacks,
        #[case] operation: Operation,
        #[case] crates_after: CrateStacks,
    ) {
        assert_eq!(apply_operation_p1(crates, &operation), crates_after)
    }

    #[rstest]
    #[case(
        vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']],
    )]
    fn test_apply_operations_p1(input_str: String, #[case] expected: CrateStacks) {
        let (crates, operations) = parse_input(&input_str);
        assert_eq!(apply_operations_p1(crates, &operations), expected)
    }

    #[rstest]
    fn test_solve_part_1(input_str: String) {
        let (crates, operations) = parse_input(&input_str);
        assert_eq!(solve_part_1(crates, &operations), "CMZ")
    }

    #[rstest]
    #[case(
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        (1, 2, 1),
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
    )]
    #[case(
        vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
        (3, 1, 3),
        vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']],
    )]
    #[case(
        vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']],
        (2, 2, 1),
        vec![vec!['M', 'C'], vec![], vec!['P', 'Z', 'N', 'D']],

    )]
    #[case(
        vec![vec!['M', 'C'], vec![], vec!['P', 'Z', 'N', 'D']],
        (1, 1, 2),
        vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']],

    )]
    fn test_apply_operation_p2(
        #[case] crates: CrateStacks,
        #[case] operation: Operation,
        #[case] crates_after: CrateStacks,
    ) {
        assert_eq!(apply_operation_p2(crates, &operation), crates_after)
    }
}
