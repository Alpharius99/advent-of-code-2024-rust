use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use regex::Regex;
use std::fs;
use std::str::FromStr;

pub const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Above
    (0, -1), // Left
    (0, 1),  // Right
    (1, 0),  // Below
];

pub const DIAGONALES: [(isize, isize); 4] = [
    (-1, -1), // Above-left
    (1, -1),  // Below-left
    (1, 1),   // Below-right
    (-1, 1),  // Above-right
];

pub const FULL_DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // Above
    (0, -1),  // Left
    (0, 1),   // Right
    (1, 0),   // Below
    (-1, -1), // Above-left
    (1, -1),  // Below-left
    (1, 1),   // Below-right
    (-1, 1),  // Above-right
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coords {
    pub row: isize,
    pub col: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Element {
    pub row: usize,
    pub col: usize,
    pub value: char,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub array: Array2<char>,
}

impl Grid {
    pub fn new(input: String) -> Grid {
        // Split the input into lines
        let lines: Vec<&str> = input.lines().collect();
        let rows: usize = lines.len();
        let cols: usize = lines.first().map_or(0, |line| line.len());

        // Create a 2D Array2 to hold the grid data
        let mut data: ArrayBase<OwnedRepr<char>, Ix2> = Array2::from_elem((rows, cols), ' ');

        // Fill the Array2 with characters from the input string
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                data[[row, col]] = ch;
            }
        }

        Grid {
            array: Array2::from_shape_fn((rows, cols), |(row, col)| data[[row, col]]),
        }
    }

    pub fn get_element(&self, row: usize, col: usize) -> Option<char> {
        if row < self.array.nrows() && col < self.array.ncols() {
            Some(self.array[[row, col]])
        } else {
            None
        }
    }
    
    pub fn set_element(&mut self, row: usize, col: usize, value: char) {
        if row < self.array.nrows() && col < self.array.ncols() {
            self.array[[row, col]] = value;
        }
    }

    pub fn get_element_by_value(&self, value: char) -> Option<(usize, usize)> {
        // Flatten the array and find the position of the target value
        let flat_index: usize = self.array.iter().position(|&x| x == value)?;

        // Convert flat index to 2D indices
        let (_rows, cols) = self.array.dim();
        Some((flat_index / cols, flat_index % cols))
    }
}

pub fn get_file_contents(filename: &str) -> String {
    let mut content =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("Failed to read file {}", filename));

    // Check for and remove the BOM
    if content.starts_with('\u{feff}') {
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    content
}

pub fn get_1d_vector(input: &str) -> Vec<u64> {
    input
        .split_whitespace() // Split the string by whitespace
        .filter_map(|s| s.parse::<u64>().ok()) // Parse each part to usize, filtering out invalid entries
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

pub fn get_2d_array_usize(input: &str) -> Array2<usize> {
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

pub fn get_2d_array_char(input: &str) -> Array2<char> {
    // Split the input into rows
    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars() // Iterate over characters in each line
                .collect::<Vec<_>>() // Collect into a vector
        })
        .collect();

    // Determine the shape of the array
    let num_rows = rows.len();
    let num_cols = rows[0].len();

    // Flatten the rows into a single vector
    let flattened: Vec<char> = rows.into_iter().flatten().collect();

    // Convert the flattened vector into an Array2
    Array2::from_shape_vec((num_rows, num_cols), flattened).unwrap()
}

pub fn get_value_by_regex(s: &str, re: &Regex) -> isize {
    re.captures(s)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<isize>()
        .unwrap()
}

pub fn find_coords_of_char(array: &Array2<char>, target: char) -> Option<Coords> {
    // Flatten the array and find the position of the target value
    let flat_index: usize = array.iter().position(|&x| x == target)?;

    // Convert flat index to 2D indices
    let (_rows, cols) = array.dim();
    Some(Coords {
        row: (flat_index / cols) as isize,
        col: (flat_index % cols) as isize,
    })
}
