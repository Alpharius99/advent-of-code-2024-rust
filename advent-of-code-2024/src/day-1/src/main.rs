
use utils::get_file_contents;

const FILE_PATH: &str = "input.txt";

fn main() {

    let file_content: String = get_file_contents(FILE_PATH);

    let int_entries: Vec<i32> = get_numbers_from_file_content(&file_content);

    let left: Vec<i32> = get_sorted_vector(int_entries.clone(), false);
    let right: Vec<i32> = get_sorted_vector(int_entries.clone(), true);

    let distances: Vec<i32> = calculate_distances(left, right);

    let distance: i32 = sum_distances(distances);

    println!("Day 1 answer is {}", distance);
}

fn get_numbers_from_file_content(file_content: &str) -> Vec<i32> {
    let string_entries: Vec<&str> = file_content.split_whitespace().collect();

    let int_entries: Vec<i32> = string_entries
        .iter()
        .map(|s| s.parse::<i32>().expect("Failed to parse string as integer"))
        .collect();

    int_entries
}

fn get_sorted_vector(vec: Vec<i32>, odd: bool) -> Vec<i32> {
    let mut result: Vec<_> = vec
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == if odd { 0 } else { 1 })
        .map(|(_, &value)| value)
        .collect();

    result.sort();

    result
}

fn calculate_distances(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let distances: Vec<i32> = left
        .iter()
        .zip(&right)
        .map(|(a, b)| (b - a).abs())
        .collect();

    distances
}

fn sum_distances(distances: Vec<i32>) -> i32 {
    distances.iter().sum()
}
