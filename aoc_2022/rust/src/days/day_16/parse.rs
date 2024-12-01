use super::valve::{Name, Valve};

pub fn parse_input(input: &str) -> Vec<Valve> {
    input.trim_end().split("\n").map(Valve::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::Valve;
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("Valve AW has flow rate=0; tunnels lead to valves LG, TL", vec![Valve::new("AW".into(), 0,  vec!["LG".into(), "TL".into()])])]
    #[case("Valve AW has flow rate=10; tunnels lead to valves LG, TL", vec![Valve::new("AW".into(), 10, vec!["LG".into(), "TL".into()])])]
    fn test_parse_input(#[case] example_input_str: &str, #[case] expected: Vec<Valve>) {
        assert_eq!(parse_input(example_input_str), expected)
    }
}
