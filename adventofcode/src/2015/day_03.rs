use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("inputs/03.txt").expect("input file should exist");
    let prepared_input = prepare_input(input);

    let part_one_solution = solve_part_one(&prepared_input);
    let part_two_solution = solve_part_two(&prepared_input);

    println!("Solution of Part One: {}", part_one_solution);
    println!("Solution of Part Two: {}", part_two_solution);
}

#[derive(Debug)]
enum Move {
    North,
    East,
    South,
    West,
}

fn prepare_input(input: String) -> Vec<Move> {
    input
        .chars()
        .filter_map(|m| match m {
            '^' => Some(Move::North),
            '>' => Some(Move::East),
            'v' => Some(Move::South),
            '<' => Some(Move::West),
            _ => None,
        })
        .collect()
}

fn solve_part_one(moves: &Vec<Move>) -> i64 {
    let mut coordinates = (0, 0);
    let mut history: HashMap<(i64, i64), i64> = HashMap::from([(coordinates, 1)]);
    for r#move in moves {
        match r#move {
            Move::North => coordinates.1 += 1,
            Move::East => coordinates.0 += 1,
            Move::South => coordinates.1 -= 1,
            Move::West => coordinates.0 -= 1,
        }
        history
            .entry(coordinates)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    history.len() as i64
}
fn solve_part_two(moves: &Vec<Move>) -> i64 {
    let santa_coordinates = (0, 0);
    let santa_moves: Vec<&Move> = moves.iter().step_by(2).collect();

    let robot_santa_coordinates = (0, 0);
    let robot_santa_moves: Vec<&Move> = moves.iter().skip(1).step_by(2).collect();

    let mut history: HashMap<(i64, i64), i64> = HashMap::from([(santa_coordinates, 2)]);

    for (mut coordinates, moves) in [
        (santa_coordinates, santa_moves),
        (robot_santa_coordinates, robot_santa_moves),
    ] {
        for r#move in moves {
            match r#move {
                Move::North => coordinates.1 += 1,
                Move::East => coordinates.0 += 1,
                Move::South => coordinates.1 -= 1,
                Move::West => coordinates.0 -= 1,
            };
            history
                .entry(coordinates)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    history.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(&prepare_input(">".to_string())), 2);
        assert_eq!(solve_part_one(&prepare_input("^>v<".to_string())), 4);
        assert_eq!(solve_part_one(&prepare_input("^v^v^v^v^v".to_string())), 2);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(&prepare_input("^v".to_string())), 3);
        assert_eq!(solve_part_two(&prepare_input("^>v<".to_string())), 3);
        assert_eq!(solve_part_two(&prepare_input("^v^v^v^v^v".to_string())), 11);
    }
}
