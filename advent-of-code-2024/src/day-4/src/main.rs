use utils::{get_file_contents, get_grid};

const FILE_PATH: &str = "input";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);

    let grid: Vec<Vec<char>> = get_grid(&file_content);

    println!("Day 4 Part One answer is {:?}", find_chain(&grid, "XMAS"));
    println!(
        "Day 4 Part Two answer is {:?}",
        find_crossed_chain(&grid, "MAS")
    );
}

fn find_chain(grid: &Vec<Vec<char>>, target: &str) -> i32 {
    let target: Vec<char> = target.chars().collect();
    let rows: usize = grid[0].len();
    let cols: usize = grid.len();

    // Directions: (row delta, column delta)
    //   |-1 | 0 | 1 |
    // -1|   |   |   |
    //  0|   | X |   |
    //  1|   |   |   |
    let directions: [(isize, isize); 8] = [
        (0, 1),   // Right
        (-1, 0),  // Left
        (1, 0),   // Down
        (0, -1),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, -1), // Up-left
        (-1, 1),  // Up-right
    ];

    let mut count: i32 = 0;

    // Iterate through the grid
    for row in 0..rows {
        for col in 0..cols {
            for &(dir_row, dir_col) in &directions {
                if match_chain(grid, &target, row, col, dir_row, dir_col) {
                    count += 1; // Found the chain, increment the count
                }
            }
        }
    }

    count
}

fn find_crossed_chain(grid: &Vec<Vec<char>>, target: &str) -> i32 {
    let rows: usize = grid.len();
    let cols: usize = grid[0].len();

    let mut count: i32 = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            if match_cross(&grid, &target, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn match_chain(
    grid: &Vec<Vec<char>>,
    target: &Vec<char>,
    start_row: usize,
    start_col: usize,
    dir_row: isize,
    dir_col: isize,
) -> bool {
    for (i, &c) in target.iter().enumerate() {
        let row = start_row as isize + i as isize * dir_row;
        let col = start_col as isize + i as isize * dir_col;

        if row < 0 || col < 0 || row >= grid.len() as isize || col >= grid[0].len() as isize {
            return false; // Out of bounds
        }

        if grid[row as usize][col as usize] != c {
            return false; // Character mismatch
        }
    }

    true
}

fn match_cross(grid: &Vec<Vec<char>>, target: &str, row: usize, col: usize) -> bool {
    let target: &Vec<char> = &target.chars().collect();

    if grid[row][col] != target[1] {
        return false;
    }

    if match_first_chain(grid, target, row, col) && match_second_chain(grid, target, row, col) {
        return true;
    }

    false
}

fn match_first_chain(grid: &Vec<Vec<char>>, target: &Vec<char>, row: usize, col: usize) -> bool {
    // Directions: (row delta, column delta)
    //   |-1 | 0 | 1 |
    // -1|   |   |   |
    //  0|   | X |   |
    //  1|   |   |   |
    if grid[row - 1][col - 1] == target[0] && grid[row + 1][col + 1] == target[2] {
        return true;
    }

    if grid[row + 1][col + 1] == target[0] && grid[row - 1][col - 1] == target[2] {
        return true;
    }

    false
}

fn match_second_chain(grid: &Vec<Vec<char>>, target: &Vec<char>, row: usize, col: usize) -> bool {
    // Directions: (row delta, column delta)
    //   |-1 | 0 | 1 |
    // -1|   |   |   |
    //  0|   | X |   |
    //  1|   |   |   |
    if grid[row - 1][col + 1] == target[0] && grid[row + 1][col - 1] == target[2] {
        return true;
    }

    if grid[row + 1][col - 1] == target[0] && grid[row - 1][col + 1] == target[2] {
        return true;
    }

    false
}
