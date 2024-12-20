#![warn(clippy::all, clippy::pedantic)]

use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let grid = preamble(file_path);

    println!("Day 16 Part One answer is {}", get_result_part_one(&grid));
    println!("Day 16 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> Grid {
    let file_content: String = get_file_contents(file_path);
    // Grid::new(file_content)
}

fn get_result_part_one(grid: &Grid) -> usize {
    // let (_, cost) = find_cheapest_path(grid).unwrap();
    cost as usize
}

fn get_result_part_two(grid: &Grid) -> usize {
    // count_tiles_of_all_cheapest_paths(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_sample_part_one() {
    //     let grid = preamble("sample");
    //     assert_eq!(get_result_part_one(&grid), 11048);
    // }

    // #[test]
    // fn test_input_part_one() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_one(&grid), 72400);
    // }

    // #[test]
    // fn test_sample_part_two() {
    //     let grid = preamble("sample");
    //     assert_eq!(get_result_part_two(&grid), 64);
    // }

    // #[test]
    // fn test_input_part_two() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_two(&grid), 435);
    // }
}
