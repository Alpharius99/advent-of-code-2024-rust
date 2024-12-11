use std::fs;
use std::str::FromStr;

pub fn get_file_contents(filename: &str) -> String {
    let mut content =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Failed to read file {}", filename));

    // Check for and remove the BOM
    if content.starts_with('\u{feff}') {
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    content
}

pub fn get_1d_vector(input: &str) -> Vec<usize> {
    input
        .split_whitespace() // Split the string by whitespace
        .filter_map(|s| s.parse::<usize>().ok()) // Parse each part to usize, filtering out invalid entries
        .collect() // Collect into a Vec<usize>
}

pub fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn convert_string_vector_to_integer_vector(vector: Vec<&str>) -> Vec<i32> {
    vector
        .iter()
        .map(|s| s.parse::<i32>().expect("Failed to parse string as integer"))
        .collect()
}

pub fn string_to_int32(s: &str) -> i32 {
    s.parse::<i32>()
        .expect("Failed to parse string as integer 32")
}

pub fn string_to_int64(s: &str) -> i64 {
    s.parse::<i64>()
        .expect("Failed to parse string as integer 64")
}

pub fn string_to_int<T>(input: &str) -> Result<T, T::Err>
where
    T: FromStr,
{
    input.parse::<T>()
}

pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        let joined: String = row.into_iter().collect();
        println!("{:?}", joined);
    }
}

pub fn join_integers<T>(numbers: &Vec<T>, delimiter: &str) -> String
where
    T: FromStr + ToString,
{
    numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(delimiter)
}
