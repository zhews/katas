use std::{fs, num::ParseIntError};

fn main() {
    let input = fs::read_to_string("inputs/02.txt").expect("input file should exist");
    let prepared_input = prepare_input(input).expect("input should be prepareable");

    let part_one_solution = solve_part_one(&prepared_input);
    let part_two_solution = solve_part_two(&prepared_input);

    println!("Solution of Part One: {}", part_one_solution);
    println!("Solution of Part Two: {}", part_two_solution);
}

const DIMENSIONS_COUNT: usize = 3;
const DIMENSIONS_SEPARATOR: &str = "x";
const LENGTH_INDEX: usize = 0;
const WIDTH_INDEX: usize = 1;
const HEIGHT_INDEX: usize = 2;

fn prepare_input(input: String) -> Result<Vec<(i64, i64, i64)>, ParseIntError> {
    let mut dimensions_list: Vec<(i64, i64, i64)> = Vec::new();

    for line in input.lines() {
        let dimensions: Vec<String> = line
            .splitn(DIMENSIONS_COUNT, DIMENSIONS_SEPARATOR)
            .map(String::from)
            .collect();

        let length: i64 = dimensions
            .get(LENGTH_INDEX)
            .expect("length dimension should be present")
            .parse()?;
        let width: i64 = dimensions
            .get(WIDTH_INDEX)
            .expect("width dimension should be present")
            .parse()?;
        let height: i64 = dimensions
            .get(HEIGHT_INDEX)
            .expect("height dimension should be present")
            .parse()?;

        dimensions_list.push((length, width, height))
    }

    Ok(dimensions_list)
}

fn solve_part_one(dimensions_list: &Vec<(i64, i64, i64)>) -> i64 {
    let mut wrapping_paper_amount: i64 = 0;
    for (length, width, height) in dimensions_list {
        let lw_side = length * width;
        let wh_side = width * height;
        let hl_side = height * length;
        let sides = [lw_side, wh_side, hl_side];

        let surface = 2 * lw_side + 2 * wh_side + 2 * hl_side;
        wrapping_paper_amount += surface;

        let smallest_side = sides.iter().min().expect("smallest side should be present");
        wrapping_paper_amount += smallest_side;
    }
    wrapping_paper_amount
}

fn solve_part_two(dimensions_list: &Vec<(i64, i64, i64)>) -> i64 {
    let mut ribbon_amount: i64 = 0;
    for (length, width, height) in dimensions_list {
        let lw_perimeter = 2 * (length + width);
        let wh_perimeter = 2 * (width + height);
        let hl_perimeter = 2 * (height + length);
        let perimeters = [lw_perimeter, wh_perimeter, hl_perimeter];

        let volume = length * width * height;
        ribbon_amount += volume;

        let smallest_perimeter = perimeters
            .iter()
            .min()
            .expect("smallest perimeter should be present");
        ribbon_amount += smallest_perimeter;
    }
    ribbon_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(
            solve_part_one(&prepare_input("2x3x4".to_string()).unwrap()),
            58
        );
        assert_eq!(
            solve_part_one(&prepare_input("1x1x10".to_string()).unwrap()),
            43
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            solve_part_two(&prepare_input("2x3x4".to_string()).unwrap()),
            34
        );
        assert_eq!(
            solve_part_two(&prepare_input("1x1x10".to_string()).unwrap()),
            14
        );
    }
}
