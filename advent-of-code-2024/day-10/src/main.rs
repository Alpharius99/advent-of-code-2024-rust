#![warn(clippy::all, clippy::pedantic)]
use std::time::Instant;
use ndarray::Array2;
use utils::{get_file_contents};

const FILE_PATH: &str = "input";
const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // Above
    (0, -1), // Left
    (0, 1),  // Right
    (1, 0),  // Below
];
const TRAILHEAD: usize = 0;
const TOP: usize = 9;
const TOP_DOWN: bool = false;

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);
    let grid: Array2<usize> = get_2d_array(&file_content);

    println!("Day 10 Part One answer is {}", get_result_part_one(&grid));
    println!("Day 10 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn get_result_part_one(array: &Array2<usize>) -> usize {
    get_result(array, true)
}

fn get_result_part_two(array: &Array2<usize>) -> usize {
    get_result(array, false)
}

fn get_result(array: &Array2<usize>, unique: bool) -> usize {
    let starts = find_starts(array, TOP_DOWN);
    let mut result = 0;

    for start in starts {
        let mut targets = Vec::new();
        targets.push(start);

        match find_neighbors(array, targets, TOP_DOWN, unique) {
            Some(neighbors) => {
                result += neighbors.len();
            }
            None => (),
        }
    }

    result
}

fn find_starts(array: &Array2<usize>, reverse: bool) -> Vec<(usize, usize)> {
    let mut starts = Vec::new();
    let target;

    if reverse {
        target = TOP;
    } else {
        target = TRAILHEAD
    };

    // Iterate through the array with indexing
    for ((row, col), &value) in array.indexed_iter() {
        if value == target {
            starts.push((row, col)); // Collect the matching indices
        }
    }

    #[cfg(feature = "debug")]
    println!("Found {} of value {} ({:?})", starts.len(), target, starts);
    starts
}

fn find_neighbors(
    array: &Array2<usize>,
    targets: Vec<(usize, usize)>,
    reverse: bool,
    unique: bool,
) -> Option<Vec<(usize, usize)>> {
    let rows = array.nrows();
    let cols = array.ncols();
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let mut value = array[[targets[0].0, targets[0].1]];
    let end_value;

    if !reverse {
        value += 1;
        end_value = TOP;
    } else {
        value -= 1;
        end_value = TRAILHEAD;
    }

    for target in targets {
        for (dr, dc) in DIRECTIONS {
            // Calculate the neighbor's position
            let new_row: isize = target.0 as isize + dr;
            let new_col: isize = target.1 as isize + dc;

            // Ensure the position is within bounds
            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let r = new_row as usize;
                let c = new_col as usize;
                if array[[r, c]] == value {
                    neighbors.push((r, c));
                }
            }
        }
    }

    #[cfg(feature = "debug")]
    println!(
        "Found {} of value {} ({:?})",
        neighbors.len(),
        value,
        neighbors
    );

    if value == end_value {
        if unique {
            neighbors.sort();
            neighbors.dedup();
        }
        Some(neighbors)
    } else {
        find_neighbors(&array, neighbors, reverse, unique)
    }
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
