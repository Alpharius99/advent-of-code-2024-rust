use std::time::Instant;
use utils::{get_file_contents, get_grid, print_grid};

const FILE_PATH: &str = "input";

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);

    let mut grid: Vec<Vec<char>> = get_grid(&file_content);
    let grid_size: (usize, usize) = (grid.len(), grid[0].len());
    let mut antinods: Vec<Point> = Vec::new();

    // main loop
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                continue;
            } // already an antinode
            if grid[row][col] == '.' {
                continue;
            } // skip empty

            // antinode finding loop
            for row_inner in 0..grid.len() {
                for col_inner in 0..grid[0].len() {
                    if row_inner == row && col_inner == col {
                        continue;
                    }
                    if grid[row_inner][col_inner] == grid[row][col] {
                        let valid_targets: Vec<Point> = extrapolate_line(
                            Point { row, col },
                            Point {
                                row: row_inner,
                                col: col_inner,
                            },
                            grid_size,
                        );

                        valid_targets.iter().for_each(|p| {
                            // if grid[p.row][p.col] == '.' || grid[p.row][p.col] == '#' {
                            //     grid[p.row][p.col] = '#';
                            //     }
                            if !antinods.contains(&p) {
                                antinods.push(*p);
                            }
                        });
                    }
                }
            }
        }
    }
    
    println!("Day 8 Part One answer is {}", antinods.len());
    println!("Day 8 Part Two answer is {}", 0);

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn extrapolate_line(p1: Point, p2: Point, grid_size: (usize, usize)) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let d_row: isize = p2.row as isize - p1.row as isize;
    let d_col: isize = p2.col as isize - p1.col as isize;
    let p1_row: isize = p1.row as isize - d_row;
    let p1_col: isize = p1.col as isize - d_col;
    let p2_row: isize = p2.row as isize + d_row;
    let p2_col: isize = p2.col as isize + d_col;

    if !is_out_of_bounce(grid_size, (p1_row, p1_col)) {
        result.push(Point {
            row: p1_row as usize,
            col: p1_col as usize,
        })
    }
    if !is_out_of_bounce(grid_size, (p2_row, p2_col)) {
        result.push(Point {
            row: p2_row as usize,
            col: p2_col as usize,
        })
    }

    result
}

fn is_out_of_bounce(grid_size: (usize, usize), point: (isize, isize)) -> bool {
    if point.0 < 0
        || point.1 < 0
        || point.0 >= grid_size.0 as isize
        || point.1 >= grid_size.1 as isize
    {
        return true;
    }
    false
}
