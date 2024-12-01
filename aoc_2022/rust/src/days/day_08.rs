use std::cmp::max;

use itertools::Itertools;

pub fn solve(input_str: &str) -> (u32, usize) {
    let height_map = parse_input(input_str);
    let part_1 = solve_part_1(&height_map);
    let part_2 = solve_part_2(&height_map);
    (part_1, part_2)
}

fn solve_part_1(height_map: &Vec<Vec<u8>>) -> u32 {
    let visibility_map = create_visibility_map(height_map);
    visibility_map.iter().fold(0, |sum, line| {
        sum + line.iter().fold(0, |sum, vis| sum + *vis as u32)
    })
}

fn solve_part_2(height_map: &Vec<Vec<u8>>) -> usize {
    let mut max_scenery = 0;
    for i in 1..(height_map.len() - 1) {
        for j in 1..(height_map[0].len() - 1) {
            max_scenery = max(max_scenery, calculate_scenary_score(i, j, height_map));
        }
    }
    max_scenery
}

fn calculate_scenary_score(i: usize, j: usize, height_map: &[Vec<u8>]) -> usize {
    let tree = height_map[i][j];
    let mut up = 0;
    for k in (0..i).rev() {
        up = i - k;
        if height_map[k][j] >= tree {
            break;
        }
    }

    let mut down = 0;
    for (k, tree_line) in height_map.iter().enumerate().skip(i + 1) {
        down = k - i;
        if tree_line[j] >= tree {
            break;
        }
    }

    let mut left = 0;
    let tree_line = &height_map[i];
    for k in (0..j).rev() {
        left = j - k;
        if tree_line[k] >= tree {
            break;
        }
    }

    let mut right = 0;
    let tree_line = &height_map[i];
    for (k, tree_view) in tree_line
        .iter()
        .enumerate()
        .take(height_map[i].len())
        .skip(j + 1)
    {
        right = k - j;
        if tree_view >= &tree {
            break;
        }
    }
    up * left * down * right
}

fn parse_input(input_str: &str) -> Vec<Vec<u8>> {
    input_str
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|tree| tree.to_digit(10).unwrap().try_into().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn create_visibility_map_lr(height_map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut left_vis = Vec::with_capacity(height_map.len());
    let mut right_vis = Vec::with_capacity(height_map.len());
    for tree_line in height_map {
        let mut line_vis = Vec::<bool>::with_capacity(tree_line.len());
        let mut max_height = 0;
        for tree in tree_line {
            if tree >= &max_height {
                line_vis.push(true);
                max_height = tree + 1;
            } else {
                line_vis.push(false);
            }
        }
        left_vis.push(line_vis);

        let mut line_vis = Vec::<bool>::with_capacity(tree_line.len());
        max_height = 0;
        for tree in tree_line.iter().rev() {
            if tree >= &max_height {
                line_vis.push(true);
                max_height = tree + 1;
            } else {
                line_vis.push(false);
            }
        }
        line_vis.reverse();
        right_vis.push(line_vis);
    }
    combine_visibility_map(left_vis, right_vis)
}

fn combine_visibility_map(left: Vec<Vec<bool>>, right: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    left.into_iter()
        .zip(right)
        .map(|(left, right)| {
            left.into_iter()
                .zip(right)
                .map(|(left, right)| left || right)
                .collect_vec()
        })
        .collect_vec()
}

fn create_visibility_map_v(height_map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut top: Vec<Vec<bool>> = Vec::with_capacity(height_map.len());
    top.resize_with(height_map.len(), || {
        let mut line = Vec::with_capacity(height_map[0].len());
        line.resize(height_map[0].len(), false);
        line
    });
    let mut bot: Vec<Vec<bool>> = Vec::with_capacity(height_map.len());
    bot.resize_with(height_map.len(), || {
        let mut line = Vec::with_capacity(height_map[0].len());
        line.resize(height_map[0].len(), false);
        line
    });

    for j in 0..height_map.get(0).unwrap().len() {
        let mut max_height = 0;
        for i in 0..height_map.len() {
            let tree = height_map[i][j];
            if tree >= max_height {
                top[i][j] = true;
                max_height = tree + 1;
            } else {
                top[i][j] = false;
            }
        }

        max_height = 0;
        for i in (0..height_map.len()).rev() {
            let tree = height_map[i][j];
            if tree >= max_height {
                bot[i][j] = true;
                max_height = tree + 1;
            } else {
                bot[i][j] = false;
            }
        }
    }
    combine_visibility_map(top, bot)
}

fn create_visibility_map(height_map: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    combine_visibility_map(
        create_visibility_map_lr(height_map),
        create_visibility_map_v(height_map),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("30373\n25512\n65332\n33549\n35390\n")]
    fn test_parse_input(#[case] input_str: &str) {
        assert_eq!(
            parse_input(input_str),
            vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ]
        )
    }

    #[rstest]
    #[case("30373\n25512\n65332\n33549\n35390\n")]
    fn test_solve_part_1(#[case] input_str: &str) {
        let height_map = parse_input(input_str);
        assert_eq!(solve_part_1(&height_map), 21)
    }

    #[rstest]
    #[case("30373\n25512\n65332\n33549\n35390\n")]
    fn test_solve_part_2(#[case] input_str: &str) {
        let height_map = parse_input(input_str);
        assert_eq!(solve_part_2(&height_map), 8)
    }

    #[rstest]
    #[case(1, 2, 4)]
    #[case(3, 2, 8)]
    fn test_calculate_scenary_score(
        #[case] i: usize,
        #[case] j: usize,
        #[case] expected_score: usize,
    ) {
        let height_map = parse_input("30373\n25512\n65332\n33549\n35390\n");
        assert_eq!(calculate_scenary_score(i, j, &height_map), expected_score)
    }

    #[rstest]
    fn test_create_visibility_map() {
        let example_input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let expected_visibility = vec![
            vec![true, true, true, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, true, true, true],
        ];
        assert_eq!(create_visibility_map(&example_input), expected_visibility);
    }

    #[rstest]
    fn test_create_visibility_map_lr() {
        let example_input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let expected_visibility = vec![
            vec![true, false, false, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, false, true, true],
        ];
        assert_eq!(
            create_visibility_map_lr(&example_input),
            expected_visibility
        );
    }

    #[rstest]
    fn test_create_visibility_map_v() {
        let example_input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let expected_visibility = vec![
            vec![true, true, true, true, true],
            vec![false, true, true, false, false],
            vec![true, false, false, false, false],
            vec![false, false, true, false, true],
            vec![true, true, true, true, true],
        ];
        assert_eq!(create_visibility_map_v(&example_input), expected_visibility);
    }
}
