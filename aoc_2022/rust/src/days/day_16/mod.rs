mod day_16_old;
mod network;
mod parse;
mod valve;
use day_16_old::{parse_input, solve_part_1, solve_part_2};

pub fn solve(input: &str) -> (u32, u32) {
    let valves = parse_input(input);
    let part_1 = solve_part_1(&valves, 30);
    let part_2 = solve_part_2(&valves, 25);
    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use super::day_16_old::Valve;
    use super::*;
    use crate::days::read_day_input;

    use rstest::*;

    #[fixture]
    fn example_input_str() -> &'static str {
        Box::leak(read_day_input("test_day_16").into_boxed_str())
    }

    #[fixture]
    fn example_graph(example_input_str: &'static str) -> Vec<Valve<'static>> {
        parse_input(example_input_str)
    }

    #[rstest]
    fn test_example_part_1(example_graph: Vec<Valve<'static>>) {
        assert_eq!(solve_part_1(&example_graph, 30), 1651)
    }

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 0)]
    #[case(3, 20)]
    #[case(12, 573)]
    #[case(25, 1626)]
    #[case(26, 1707)]
    fn test_example_part_2(
        example_graph: Vec<Valve<'static>>,
        #[case] minutes_remaining: u32,
        #[case] total_pressure_released: u32,
    ) {
        assert_eq!(
            solve_part_2(&example_graph, minutes_remaining),
            total_pressure_released
        )
    }

    #[rstest]
    #[case("Valve AW has flow rate=0; tunnels lead to valves LG, TL", vec![Valve::new("AW", 0,  vec!["LG", "TL"])])]
    #[case("Valve AW has flow rate=10; tunnels lead to valves LG, TL", vec![Valve::new("AW", 10, vec!["LG", "TL"])])]
    fn test_parse_input(#[case] example_input_str: &str, #[case] expected: Vec<Valve>) {
        assert_eq!(parse_input(example_input_str), expected)
    }
}
