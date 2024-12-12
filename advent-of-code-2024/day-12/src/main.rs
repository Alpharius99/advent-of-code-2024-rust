use std::time::Instant;
use utils::{get_file_contents, get_2d_array};
use ndarray::Array2;

const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Above
    (0, -1), // Left
    (0, 1),  // Right
    (1, 0),  // Below
];

fn main() {
    let start_time = Instant::now();
    let grid = preamble();


    println!("Day 12 Part One answer is {}", get_result_part_one(&grid));
    println!("Day 12 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble() -> Array2<usize> {
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let file_content: String = get_file_contents(file_path);
    get_2d_array(&file_content)
}

fn get_result_part_one(grid: &Array2<usize>) -> usize {
    let mut result: usize = 0;

    let fields: Vec<(char, usize)> = Vec::new();

    result
}

fn get_result_part_two(grid: &Array2<usize>) -> usize {
    let mut result: usize = 0;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 1_930);
    }

    #[test]
    fn test_sample_part_two() {
        let input = get_file_contents("sample");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 1930);
    }

    #[test]
    fn test_input_part_one() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_one(&data), 1930);
    }

    #[test]
    fn test_input_part_two() {
        let input = get_file_contents("input");
        let data = get_1d_vector(&input);
        assert_eq!(get_result_part_two(&data), 1930);
    }
}
