use std::fs;
const FILE_PATH: &str = "input.txt";

fn main() {
    let mut content = fs::read_to_string(FILE_PATH)
        .expect("Failed to read file");

    // Check for and remove the BOM
    if content.starts_with('\u{feff}') {
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    let string_entries: Vec<&str> = content.split_whitespace().collect();

    let int_entries: Vec<i32> = string_entries
        .iter()
        .map(|s| s.parse::<i32>().expect("Failed to parse string as integer"))
        .collect();

    let mut left: Vec<_> = int_entries
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == 0)
        .map(|(_, &value)| value)
        .collect();

    left.sort();

    let mut right: Vec<_> = int_entries
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 != 0)
        .map(|(_, &value)| value)
        .collect();

    right.sort();

    let distances: Vec<i32> = left
        .iter()
        .zip(&right)
        .map(|(a, b)| (b - a).abs())
        .collect();

    let all: Vec<String> = left
        .iter()
        .zip(&right)
        .zip(distances.iter())
        .map(|((a, b), d)| format!("left: {}, right : {}, distance: {}", a, b, d))
        .collect();

    let distance: i32 = distances.iter().sum();

//    for item in &all {
//        println!("{}", item);
//    }

    println!("Day 1 answer is {}", distance);
}
