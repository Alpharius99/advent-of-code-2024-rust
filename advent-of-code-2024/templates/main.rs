#![warn(clippy::all, clippy::pedantic)]
use std::time::Instant;
use utils::{get_file_contents};

const FILE_PATH: &str = "input";

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);
    println!("Day 6 Part One answer is {}", 1);
    println!("Day 6 Part Two answer is {}", 2);

    println!("Execution time: {:.2?}", start_time.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let input = get_file_contents("sample");
        let array = get_2d_array(&input);
        assert_eq!(get_result_part_one(&array), 36);
    }

    #[test]
    fn test_sample_part_two() {
        let input = get_file_contents("sample");
        let array = get_2d_array(&input);
        assert_eq!(get_result_part_two(&array), 81);
    }

    #[test]
    fn test_input_part_one() {
        let input = get_file_contents("input");
        let array = get_2d_array(&input);
        assert_eq!(get_result_part_one(&array), 709);
    }

    #[test]
    fn test_input_part_two() {
        let input = get_file_contents("input");
        let array = get_2d_array(&input);
        assert_eq!(get_result_part_two(&array), 1326);
    }
}