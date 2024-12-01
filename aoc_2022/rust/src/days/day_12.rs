use std::cmp::min;

type MapObjective = (Vec<Vec<u32>>, (usize, usize), (usize, usize));

pub fn solve(input: &str) -> (u32, u32) {
    let (height_map, start, end) = parse_input(input);
    let distance_map = convert_to_distance_map(&height_map, &end);
    let part_1 = solve_part_1(&distance_map, &start);
    let part_2 = solve_part_2(&distance_map, &height_map);
    (part_1, part_2)
}

fn solve_part_1(distance_map: &[Vec<u32>], start: &(usize, usize)) -> u32 {
    distance_map[start.0][start.1]
}

fn solve_part_2(distance_map: &[Vec<u32>], height_map: &[Vec<u32>]) -> u32 {
    let mut min_distance = u32::MAX;
    for (i, j) in height_map.iter().enumerate().flat_map(|(i, line)| {
        line.iter()
            .enumerate()
            .filter(|(_, h)| h == &&0)
            .map(move |(j, _)| (i, j))
    }) {
        min_distance = min(min_distance, distance_map[i][j]);
    }
    min_distance
}

fn parse_input(input_str: &str) -> MapObjective {
    let mut map: Vec<Vec<u32>> = input_str
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(36).unwrap() - 10).collect())
        .collect();

    let start = input_str
        .split('\n')
        .enumerate()
        .fold((0, 0), |loc, (i, line)| {
            line.chars().enumerate().fold(loc, |loc, (j, c)| {
                if c == 'S' {
                    (i as usize, j as usize)
                } else {
                    loc
                }
            })
        });
    let end = input_str
        .split('\n')
        .enumerate()
        .fold((0, 0), |loc, (i, line)| {
            line.chars().enumerate().fold(loc, |loc, (j, c)| {
                if c == 'E' {
                    (i as usize, j as usize)
                } else {
                    loc
                }
            })
        });
    map[start.0][start.1] = 0;
    map[end.0][end.1] = 25;
    (map, start, end)
}

fn convert_to_distance_map(map: &Vec<Vec<u32>>, end: &(usize, usize)) -> Vec<Vec<u32>> {
    let mut distance_map = vec![vec![u32::MAX - 1; map[0].len()]; map.len()];
    distance_map[end.0][end.1] = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                let h = map[i][j];
                let mut shortest_distance = u32::MAX;
                if 0 < i && h + 2 > map[i - 1][j] {
                    shortest_distance = min(shortest_distance, distance_map[i - 1][j] + 1);
                }

                if (i + 1) < map.len() && h + 2 > map[i + 1][j] {
                    shortest_distance = min(shortest_distance, distance_map[i + 1][j] + 1);
                }

                if (j + 1) < map[0].len() && h + 2 > map[i][j + 1] {
                    shortest_distance = min(shortest_distance, distance_map[i][j + 1] + 1);
                }

                if 0 < j && h + 2 > map[i][j - 1] {
                    shortest_distance = min(shortest_distance, distance_map[i][j - 1] + 1);
                }

                if shortest_distance < distance_map[i][j] {
                    changed = true;
                    distance_map[i][j] = shortest_distance
                }
            }
        }
    }

    distance_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example_input_str() -> &'static str {
        "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi"
    }

    #[fixture]
    fn example_map(example_input_str: &str) -> MapObjective {
        parse_input(example_input_str)
    }

    #[rstest]
    fn test_parse_input(example_map: MapObjective) {
        assert_eq!(
            example_map,
            (
                vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12,],
                    vec![0, 1, 2, 17, 24, 23, 23, 11,],
                    vec![0, 2, 2, 18, 25, 25, 23, 10,],
                    vec![0, 2, 2, 19, 20, 21, 22, 9,],
                    vec![0, 1, 3, 4, 5, 6, 7, 8,],
                ],
                (0, 0),
                (2, 5)
            )
        )
    }

    #[fixture]
    fn example_distance_map(example_map: MapObjective) -> Vec<Vec<u32>> {
        convert_to_distance_map(&example_map.0, &example_map.2)
    }

    #[rstest]
    fn test_convert_to_distance_map(example_distance_map: Vec<Vec<u32>>) {
        assert_eq!(
            example_distance_map,
            vec![
                vec![31, 30, 29, 12, 13, 14, 15, 16],
                vec![30, 29, 28, 11, 2, 3, 4, 17],
                vec![31, 28, 27, 10, 1, 0, 5, 18],
                vec![30, 27, 26, 9, 8, 7, 6, 19],
                vec![29, 28, 25, 24, 23, 22, 21, 20],
            ],
        );
    }

    #[rstest]
    fn test_solve_part_1(example_distance_map: Vec<Vec<u32>>, example_map: MapObjective) {
        assert_eq!(solve_part_1(&example_distance_map, &example_map.1), 31);
    }

    #[rstest]
    fn test_solve_part_2(example_distance_map: Vec<Vec<u32>>, example_map: MapObjective) {
        assert_eq!(solve_part_2(&example_distance_map, &example_map.0), 29);
    }
}
