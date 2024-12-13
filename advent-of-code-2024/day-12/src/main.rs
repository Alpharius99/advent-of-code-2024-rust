#![warn(clippy::all)]
use ndarray::Array2;
use std::collections::HashSet;
use std::time::Instant;
use utils::{get_2d_array_char, get_file_contents, Point, DIRECTIONS};

fn main() {
    let start_time = Instant::now();
    let grid = preamble();

    println!("Day 12 Part One answer is {}", get_result_part_one(&grid));
    println!("Day 12 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble() -> Array2<char> {
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let file_content: String = get_file_contents(file_path);
    get_2d_array_char(&file_content)
}

fn get_result_part_one(array: &Array2<char>) -> usize {
    let mut result: usize = 0;
    let regions = find_regions(array);

    for region in regions {
        result += region.len() * get_fences_of_region(&region, array);
    }
    result
}

fn get_result_part_two(array: &Array2<char>) -> usize {
    let mut result: usize = 0;
    let regions = find_regions(array);

    for region in regions {
        #[cfg(feature = "debug")]
        println!(
            "Region {} has area of {} and {} sides",
            array[[region[0].row as usize, region[0].col as usize]],
            region.len(),
            count_corners(&region)
        );
        result += region.len() * count_corners(&region);
    }
    result
}

fn find_regions(array: &Array2<char>) -> Vec<Vec<Point>> {
    let rows = array.nrows();
    let cols = array.ncols();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if !visited[row][col] {
                let mut region = Vec::new();
                dfs(
                    array,
                    &mut visited,
                    Point {
                        row: row as isize,
                        col: col as isize,
                    },
                    array[[row, col]],
                    &mut region,
                );
                regions.push(region);
            }
        }
    }

    #[cfg(feature = "debug")]
    println!("regions: {} items", regions.len());

    regions
}

// Helper function for DFS
fn dfs(
    array: &Array2<char>,
    visited: &mut Vec<Vec<bool>>,
    point: Point,
    value: char,
    region: &mut Vec<Point>,
) {
    visited[point.row as usize][point.col as usize] = true;
    region.push(point);

    for (dr, dc) in DIRECTIONS {
        let new_row = point.row + dr;
        let new_col = point.col + dc;

        if new_row >= 0
            && new_row < array.nrows() as isize
            && new_col >= 0
            && new_col < array.ncols() as isize
        {
            let new_point = Point {
                row: new_row,
                col: new_col,
            };
            if !visited[new_point.row as usize][new_point.col as usize]
                && array[[new_point.row as usize, new_point.col as usize]] == value
            {
                dfs(array, visited, new_point, value, region);
            }
        }
    }
}

fn get_fences_of_region(region: &Vec<Point>, array: &Array2<char>) -> usize {
    let mut result: usize = 0;
    for point in region {
        for (dr, dc) in DIRECTIONS {
            let new_row: isize = point.row + dr;
            let new_col: isize = point.col + dc;

            if new_row >= 0
                && new_row < array.nrows() as isize
                && new_col >= 0
                && new_col < array.ncols() as isize
            {
                let r = new_row as usize;
                let c = new_col as usize;
                if array[[r, c]] != array[[point.row as usize, point.col as usize]] {
                    result += 1;
                }
            } else {
                // Count out of bound as a match
                result += 1;
            }
        }
    }
    result
}

fn count_corners(region: &Vec<Point>) -> usize {
    let mut corners = HashSet::new();
    let kernels = [
        [(-1, 0), (0, -1), (-1, -1)], // upper left
        [(1, 0), (0, -1), (1, -1)],   // upper right
        [(-1, 0), (0, 1), (-1, 1)],   // lower left
        [(1, 0), (0, 1), (1, 1)],     // lower right
    ];

    // get outer corners
    for &point in region {
        for (i, kernel) in kernels.iter().enumerate() {
            let vals: Vec<Point> = kernel
                .iter()
                .map(|(kx, ky)| Point {
                    row: point.row + kx,
                    col: point.col + ky,
                })
                .collect();
            if vals.iter().all(|v| !region.contains(v)) {
                corners.insert((point.row, point.col, i));
            }
        }
    }

    let inner_kernels = [
        [(-1, 0), (0, -1)],
        [(-1, 0), (0, 1)],
        [(1, 0), (0, -1)],
        [(1, 0), (0, 1)],
    ];
    let mut inner_corners: HashSet<(Point, usize)> = HashSet::new();
    // get inner corners
    for &point in &get_boundaries(region) {
        for (i, kernel) in inner_kernels.iter().enumerate() {
            let vals: Vec<Point> = kernel
                .iter()
                .map(|k| Point {
                    row: point.row + k.0,
                    col: point.col + k.1,
                })
                .collect();
            if vals.iter().all(|v| region.contains(v)) {
                let (dx, dy) = (kernel[0].0 + kernel[1].0, kernel[0].1 + kernel[1].1);
                if region.contains(&Point {
                    row: point.row + dx,
                    col: point.col + dy,
                }) {
                    inner_corners.insert((
                        Point {
                            row: point.row + dx,
                            col: point.col + dy,
                        },
                        i,
                    ));
                } else {
                    let (Point { row: v1x, col: v1y }, Point { row: v2x, col: v2y }) =
                        (vals[0], vals[1]);
                    let (dx, dy) = (v1x - v2x, v1y - v2y);
                    let d1 = [(-dx, 0), (0, dy)];
                    let d2 = [(dx, 0), (0, -dy)];

                    inner_corners.insert((
                        Point { row: v1x, col: v1y},
                        inner_kernels.iter().position(|&x| x == d1).unwrap(),
                    ));
                    inner_corners.insert((
                        Point { row: v2x, col: v2y},
                        inner_kernels.iter().position(|&x| x == d2).unwrap(),
                    ));
                }
            }
        }
    }

    corners.len() + inner_corners.len()
}

fn get_boundaries(points: &Vec<Point>) -> HashSet<Point> {
    // let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    points
        .iter()
        .flat_map(|p| {
            DIRECTIONS
                .iter()
                .map(move |(dx, dy)| Point {
                    row: p.row + dx,
                    col: p.col + dy,
                })
                .filter(|new_p| !points.contains(new_p))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let input = get_file_contents("sample");
        let data = get_2d_array_char(&input);
        assert_eq!(get_result_part_one(&data), 1_930);
    }

    #[test]
    fn test_sample_part_two() {
        let input = get_file_contents("sample");
        let data = get_2d_array_char(&input);
        assert_eq!(get_result_part_two(&data), 1_206);
    }

    #[test]
    fn test_input_part_one() {
        let input = get_file_contents("input");
        let data = get_2d_array_char(&input);
        assert_eq!(get_result_part_one(&data), 1_424_472);
    }

    #[test]
    fn test_input_part_two() {
        let input = get_file_contents("input");
        let data = get_2d_array_char(&input);
        assert_eq!(get_result_part_two(&data), 870_202);
    }
}
