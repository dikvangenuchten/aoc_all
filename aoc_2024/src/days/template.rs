pub fn solve_day(input_file: String) -> (u32, u32) {
    let a = part_a(&input_file);
    let b = part_b(&input_file);
    (a, b)
}

pub fn part_a(input_file: &str) -> u32 {}

pub fn part_b(input_file: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {}
}
