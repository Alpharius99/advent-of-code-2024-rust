use std::time::Instant;
use utils::{get_file_contents, get_grid};

const FILE_PATH: &str = "input";

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);

    let grid: Vec<Vec<char>> = get_grid(&file_content);
    let grid_size: (usize, usize) = (grid.len(), grid[0].len());
    let mut antinods_part1: Vec<Point> = Vec::new();
    let mut antinods_part2: Vec<Point> = Vec::new();

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
                        get_points_part1(
                            Point { row, col },
                            Point {
                                row: row_inner,
                                col: col_inner,
                            },
                            grid_size,
                        )
                        .iter()
                        .for_each(|p| {
                            if !antinods_part1.contains(&p) {
                                antinods_part1.push(*p);
                            }
                        });

                        get_points_part2(
                            Point { row, col },
                            Point {
                                row: row_inner,
                                col: col_inner,
                            },
                            grid_size,
                        )
                        .iter()
                        .for_each(|p| {
                            if !antinods_part2.contains(&p) {
                                antinods_part2.push(*p);
                            }
                        })
                    }
                }
            }
        }
    }

    println!("Day 8 Part One answer is {}", antinods_part1.len());
    println!("Day 8 Part Two answer is {}", antinods_part2.len());

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn get_points_part1(p1: Point, p2: Point, grid_size: (usize, usize)) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let d_row: isize = p2.row as isize - p1.row as isize;
    let d_col: isize = p2.col as isize - p1.col as isize;
    let p1_row: isize = p1.row as isize - d_row;
    let p1_col: isize = p1.col as isize - d_col;
    let p2_row: isize = p2.row as isize + d_row;
    let p2_col: isize = p2.col as isize + d_col;

    point_loop_part1(grid_size, &mut result, p1_row, p1_col);
    point_loop_part1(grid_size, &mut result, p2_row, p2_col);

    result
}

fn point_loop_part1(grid_size: (usize, usize), result: &mut Vec<Point>, row: isize, col: isize) {
    if !is_out_of_bounce(grid_size, (row, col)) {
        result.push(Point {
            row: row as usize,
            col: col as usize,
        })
    }
}

fn get_points_part2(p1: Point, p2: Point, grid_size: (usize, usize)) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    let d_row: isize = p2.row as isize - p1.row as isize;
    let d_col: isize = p2.col as isize - p1.col as isize;

    point_loop_part2(p1, grid_size, &mut result, d_row, d_col);
    point_loop_part2(p2, grid_size, &mut result, d_row, d_col);

    result
}

fn point_loop_part2(
    point: Point,
    grid_size: (usize, usize),
    result: &mut Vec<Point>,
    d_row: isize,
    d_col: isize,
) {
    let mut step: isize = 1;

    while true {
        let row: isize = point.row as isize - d_row * step;
        let col: isize = point.col as isize - d_col * step;

        if !is_out_of_bounce(grid_size, (row, col)) {
            result.push(Point {
                row: row as usize,
                col: col as usize,
            })
        } else {
            break;
        }
        step += 1;
    }
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
