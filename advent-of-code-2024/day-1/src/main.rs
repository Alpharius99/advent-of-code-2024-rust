use std::env;
use utils::{convert_string_vector_to_integer_vector, get_file_contents};

const FILE_PATH: &str = "input";

fn main() {
    println!(
        "Current working directory: {:?}",
        env::current_dir().unwrap()
    );
    let file_content: String = get_file_contents(FILE_PATH);

    let int_entries: Vec<i32> = get_numbers_from_file_content(&file_content);

    let left: Vec<i32> = get_sorted_vector(int_entries.clone(), false);
    let right: Vec<i32> = get_sorted_vector(int_entries.clone(), true);

    let distances: Vec<i32> = calculate_distances(&left, &right);

    let distance: i32 = sum_distances(distances);

    println!("Day 1 Part One answer is {}", distance);

    let score = calculate_similarity_score(&left, &right);

    println!("Day 1 Part Two answer is {}", score);
}

fn get_numbers_from_file_content(file_content: &str) -> Vec<i32> {
    let string_entries: Vec<&str> = file_content.split_whitespace().collect();

    convert_string_vector_to_integer_vector(string_entries)
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

fn calculate_distances(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let distances: Vec<i32> = left.iter().zip(right).map(|(a, b)| (b - a).abs()).collect();

    distances
}

fn sum_distances(distances: Vec<i32>) -> i32 {
    distances.iter().sum()
}

fn calculate_similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut score: i32 = 0;

    for item in left {
        score += item * get_count_by_number(right, *item);
    }

    score
}

fn get_count_by_number(vector: &Vec<i32>, target: i32) -> i32 {
    vector.iter().filter(|&&x| x == target).count() as i32
}
