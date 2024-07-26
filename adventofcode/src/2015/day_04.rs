use std::fs;

use md5::{Digest, Md5};

fn main() {
    let input = fs::read_to_string("inputs/04.txt").expect("input file should exist");
    let prepared_input = input.trim().to_string();

    let part_one_solution = solve_part_one(&prepared_input);
    let part_two_solution = solve_part_two(&prepared_input);

    println!("Solution of Part One: {}", part_one_solution);
    println!("Solution of Part Two: {}", part_two_solution);
}

fn solve_part_one(secret_key: &String) -> i64 {
    advent_coin_miner(secret_key.to_string(), "00000".to_string())
}

fn solve_part_two(secret_key: &String) -> i64 {
    advent_coin_miner(secret_key.to_string(), "000000".to_string())
}

fn advent_coin_miner(secret_key: String, required_prefix: String) -> i64 {
    let mut number = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", secret_key, number));

        let hash = format!("{:x}", hasher.finalize());
        if hash.starts_with(&required_prefix) {
            break;
        }

        number += 1;
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advent_coin_miner() {
        assert_eq!(
            advent_coin_miner("abcdef".to_string(), "00000".to_string()),
            609043
        );
        assert_eq!(
            advent_coin_miner("pqrstuv".to_string(), "00000".to_string()),
            1048970
        );
    }
}
