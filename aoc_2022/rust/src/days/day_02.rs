pub fn solve(input: &str) -> (u32, u32) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn solve_part_1(input_str: &str) -> u32 {
    input_str
        .split('\n')
        .map(calculate_round_score_part_1)
        .sum()
}

fn solve_part_2(input_str: &str) -> u32 {
    input_str
        .split('\n')
        .map(calculate_round_score_part_2)
        .sum()
}

fn calculate_round_score_part_1(round_str: &str) -> u32 {
    match round_str {
        // Win
        "A Y" => 8,
        "B Z" => 9,
        "C X" => 7,
        // Draw
        "A X" => 4,
        "B Y" => 5,
        "C Z" => 6,
        // Lose
        "A Z" => 3,
        "B X" => 1,
        "C Y" => 2,
        _ => unreachable!("Invalid input"),
    }
}

fn calculate_round_score_part_2(round_str: &str) -> u32 {
    match round_str {
        // Win
        "A Y" => 4,
        "B Z" => 9,
        "C X" => 2,
        // Draw
        "A X" => 3,
        "B Y" => 5,
        "C Z" => 7,
        // Lose
        "A Z" => 8,
        "B X" => 1,
        "C Y" => 6,
        _ => unreachable!("Invalid input"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_day_2p1(#[case] input_str: &str) {
        assert_eq!(solve_part_1(input_str), 15)
    }

    #[rstest]
    #[case("A Y\nB X\nC Z")]
    fn test_day_2p2(#[case] input_str: &str) {
        assert_eq!(solve_part_2(input_str), 12)
    }

    #[rstest]
    #[case("A Y", 8)]
    #[case("B X", 1)]
    #[case("C Z", 6)]
    fn test_round_scoring(#[case] round_str: &str, #[case] expected_score: u32) {
        assert_eq!(calculate_round_score_part_1(round_str), expected_score);
    }

    #[rstest]
    #[case("A Y", 4)]
    #[case("B X", 1)]
    #[case("C Z", 7)]
    fn test_round_scoring_p2(#[case] round_str: &str, #[case] expected_score: u32) {
        assert_eq!(calculate_round_score_part_2(round_str), expected_score);
    }
}
