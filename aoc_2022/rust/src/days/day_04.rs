use itertools::Itertools;

pub fn solve(input: &str) -> (usize, usize) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn solve_part_1(input_str: &str) -> usize {
    parse_input(input_str).filter(check_overlap).count()
}

fn solve_part_2(input_str: &str) -> usize {
    parse_input(input_str).filter(check_any_overlap).count()
}

fn check_overlap(assignment: &((u16, u16), (u16, u16))) -> bool {
    let (left, right) = assignment;
    left.0 >= right.0 && left.1 <= right.1 || right.0 >= left.0 && right.1 <= left.1
}

fn check_any_overlap(assignment: &((u16, u16), (u16, u16))) -> bool {
    let (left, right) = assignment;
    (right.0 <= left.0 && left.0 <= right.1) || (left.0 <= right.0 && right.0 <= left.1)
}

fn parse_input(input_str: &str) -> impl Iterator<Item = ((u16, u16), (u16, u16))> + '_ {
    input_str.split('\n').map(parse_elf_assignment)
}

fn parse_elf_assignment(elf_str: &str) -> ((u16, u16), (u16, u16)) {
    elf_str
        .split(',')
        .map(parse_bounds)
        .collect_tuple()
        .expect("elf should be assigned to 2 section ranges exactly")
}

fn parse_bounds(bound_str: &str) -> (u16, u16) {
    bound_str
        .split('-')
        .map(|bound| bound.parse::<u16>().expect("Should be valid"))
        .collect_tuple::<(u16, u16)>()
        .expect("Bound should contain exact 2 u16")
}

#[cfg(test)]
mod tests_day4 {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"
    }

    #[rstest]
    fn test_solve_part_1(test_input: &str) {
        assert_eq!(solve_part_1(test_input), 2)
    }

    #[rstest]
    fn test_solve_part_2(test_input: &str) {
        assert_eq!(solve_part_2(test_input), 4)
    }

    #[rstest]
    fn test_parse_input(test_input: &str) {
        assert_eq!(
            parse_input(test_input).collect::<Vec<((u16, u16), (u16, u16))>>(),
            vec![
                ((2, 4), (6, 8)),
                ((2, 3), (4, 5)),
                ((5, 7), (7, 9)),
                ((2, 8), (3, 7)),
                ((6, 6), (4, 6)),
                ((2, 6), (4, 8)),
            ],
        )
    }

    #[rstest]
    #[case("2-4", (2, 4))]
    #[case("6-8", (6, 8))]
    #[case("3-8", (3, 8))]
    fn test_parse_bounds(#[case] bound_str: &str, #[case] expected: (u16, u16)) {
        assert_eq!(parse_bounds(bound_str), expected)
    }

    #[rstest]
    #[case("2-4,6-8",((2, 4), (6, 8)))]
    #[case("2-3,4-5",((2, 3), (4, 5)))]
    #[case("5-7,7-9",((5, 7), (7, 9)))]
    #[case("2-8,3-7",((2, 8), (3, 7)))]
    #[case("6-6,4-6",((6, 6), (4, 6)))]
    #[case("2-6,4-8",((2, 6), (4, 8)))]

    fn test_parse_elf_assignment(
        #[case] elf_str: &str,
        #[case] expected: ((u16, u16), (u16, u16)),
    ) {
        assert_eq!(parse_elf_assignment(elf_str), expected)
    }

    #[rstest]
    #[case(((2, 4), (6, 8)), false)]
    #[case(((2, 3), (4, 5)), false)]
    #[case(((5, 7), (7, 9)), false)]
    #[case(((2, 8), (3, 7)), true)]
    #[case(((6, 6), (4, 6)), true)]
    #[case(((2, 6), (4, 8)), false)]
    fn test_check_overlap(#[case] assignment: ((u16, u16), (u16, u16)), #[case] expected: bool) {
        assert_eq!(check_overlap(&assignment), expected)
    }

    #[rstest]
    #[case(((2, 4), (6, 8)), false)]
    #[case(((2, 3), (4, 5)), false)]
    #[case(((5, 7), (7, 9)), true)]
    #[case(((2, 8), (3, 7)), true)]
    #[case(((6, 6), (4, 6)), true)]
    #[case(((2, 6), (4, 8)), true)]
    fn test_count_overlap(#[case] assignment: ((u16, u16), (u16, u16)), #[case] expected: bool) {
        assert_eq!(check_any_overlap(&assignment), expected)
    }
}
