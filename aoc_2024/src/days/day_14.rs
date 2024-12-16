use std::str::FromStr;

pub fn solve_day(input_file: &str) -> (u32, u32) {
    let map_size = (101, 103);
    let robots = parse_input(input_file);
    let a = part_a(&robots, &map_size);
    let b = part_b(&robots);
    (a, b)
}

fn parse_input(input_file: &str) -> Vec<Robot> {
    input_file
        .trim()
        .lines()
        .map(|r| r.parse().expect("Invalid input"))
        .collect()
}

fn part_a(robots: &[Robot], map_size: &(i32, i32)) -> u32 {
    let (half_x, half_y) = (map_size.0 / 2, map_size.1 / 2);
    robots
        .iter()
        .map(|r| r.get_pos_in_n_sec(100, map_size))
        .fold([0, 0, 0, 0], |mut sum, (x, y)| {
            if x < half_x && y < half_y {
                sum[0] += 1;
            } else if x > half_x && y < half_y {
                sum[1] += 1;
            } else if x < half_x && y > half_y {
                sum[2] += 1;
            } else if x > half_x && y > half_y {
                sum[3] += 1;
            }
            sum
        })
        .iter()
        .product()
}

fn part_b(robots: &[Robot]) -> u32 {
    let map_size = (101, 103);

    for i in 0..10000 {
        let mut matrix = [[false; 101]; 103];

        for r in robots {
            let (x, y) = r.get_pos_in_n_sec(i, &map_size);
            matrix[y as usize][x as usize] = true
        }

        // To visualize the christmas tree uncomment below
        // let map: String = matrix
        //     .iter()
        //     .map(|row| row.iter().collect::<String>() + "|\n")
        //     .collect();

        // if map.contains("########") {
        //     // dbg!(map);
        //     return i as u32;
        // }

        // Check for a straight horizontal line which is longer then 7 robots
        if matrix.iter().any(|row| {
            row.iter()
                .try_fold((false, 0), |(found_pattern, cur_count), char| {
                    match (found_pattern, char) {
                        (true, _) => Err((true, 0)),
                        (false, true) => Ok((cur_count > 7, cur_count + 1)),
                        (false, false) => Ok((false, 0)),
                    }
                })
                .is_err_and(|(b, _)| b)
        }) {
            return i as u32;
        }
    }
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    pos: (i32, i32),
    v: (i32, i32),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl FromStr for Robot {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((pos, v)) = s.split_once(" ") {
            let (x, y) = pos
                .strip_prefix("p=")
                .ok_or(ParseError)?
                .split_once(",")
                .ok_or(ParseError)?;
            let pos = (
                x.parse::<i32>().map_err(|_| ParseError {})?,
                y.parse::<i32>().map_err(|_| ParseError {})?,
            );

            let (x, y) = v
                .strip_prefix("v=")
                .ok_or(ParseError)?
                .split_once(",")
                .ok_or(ParseError)?;
            let v = (
                x.parse::<i32>().map_err(|_| ParseError {})?,
                y.parse::<i32>().map_err(|_| ParseError {})?,
            );

            return Ok(Robot { pos, v });
        }
        Err(ParseError)
    }
}

impl Robot {
    fn get_pos_in_n_sec(&self, n: i32, map_size: &(i32, i32)) -> (i32, i32) {
        (
            (self.pos.0 + n * self.v.0).rem_euclid(map_size.0),
            (self.pos.1 + n * self.v.1).rem_euclid(map_size.1),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("p=0,4 v=3,-3", Robot {pos: (0, 4), v: (3, -3) })]
    #[case("p=-0,4 v=3,-3", Robot {pos: (0, 4), v: (3, -3) })]
    #[case("p=-0,-4 v=3,-3", Robot {pos: (0, -4), v: (3, -3) })]
    fn test_parse(#[case] s: &str, #[case] robot: Robot) {
        assert_eq!(s.parse(), Ok(robot))
    }

    #[rstest]
    #[case("p=0,4 v=3,-3", 0, (0, 4))]
    #[case("p=0,4 v=3,-3", 1, (3, 1))]
    #[case("p=0,4 v=3,-3", 2, (6, 5))]
    fn test_get_pos_in_n_sec(#[case] robot: Robot, #[case] n: i32, #[case] pos: (i32, i32)) {
        assert_eq!(robot.get_pos_in_n_sec(n, &(11, 7)), pos)
    }

    #[rstest]
    #[case(
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        12
    )]
    fn test_part_a(#[case] input: &str, #[case] expected: u32) {
        let robots = parse_input(input);
        assert_eq!(part_a(&robots, &(11, 7)), expected);
    }
}
