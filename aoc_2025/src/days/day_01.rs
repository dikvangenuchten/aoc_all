pub fn solve_day(input_file: &str) -> (u32, u32) {
    let instructions = parse(input_file);
    let a = part_a(&instructions);
    let b = part_b(&instructions);
    (a, b)
}

pub fn part_a(instructions: &[Instruction]) -> u32 {
    instructions.iter().fold(
        (50, 0),
        |mut state: (i32, u32), instruction| {
            state.0 += instruction.number;
            state.0 = state.0.rem_euclid(100);
            if state.0 == 0 {
                state.1 += 1;
            }
            state
        }
    ).1
}

pub fn part_b(instructions: &[Instruction]) -> u32 {
    instructions.iter().fold(
        (50, 0),
        |mut state: (i32, u32), instruction| {
            let init_state = state.clone();
            state = step(init_state.clone(), instruction);
            state
        }
    ).1
}


fn step(mut state: (i32, u32), instruction: &Instruction) -> (i32, u32) {
    let movement = instruction.number;

    let direction = movement.signum();
    
    let is_positive = (direction > 0) as i32;
    let is_at_zero = (state.0 == 0) as i32;
    
    let distance_to_zero = is_positive * (100 * is_at_zero + (100 - state.0) * (1 - is_at_zero)) +
                          (1 - is_positive) * (100 * is_at_zero + state.0 * (1 - is_at_zero));
    
    let can_cross = (movement.abs() >= distance_to_zero) as i32;
    let crossings = movement.abs().signum() * can_cross * (1 + (movement.abs() - distance_to_zero) / 100);
    
    state.1 += crossings as u32;
    state.0 = (state.0 + instruction.number).rem_euclid(100);
    state
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction character: {}", c),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    number: i32,
}

impl Instruction {
    fn new(direction: Direction, number: u32) -> Self {
        if direction == Direction::Left {
            Instruction { number: -(number as i32) }
        } else {
            Instruction { number: number as i32 }
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let dir_char = line.chars().next().unwrap();
            let direction = Direction::from(dir_char);
            let number: u32 = line[1..].parse().unwrap();
            Instruction::new(direction, number)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::days::read_test_day_input;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_parse() {
        let input_file = read_test_day_input("01");
        let input = parse(&input_file);
        assert_eq!(
            input,
            vec![
                Instruction::new(Direction::Left, 68),
                Instruction::new(Direction::Left, 30),
                Instruction::new(Direction::Right, 48),
                Instruction::new(Direction::Left, 5),
                Instruction::new(Direction::Right, 60),
                Instruction::new(Direction::Left, 55),
                Instruction::new(Direction::Left, 1),
                Instruction::new(Direction::Left, 99),
                Instruction::new(Direction::Right, 14),
                Instruction::new(Direction::Left, 82),
            ]
        )

    }

    #[rstest]
    fn test_day_01_a() {
        let input_file = read_test_day_input("01");
        let instructions = parse(&input_file);
        let result = part_a(&instructions);
        assert_eq!(result, 3)
    }

    // #[rstest]
    // fn test_day_01_b() {
    //     let input_file = read_test_day_input("01");
    //     let instructions = parse(&input_file);
    //     let result = part_b(&instructions);
    //     assert_eq!(result, 6)
    // }

    #[rstest]
    #[case((50, 0), Instruction::new(Direction::Left, 60), (90, 1))]
    #[case((50, 0), Instruction::new(Direction::Left, 160), (90, 2))]
    #[case((50, 0), Instruction::new(Direction::Left, 1060), (90, 11))]
    #[case((50, 0), Instruction::new(Direction::Right, 60), (10, 1))]
    #[case((50, 0), Instruction::new(Direction::Right, 160), (10, 2))]
    #[case((50, 0), Instruction::new(Direction::Right, 1060), (10, 11))]
    #[case((50, 0), Instruction::new(Direction::Left, 50), (0, 1))]
    #[case((50, 0), Instruction::new(Direction::Left, 150), (0, 2))]
    #[case((99, 0), Instruction::new(Direction::Left, 98), (1, 0))]
    #[case((62, 13), Instruction::new(Direction::Left, 62), (0, 14))]
    #[case((99, 0), Instruction::new(Direction::Right, 1), (0, 1))]
    #[case((0, 14), Instruction::new(Direction::Left, 59), (41, 14))]
    #[case((0, 13), Instruction::new(Direction::Left, 100), (0, 14))]
    #[case((0, 13), Instruction::new(Direction::Right, 100), (0, 14))]
    fn test_step(#[case] state: (i32, u32), #[case] instruction: Instruction, #[case] expected: (i32, u32)) {
        println!("Testing step from {:?} with instruction {:?}", state, instruction);
        let result = step(state,&instruction);
        assert_eq!(result, expected)
    }
}