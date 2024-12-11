#![warn(clippy::all, clippy::pedantic)]
use std::time::Instant;
use utils::{get_1d_vector, get_file_contents};

fn main() {
    let start_time = Instant::now();
    let data = preamble();

    println!("Day 11 Part One answer is {}", get_result_part_one(&data));
    println!("Day 11 Part Two answer is {}", get_result_part_two(&data));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble() -> Vec<usize> {
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let file_content: String = get_file_contents(file_path);
    get_1d_vector(&file_content)
}

fn get_result_part_one(vec: &Vec<usize>) -> usize {
    0
}

fn get_result_part_two(vec: &Vec<usize>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 55312);
    }

    #[test]
    fn test_sample_part_two() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 81);
    }

    #[test]
    fn test_input_part_one() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 709);
    }

    #[test]
    fn test_input_part_two() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 1326);
    }
}
