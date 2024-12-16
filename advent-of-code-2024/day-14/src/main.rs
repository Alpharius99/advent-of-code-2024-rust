#![warn(clippy::all)]

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Index;
use std::time::Instant;
use utils::{get_file_contents, get_value_by_regex, Point};

#[derive(Debug, PartialEq, Clone)]
struct Robot {
    start_position: Point,
    velocity: Point,
    end_position: Point,
}

impl Robot {
    pub(crate) fn go_steps(&mut self, steps: usize) {
        let mut row: isize = self.start_position.row + self.velocity.row * steps as isize;
        let mut col: isize = self.start_position.col + self.velocity.col * steps as isize;
        row = row.rem_euclid(LIMITS.0 as isize);
        col = col.rem_euclid(LIMITS.1 as isize);
        self.end_position.row = row;
        self.end_position.col = col;
    }

    pub(crate) fn go_next_step(&mut self) {
        let mut row: isize = self.end_position.row + self.velocity.row;
        let mut col: isize = self.end_position.col + self.velocity.col;
        row = row.rem_euclid(LIMITS.0 as isize);
        col = col.rem_euclid(LIMITS.1 as isize);
        self.end_position.row = row;
        self.end_position.col = col;
    }
}

#[cfg(feature = "debug")]
const LIMITS: (usize, usize) = (11, 7);
#[cfg(not(feature = "debug"))]
const LIMITS: (usize, usize) = (101, 103);

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let data = preamble(file_path);

    println!(
        "Day 13 Part One answer is {}",
        get_result_part_one(&mut data.clone())
    );
    println!(
        "Day 13 Part Two answer is {}",
        get_result_part_two(&mut data.clone())
    );

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> Vec<Robot> {
    let file_content: String = get_file_contents(file_path);
    let lines: Vec<&str> = file_content.lines().collect();
    let mut robots: Vec<Robot> = Vec::new();
    for line in lines {
        robots.push(Robot {
            start_position: Point {
                row: get_value_by_regex(line, &Regex::new(r"p=(-?\d+),*").unwrap()),
                col: get_value_by_regex(line, &Regex::new(r"p=-?\d+,(-?\d+)*").unwrap()),
            },
            velocity: Point {
                row: get_value_by_regex(line, &Regex::new(r"v=(-?\d+),*").unwrap()),
                col: get_value_by_regex(line, &Regex::new(r"v=-?\d+,(-?\d+)*").unwrap()),
            },
            end_position: Point { row: -1, col: -1 },
        });
    }
    robots
}

fn get_result_part_one(data: &mut [Robot]) -> usize {
    let mut quadrants: [usize; 4] = [0; 4];
    let mid_row = LIMITS.0 / 2;
    let mid_col = LIMITS.1 / 2;
    for robot in data.iter_mut() {
        robot.go_steps(100);
        update_quadrants(&mut quadrants, &mid_row, &mid_col, robot);
    }
    calc_score(quadrants)
}

fn get_result_part_two(robots: &mut [Robot]) -> usize {
    let mut quadrants: [usize; 4] = [0; 4];
    let mid_row = LIMITS.0 / 2;
    let mid_col = LIMITS.1 / 2;
    let mut scores: Vec<usize> = Vec::new();

    for robot in robots.iter_mut() {
        robot.end_position = robot.start_position;
    }

    for _ in 0..10402 {
        for robot in robots.iter_mut() {
            robot.go_next_step();
            update_quadrants(&mut quadrants, &mid_row, &mid_col, robot);
        }

        scores.push(calc_middle_line_score(robots));
    }

    scores
        .iter() // Create an iterator over the vector
        .enumerate() // Pair each element with its index
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)) // Find the minimum value
        .map(|(index, _)| index) // Extract the index of the minimum value
        .unwrap()
}

fn calc_score(quadrants: [usize; 4]) -> usize {
    let mut score = 1;
    for x in quadrants {
        if x > 0 {
            score *= x;
        }
    }
    score
}

fn calc_middle_line_score(robots: &[Robot]) -> usize {
    let mut score = 0;
    let mid = 103 / 2;
    for robot in robots.iter() {
        if robot.end_position.col == mid {
            score += 1;
        }
    }
    score
}

fn update_quadrants(
    quadrants: &mut [usize; 4],
    mid_row: &usize,
    mid_col: &usize,
    robot: &mut Robot,
) {
    match (robot.end_position.row as usize).cmp(mid_row) {
        Ordering::Less => match (robot.end_position.col as usize).cmp(mid_col) {
            Ordering::Less => {
                quadrants[0] += 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                quadrants[1] += 1;
            }
        },
        Ordering::Equal => {}
        Ordering::Greater => match (robot.end_position.col as usize).cmp(mid_col) {
            Ordering::Less => {
                quadrants[2] += 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                quadrants[3] += 1;
            }
        },
    }
}

fn get_grid(robots: &[Robot]) -> [[usize; 103]; 101] {
    let mut grid: [[usize; 103]; 101] = [[0; 103]; 101];
    for robot in robots.iter() {
        let row = robot.end_position.row as usize;
        let col = robot.end_position.col as usize;
        grid[row][col] += grid[row][col];
    }
    grid
}

fn calc_entropy(data: &[[usize; 103]; 101]) -> f64 {
    // Step 1: Flatten the 2D array into a 1D vector
    let flattened: Vec<usize> = data.iter().flat_map(|row| row.iter()).copied().collect();

    // Step 2: Count frequencies of each value
    let mut frequencies: HashMap<usize, usize> = HashMap::new();
    for &value in &flattened {
        *frequencies.entry(value).or_insert(0) += 1;
    }

    // Step 3: Calculate total number of elements
    let total_count = flattened.len() as f64;

    // Step 4: Compute entropy
    frequencies
        .values()
        .map(|&count| {
            let probability = count as f64 / total_count;
            if probability > 0.0 {
                probability * probability.log2() // p * log2(p)
            } else {
                0.0
            }
        })
        .sum::<f64>()
        .abs() // Take the absolute value of the result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let mut data = preamble("sample");
        assert_eq!(get_result_part_one(&mut data), 12);
    }

    #[test]
    fn test_input_part_one() {
        let mut data = preamble("input");
        assert_eq!(get_result_part_one(&mut data), 229839456);
    }

    #[test]
    fn test_sample_part_two() {
        let mut data = preamble("sample");
        assert_eq!(get_result_part_two(&mut data), 875318608908);
    }

    #[test]
    fn test_input_part_two() {
        let mut data = preamble("input");
        assert_eq!(get_result_part_two(&mut data), 83029436920891);
    }
}
