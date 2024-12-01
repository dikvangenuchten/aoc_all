use std::collections::HashSet;

pub fn solve(input: &str) -> (u32, u32) {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);
    (part_1, part_2)
}

fn solve_part_1(rugsack_contents: &str) -> u32 {
    rugsack_contents
        .split('\n')
        .map(find_duplicate)
        .map(calc_priority)
        .sum::<u32>()
}

fn solve_part_2(input: &str) -> u32 {
    let elfs = input.split('\n').collect::<Vec<&str>>();
    elfs[0..elfs.len()]
        .chunks(3)
        .map(|group| find_group_badge(&group.join("\n")))
        .map(calc_priority)
        .sum()
}

fn find_duplicate(rugsack_contents: &str) -> char {
    let length = rugsack_contents.chars().count();
    let binding = rugsack_contents.chars().collect::<Vec<char>>();
    let (left, right) = binding.split_at(length / 2);

    **(HashSet::from_iter(left)
        .intersection(&HashSet::<&char>::from_iter(right))
        .into_iter()
        .next()
        .expect("All rugsacks should have one duplicate"))
}

fn calc_priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        return item as u32 - 96;
    }
    item as u32 - 38
}

fn find_group_badge(group: &str) -> char {
    *group
        .trim()
        .split('\n')
        .map(|items| HashSet::<char>::from_iter(items.chars()))
        .reduce(|l, r| l.intersection(&r).cloned().collect::<HashSet<char>>())
        .expect("This should work")
        .iter()
        .next()
        .expect("There should be one duplicate")
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 'p')]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L')]
    #[case("PmmdzqPrVvPwwTWBwg", 'P')]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v')]
    #[case("ttgJtRGJQctTZtZT", 't')]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 's')]
    fn test_find_duplicate(#[case] rugsack_contents: &str, #[case] duplicate: char) {
        assert_eq!(find_duplicate(rugsack_contents), duplicate)
    }

    #[rstest]
    #[case('p', 16)]
    #[case('L', 38)]
    #[case('P', 42)]
    #[case('v', 22)]
    #[case('t', 20)]
    #[case('s', 19)]
    #[case('r', 18)]
    #[case('Z', 52)]
    fn test_get_priority(#[case] item: char, #[case] priority: u32) {
        assert_eq!(calc_priority(item), priority)
    }

    #[rstest]
    #[case(157)]
    fn test_example_input_3p1(test_input: &str, #[case] priority_sum: u32) {
        assert_eq!(solve_part_1(test_input), priority_sum)
    }

    #[rstest]
    #[case(
        "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\n",
        'r'
    )]
    #[case(
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw",
        'Z'
    )]
    fn test_find_group_badge(#[case] group: &str, #[case] badge: char) {
        assert_eq!(find_group_badge(group), badge)
    }

    #[rstest]
    #[case(70)]
    fn test_part_2(test_input: &str, #[case] badge_priority_sum: u32) {
        assert_eq!(solve_part_2(test_input), badge_priority_sum);
    }
}
