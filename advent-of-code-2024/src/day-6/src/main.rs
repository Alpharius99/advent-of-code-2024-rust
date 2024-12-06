use utils::{get_file_contents, get_grid};

const FILE_PATH: &str = "input";
const VISIT: char  = 'X';

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);
    let mut grid: Vec<Vec<char>> = get_grid(&file_content);

    println!("Got grid {}x{}", grid.len(), grid[0].len());

    if let Some((start_row, start_col)) = get_start_pos(&grid) {
        println!(
            "Got start position row = {}, col = {}",
            start_row, start_col
        );
        walk(&mut grid, (start_row, start_col));
    } else {
        println!("No start position found!");
    };
    println!("Day 6 Part One answer is {}", count_distinct_positions(&mut grid));
}

fn get_start_pos(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            if ch == '^' {
                return Some((row_index, col_index));
            }
        }
    }
    None
}

fn walk(grid: &mut Vec<Vec<char>>, start_pos: (usize, usize)) {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut position = start_pos;

    loop {
        let (row, col) = position;
        let next_row: isize;
        let next_col: isize;

        // Check bounds
        if row >= rows || col >= cols {
            break
        }

        grid[row][col] = VISIT;
        //print_grid(&grid);

        if is_an_obstacle_next(&grid, position) {
            //println!("Turned at {}:{}", position.0, position.1);
            rotate_90_counterclockwise(grid);
            next_row = (cols - col) as isize - 2;
            next_col = row as isize;  // 1:4 -> 5:1
        } else {
            next_row = row as isize - 1;
            next_col = col as isize;
        }
        
        // Break if we go out of bounds
        if next_row < 0 || next_col < 0 || next_row >= rows as isize || next_col >= cols as isize {
            break;
        }

        position = (next_row as usize, next_col as usize);
        //println!("Next step into {}:{}", next_row, next_col);
    }
}

fn is_an_obstacle_next(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    if pos.0 == 0 {
        return false;
    }

    grid[pos.0 - 1][pos.1] == '#'
}

fn rotate_90_counterclockwise(grid: &mut Vec<Vec<char>>) {
    let rows = grid.len();
    let cols = grid[0].len();
    //let mut rotated = vec![vec![' '; rows]; cols];
    let rotated = grid.clone();

    for row in 0..rows {
        for col in 0..cols {
            grid[cols - col - 1][row] = rotated[row][col];
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        let joined: String = row.into_iter().collect();
        println!("{:?}", joined);
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
