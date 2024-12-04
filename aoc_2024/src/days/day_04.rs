pub fn solve_day(input_file: String) -> (u32, u32) {
    let puzzle = parse_input(&input_file);
    let a = part_a(&puzzle);
    let b = part_b(&puzzle);
    (a, b)
}

fn parse_input(input_file: &str) -> Vec<Vec<char>> {
    input_file
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn part_a(puzzle: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            for i_dir in -1..=1 {
                for j_dir in -1..=1 {
                    count += check_xmas_for_dir(puzzle, i as i32, j as i32, i_dir, j_dir);
                }
            }
        }
    }
    count
}

fn check_xmas_for_dir(puzzle: &[Vec<char>], i: i32, j: i32, i_off: i32, j_off: i32) -> u32 {
    let word = ['X', 'M', 'A', 'S'];
    let mut correct = 0;
    for mul in 0..=3 {
        if let Ok(index_i) = TryInto::<usize>::try_into(i + i_off * mul) {
            if let Ok(index_j) = TryInto::<usize>::try_into(j + j_off * mul) {
                if puzzle.get(index_i).is_some_and(|l: &Vec<char>| {
                    l.get(index_j)
                        .is_some_and(|char| char == &word[TryInto::<usize>::try_into(mul).unwrap()])
                }) {
                    correct += 1;
                } else {
                    return 0;
                };
            }
        }
    }
    if correct == 4 {
        1
    } else {
        0
    }
}

fn part_b(puzzle: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for i in 1..(puzzle.len() - 1) {
        for j in 1..(puzzle[i].len() - 1) {
            count += check_x_mas(puzzle, i, j);
        }
    }
    count
}

fn check_x_mas(puzzle: &[Vec<char>], i: usize, j: usize) -> u32 {
    if puzzle[i][j] == 'A' {
        if puzzle[i - 1][j - 1] == puzzle[i + 1][j - 1]
            && puzzle[i - 1][j + 1] == puzzle[i + 1][j + 1]
            && puzzle[i - 1][j - 1] != puzzle[i + 1][j + 1]
            && (puzzle[i - 1][j - 1] == 'M' || puzzle[i - 1][j - 1] == 'S')
            && (puzzle[i + 1][j + 1] == 'M' || puzzle[i + 1][j + 1] == 'S')
        {
            return 1;
        }

        if puzzle[i - 1][j - 1] == puzzle[i - 1][j + 1]
            && puzzle[i + 1][j - 1] == puzzle[i + 1][j + 1]
            && puzzle[i - 1][j - 1] != puzzle[i + 1][j + 1]
            && (puzzle[i - 1][j - 1] == 'M' || puzzle[i - 1][j - 1] == 'S')
            && (puzzle[i + 1][j + 1] == 'M' || puzzle[i + 1][j + 1] == 'S')
        {
            return 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let expected = vec![
            vec!['.', '.', 'X', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', 'A', '.', '.', 'A', '.'],
            vec!['X', 'M', 'A', 'S', '.', 'S'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ];
        let actual = parse_input(
            "..X...
.SAMX.
.A..A.
XMAS.S
.X....",
        );
        assert_eq!(expected, actual)
    }

    #[rstest]
    #[case("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....", 4)]
    #[case("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    fn test_part_a(#[case] input: &str, #[case] expected: u32) {
        let puzzle = parse_input(input);
        assert_eq!(part_a(&puzzle), expected);
    }

    #[rstest]
    #[case("..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....", 0)]
    #[case("M.S\n.A.\nM.S", 1)]
    #[case("S.M\n.A.\nS.M", 1)]
    #[case("S.S\n.A.\nM.M", 1)]
    #[case("M.M\n.A.\nS.S", 1)]
    #[case("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 9)]
    fn test_part_b(#[case] input: &str, #[case] expected: u32) {
        let puzzle = parse_input(input);
        assert_eq!(part_b(&puzzle), expected);
    }
}
