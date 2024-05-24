use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/01.txt").expect("input file should exist");
    let prepared_input = prepare_input(input);

    let part_one_solution = solve_part_one(&prepared_input);
    let part_two_solution =
        solve_part_two(&prepared_input).expect("input should lead to a solution");

    println!("Solution of Part One: {}", part_one_solution);
    println!("Solution of Part Two: {}", part_two_solution);
}

enum Instruction {
    Up,
    Down,
}

fn prepare_input(input: String) -> Vec<Instruction> {
    input
        .chars()
        .filter_map(|c| match c {
            '(' => Some(Instruction::Up),
            ')' => Some(Instruction::Down),
            _ => None,
        })
        .collect()
}

fn solve_part_one(instructions: &Vec<Instruction>) -> i64 {
    let initial_floor_value: i64 = 0;
    instructions
        .iter()
        .fold(initial_floor_value, |a, e| match *e {
            Instruction::Up => a + 1,
            Instruction::Down => a - 1,
        })
}

const BASEMENT_FLOOR: i64 = -1;
fn solve_part_two(instructions: &Vec<Instruction>) -> Option<i64> {
    let mut current_floor: i64 = 0;
    for (index, instruction) in instructions.iter().enumerate() {
        match *instruction {
            Instruction::Up => current_floor += 1,
            Instruction::Down => current_floor -= 1,
        }
        if current_floor == BASEMENT_FLOOR {
            return Some(index as i64 + 1); // solution is the 1-based index
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(&prepare_input("(())".to_string())), 0);
        assert_eq!(solve_part_one(&prepare_input("()()".to_string())), 0);
        assert_eq!(solve_part_one(&prepare_input("(((".to_string())), 3);
        assert_eq!(solve_part_one(&prepare_input("(()(()(".to_string())), 3);
        assert_eq!(solve_part_one(&prepare_input("))(((((".to_string())), 3);
        assert_eq!(solve_part_one(&prepare_input("())".to_string())), -1);
        assert_eq!(solve_part_one(&prepare_input("))(".to_string())), -1);
        assert_eq!(solve_part_one(&prepare_input(")))".to_string())), -3);
        assert_eq!(solve_part_one(&prepare_input(")())())".to_string())), -3);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(&prepare_input(")".to_string())), Some(1));
        assert_eq!(solve_part_two(&prepare_input("()())".to_string())), Some(5));
    }
}
