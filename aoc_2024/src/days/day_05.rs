pub fn solve_day(input_file: String) -> (u32, u32) {
    let a = part_a(&input_file);
    let b = part_b(&input_file);
    (a, b)
}

fn part_a(input_file: &str) -> u32 {
    let (rules, orders) = input_file.split_once("\n\n").unwrap();
    let rules = Rule::parse_multiple(rules);
    orders
        .trim()
        .split("\n")
        .map(PrintOrder::parse)
        .map(|order| order.get_middle_if_valid(&rules))
        .sum()
}

fn part_b(input_file: &str) -> u32 {
    let (rules, orders) = input_file.split_once("\n\n").unwrap();
    let rules = Rule::parse_multiple(rules);
    orders
        .trim()
        .split("\n")
        .map(PrintOrder::parse)
        .filter(|order| !order.check(&rules))
        .map(|order| order.reorder(&rules))
        .map(|order| order.get_middle())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    first: u32,
    second: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct PrintOrder {
    order: Vec<u32>,
}

impl Rule {
    fn parse(value: &str) -> Self {
        let (first, second) = value.split_once("|").unwrap();
        Self {
            first: first.parse().unwrap(),
            second: second.parse().unwrap(),
        }
    }

    fn parse_multiple(value: &str) -> Vec<Self> {
        value.split("\n").map(Self::parse).collect()
    }

    fn check(&self, order: &[u32]) -> bool {
        let mut seen_second = false;
        for page in order {
            if &self.first == page {
                return !seen_second;
            }
            if &self.second == page {
                seen_second = true
            }
        }
        true
    }
}

impl PrintOrder {
    fn parse(value: &str) -> Self {
        let order = value
            .split(",")
            .map(|number| number.parse().unwrap())
            .collect();
        PrintOrder { order }
    }

    fn check(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|r| r.check(&self.order))
    }

    fn reorder(mut self, rules: &[Rule]) -> Self {
        for rule in rules {
            if !rule.check(&self.order) {
                let first = self.order.iter().position(|d| d == &rule.first).unwrap();
                let second = self.order.iter().position(|d| d == &rule.second).unwrap();
                let el = self.order.remove(second);
                self.order.insert(first, el);
            }
        }
        if !self.check(rules) {
            self = self.reorder(rules);
        }
        assert!(self.check(rules));
        self
    }

    fn get_middle(&self) -> u32 {
        let mid = self.order.len() / 2;
        self.order[mid]
    }

    fn get_middle_if_valid(&self, rules: &[Rule]) -> u32 {
        if self.check(rules) {
            self.get_middle()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::{fixture, rstest};

    #[rstest]
    #[case("47|53", Rule {first: 47, second: 53})]
    #[case("53|47", Rule {first: 53, second: 47})]
    fn test_parse_rule(#[case] input_line: &str, #[case] rule: Rule) {
        assert_eq!(Rule::parse(input_line), rule)
    }

    #[rstest]
    #[case("75,47,61,53,29", PrintOrder {order: vec![75,47,61,53,29]})]
    #[case("97,61,53,29,13", PrintOrder {order: vec![97,61,53,29,13]})]
    #[case("75,29,13", PrintOrder {order: vec![75,29,13]})]
    #[case("75,97,47,61,53", PrintOrder {order: vec![75,97,47,61,53]})]
    #[case("61,13,29", PrintOrder {order: vec![61,13,29]})]
    #[case("97,13,75,29,47", PrintOrder {order: vec![97,13,75,29,47]})]
    fn test_parse_order(#[case] input_line: &str, #[case] order: PrintOrder) {
        assert_eq!(PrintOrder::parse(input_line), order)
    }

    #[fixture]
    fn example_rules() -> Vec<Rule> {
        Rule::parse_multiple(
            "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13",
        )
    }

    #[rstest]
    #[case(PrintOrder::parse("75,47,61,53,29"), true)]
    #[case(PrintOrder::parse("97,61,53,29,13"), true)]
    #[case(PrintOrder::parse("75,29,13"), true)]
    #[case(PrintOrder::parse("75,97,47,61,53"), false)]
    #[case(PrintOrder::parse("61,13,29"), false)]
    #[case(PrintOrder::parse("97,13,75,29,47"), false)]
    fn test_order_check(
        #[case] order: PrintOrder,
        #[case] is_valid: bool,
        example_rules: Vec<Rule>,
    ) {
        assert_eq!(order.check(&example_rules), is_valid)
    }

    #[rstest]
    #[case(
        PrintOrder::parse("75,47,61,53,29"),
        PrintOrder::parse("75,47,61,53,29")
    )]
    #[case(
        PrintOrder::parse("97,61,53,29,13"),
        PrintOrder::parse("97,61,53,29,13")
    )]
    #[case(PrintOrder::parse("75,29,13"), PrintOrder::parse("75,29,13"))]
    #[case(
        PrintOrder::parse("75,97,47,61,53"),
        PrintOrder::parse("97,75,47,61,53")
    )]
    #[case(PrintOrder::parse("61,13,29"), PrintOrder::parse("61,29,13"))]
    #[case(
        PrintOrder::parse("97,13,75,29,47"),
        PrintOrder::parse("97,75,47,29,13")
    )]
    fn test_reorder(
        #[case] order: PrintOrder,
        #[case] valid_order: PrintOrder,
        example_rules: Vec<Rule>,
    ) {
        let order = order.reorder(&example_rules);
        assert_eq!(order, valid_order);
        assert!(order.check(&example_rules));
    }

    #[rstest]
    #[case(PrintOrder::parse("75,47,61,53,29"), 61)]
    #[case(PrintOrder::parse("97,61,53,29,13"), 53)]
    #[case(PrintOrder::parse("75,29,13"), 29)]
    #[case(PrintOrder::parse("75,97,47,61,53"), 0)]
    #[case(PrintOrder::parse("61,13,29"), 0)]
    #[case(PrintOrder::parse("97,13,75,29,47"), 0)]
    fn test_order_get_middle(
        #[case] order: PrintOrder,
        #[case] middle: u32,
        example_rules: Vec<Rule>,
    ) {
        assert_eq!(order.get_middle_if_valid(&example_rules), middle)
    }

    #[rstest]
    fn test_part_a() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(part_a(input), 143)
    }

    #[rstest]
    fn test_part_b() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(part_b(input), 123)
    }
}
