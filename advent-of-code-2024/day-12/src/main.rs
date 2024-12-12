use std::time::Instant;
use utils::{get_1d_vector, get_file_contents};

fn main() {
    let start_time = Instant::now();
    let data = preamble();

    println!("Day 11 Part One answer is {}", get_result_part_one(&data.clone()));
    println!("Day 11 Part Two answer is {}", get_result_part_two(&data));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble() -> Vec<u64> {
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let file_content: String = get_file_contents(file_path);
    get_1d_vector(&file_content)
}

fn get_result_part_one(vec: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;

    result
}

fn get_result_part_two(vec: &Vec<u64>) -> u64 {
    let mut result: u64 = 0;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 55_312);
    }

    #[test]
    fn test_sample_part_two() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 65_601_038_650_482);
    }

    #[test]
    fn test_input_part_one() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 213_625);
    }

    #[test]
    fn test_input_part_two() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 252_442_982_856_820);
    }
}
