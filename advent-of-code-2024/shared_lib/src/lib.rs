use ndarray::Array2;
use std::fs;
use std::str::FromStr;

pub fn get_file_contents(filename: &str) -> String {
    let mut content =
        fs::read_to_string(filename).expect(format!("Failed to read file {}", filename).as_str());

    // Check for and remove the BOM
    if content.starts_with('\u{feff}') {
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    content
}

pub fn get_2d_array(input: &str) -> Array2<usize> {
    // Split the input into rows
    let rows: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars() // Iterate over characters in each line
                .filter(|c| c.is_digit(10)) // Keep only digit characters
                .map(|c| c.to_digit(10).unwrap() as usize) // Convert char to i32
                .collect::<Vec<_>>() // Collect into a vector
        })
        .collect();

    // Determine the shape of the array
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    // Flatten the rows into a single vector
    let flattened: Vec<usize> = rows.into_iter().flatten().collect();

    // Convert the flattened vector into an Array2
    Array2::from_shape_vec((num_rows, num_cols), flattened).unwrap()
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
