#![warn(clippy::all)]
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use pathfinding::prelude::dijkstra;
use std::time::Instant;
use utils::get_file_contents;

struct Cfg {}

#[cfg(feature = "debug")]
impl Cfg {
    const FILE_PATH: &'static str = "sample";
    const GRID_SIZE: usize = 6;
    const COUNTER: usize = 12;
}

#[cfg(not(feature = "debug"))]
impl Cfg {
    const FILE_PATH: &'static str = "input";
    const GRID_SIZE: usize = 70;
    const COUNTER: usize = 1024;
}

fn bench<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let t0 = Instant::now();
    let result = f(); // Call the function and store the result
    println!("time used: {:?}", Instant::now().duration_since(t0));
    result // Return the result of the function
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Tile {
    row: usize,
    col: usize,
    direction: usize, // 0: Up, 1: Right, 2: Down, 3: Left
}

impl Tile {
    fn neighbors(&self, grid: &Array2<char>) -> Vec<(Tile, u32)> {
        let directions = [
            (-1, 0), // Up
            (0, 1),  // Right
            (1, 0),  // Down
            (0, -1), // Left
        ];

        let mut neighbors = Vec::new();

        for (dir, &(dr, dc)) in directions.iter().enumerate() {
            let new_row = self.row as isize + dr;
            let new_col = self.col as isize + dc;

            if new_row >= 0
                && new_row < grid.nrows() as isize
                && new_col >= 0
                && new_col < grid.ncols() as isize
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                // Skip obstacles
                if grid[[new_row, new_col]] == '#' {
                    continue;
                }

                // Cost of moving
                let step_cost = 1;

                // Cost of changing the direction
                // let rotation_cost = if dir == self.direction { 0 } else { 1000 };

                // let total_cost = step_cost + rotation_cost;

                neighbors.push((
                    Tile {
                        row: new_row,
                        col: new_col,
                        direction: dir,
                    },
                    step_cost, // total_cost
                ));
            }
        }

        neighbors
    }
}

fn main() {
    let file_content = bench(|| preamble());
    // println!("{grid:?}");
    let p1 = bench(|| p1(&file_content));
    match p1 {
        Some(cost) => {
            println!("Part 1: {:?}", cost);
        }
        None => println!("Part 1: No path found"),
    }

    let p2 = bench(|| crate::p2(&file_content));
    match p2 {
        Some(cost) => {
            println!("Part 2: {:?}", cost);
        }
        None => println!("Part 2: No path found"),
    }
}

fn preamble() -> String {
    get_file_contents(Cfg::FILE_PATH)
}

fn p1(file_content: &str) -> Option<usize> {
    let mut grid = Array2::from_elem((Cfg::GRID_SIZE + 1, Cfg::GRID_SIZE + 1), '.');

    fill_grid(file_content, &mut grid, Cfg::COUNTER);

    let (_, cost) = find_path(&grid)?;
    Some(cost as usize)
}

fn p2(file_content: &str) -> Option<&str> {
    let mut low = 0;
    let mut high = file_content.lines().count();
    let mut result = None;

    while low <= high {
        let mut grid = Array2::from_elem((Cfg::GRID_SIZE + 1, Cfg::GRID_SIZE + 1), '.');
        let mid = low + (high - low) / 2;
        println!("mid: {}", mid);
        fill_grid(file_content, &mut grid, mid);

        match find_path(&grid) {
            Some((_, cost)) => {
                println!("cost: {}", cost);
                result = Some(file_content.lines().collect::<Vec<&str>>()[mid]);
                low = mid + 1;
            }
            None => {
                println!("no path found");
                high = mid - 1;
            }
        }
    }

    result
}

fn fill_grid(file_content: &str, grid: &mut ArrayBase<OwnedRepr<char>, Ix2>, count: usize) {
    let mut i: usize = 0;

    for line in file_content.lines() {
        if let Some((x, y)) = line
            .split_once(",")
            .and_then(|(x, y)| x.parse::<usize>().ok().zip(y.parse::<usize>().ok()))
        {
            grid[[y, x]] = '#';
        }

        i += 1;
        if i == count {
            break;
        }
    }
}

fn find_path(grid: &Array2<char>) -> Option<(Vec<Tile>, u32)> {
    let start = Tile {
        row: 0,
        col: 0,
        direction: 0,
    };
    let end = Tile {
        row: Cfg::GRID_SIZE,
        col: Cfg::GRID_SIZE,
        direction: 0,
    };

    let end_tile = |state: &Tile| state.row == end.row && state.col == end.col;

    dijkstra(&start, |tile| tile.neighbors(grid), end_tile)
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_sample_part_one() {
    //     let grid = preamble_p1();
    //     assert_eq!(find_path(&grid).unwrap().1, 22);
    // }

    // #[test]
    // fn test_input_part_one() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_one(&grid), 276);
    // }
    //
    // #[test]
    // fn test_sample_part_two() {
    //     let grid = preamble("sample");
    //     assert_eq!(get_result_part_two(&grid), "6,1");
    // }
    //
    // #[test]
    // fn test_input_part_two() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_two(&grid), "60,37");
    // }
}
