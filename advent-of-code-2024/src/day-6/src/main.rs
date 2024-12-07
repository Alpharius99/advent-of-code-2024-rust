use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use utils::{get_file_contents, get_grid};

const FILE_PATH: &str = "input";
const START: char = '^';
const VISIT: char = 'X';
const OBSTACLE: char = '#';
const RETRIES: usize = 10000;
const FAKE_OBSTACLE: char = 'O';

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);
    let mut grid: Vec<Vec<char>> = get_grid(&file_content);

    grid[0][0] = 'S'; // anchor for rotation to the original orientation

    let start_pos = get_start_pos(&grid).unwrap();

    walk_out(&mut grid, start_pos);
    println!(
        "Day 6 Part One answer is {}",
        count_distinct_positions(&mut grid)
    );

    // brute force solution with skipping unvisited fields for part two
    while grid[0][0] != 'S' {
        rotate_90_counterclockwise(&mut grid);
    }
    println!("Day 6 Part Two answer is {}", par_iterate(grid, start_pos));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn get_start_pos(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            if ch == START {
                return Some((row_index, col_index));
            }
        }
    }
    None
}

fn walk_out(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize)) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut i = 0;

    let mut position = start_pos;

    while i < RETRIES {
        let (row, col) = position;
        let next_row: isize;
        let next_col: isize;

        // Check bounds
        if row >= rows || col >= cols {
            break;
        }

        grid[row][col] = VISIT;

        if is_an_obstacle_next(&grid, position) {
            rotate_90_counterclockwise(grid);
            next_row = (cols - col) as isize - 1;
            next_col = row as isize; // 1:4 -> 5:1
        } else {
            next_row = row as isize - 1;
            next_col = col as isize;
        }

        // Break if we go out of bounds
        if next_row < 0 || next_col < 0 || next_row >= rows as isize || next_col >= cols as isize {
            return true;
        }

        position = (next_row as usize, next_col as usize);
        i += 1;
    }
    false
}

fn is_an_obstacle_next(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0 == 0 {
        return false;
    }
    grid[pos.0 - 1][pos.1] == FAKE_OBSTACLE || grid[pos.0 - 1][pos.1] == OBSTACLE
}

fn rotate_90_counterclockwise(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let rotated = grid.clone();

    for row in 0..rows {
        for col in 0..cols {
            grid[cols - col - 1][row] = rotated[row][col];
        }
    }
}

fn count_distinct_positions(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    for row in grid {
        for &ch in row {
            if ch == VISIT {
                count += 1;
            }
        }
    }
    count
}

fn par_iterate(grid: Vec<Vec<char>>, start_pos: (usize, usize)) -> usize {
    let counter = AtomicUsize::new(0);
    grid.par_iter().enumerate().for_each(|(row_index, row)| {
        let counter_ref = &counter;
        counter_ref.fetch_add(
            inner_loop(&grid, start_pos, row_index, row),
            Ordering::SeqCst,
        );
    });
    counter.load(Ordering::SeqCst)
}

fn inner_loop(
    grid: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    row_index: usize,
    row: &Vec<char>,
) -> usize {
    let mut thread_counter: usize = 0;
    row.iter().enumerate().for_each(|(col_index, element)| {
        if row_index == start_pos.0 && col_index == start_pos.1 {
            return;
        }

        if *element != VISIT {
            return;
        }

        let mut tmp_grid: Vec<Vec<char>> = grid.clone();
        tmp_grid[row_index][col_index] = FAKE_OBSTACLE;
        let result: bool = walk_out(&mut tmp_grid, start_pos);
        if !result {
            thread_counter += 1;
        }
    });
    thread_counter
}
