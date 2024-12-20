#![warn(clippy::all, clippy::pedantic)]
use std::collections::HashSet;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;
use std::time::Instant;
use utils::{get_file_contents, Grid};

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
                let rotation_cost = if dir == self.direction { 0 } else { 1000 };

                let total_cost = step_cost + rotation_cost;

                neighbors.push((
                    Tile {
                        row: new_row,
                        col: new_col,
                        direction: dir,
                    },
                    total_cost,
                ));
            }
        }

        neighbors
    }
}

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let grid = preamble(file_path);

    println!("Day 16 Part One answer is {}", get_result_part_one(&grid));
    println!("Day 16 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> Grid {
    let file_content: String = get_file_contents(file_path);
    Grid::new(file_content)
}

fn get_result_part_one(grid: &Grid) -> usize {
    let (_, cost) = find_cheapest_path(grid).unwrap();
    cost as usize
}

fn get_result_part_two(grid: &Grid) -> usize {
    count_tiles_of_all_cheapest_paths(grid)
}

fn find_cheapest_path(grid: &Grid) -> Option<(Vec<Tile>, u32)> {
    let start = grid.get_element_by_value('S').unwrap();
    let end = grid.get_element_by_value('E').unwrap();
    let start_tile = Tile {
        row: start.0,
        col: start.1,
        direction: 1, // Arbitrarily choose the initial direction (right == east)
    };

    let end_tile = |state: &Tile| state.row == end.0 && state.col == end.1;

    dijkstra(&start_tile, |tile| tile.neighbors(&grid.array), end_tile)
}

fn count_tiles_of_all_cheapest_paths(grid: &Grid) -> usize {
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();
    
    let (reference_path, min_cost) = find_cheapest_path(grid).unwrap();
    for t in &reference_path {_ = tiles.insert((t.row, t.col))}
    
    for tile in &reference_path[1..reference_path.len() - 1] {
        let mut tmp_grid = grid.clone();
        tmp_grid.set_element(tile.row, tile.col, '#');
        if let Some((path , cost)) = find_cheapest_path(&tmp_grid) {
            if cost == min_cost {
                // println!("Tile: {tile:?}, cost {cost}");
                for t in &path {_ = tiles.insert((t.row, t.col))}
            }
        }
    }
    tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        let file_content = "#####\n#..E#\n#.#.#\n#S#.#\n#####";
        let grid = Grid::new(file_content.to_string());
        let (_, score) = find_cheapest_path(&grid).unwrap();
        println!("{score:?}");
        assert_eq!(score, 2004);
    }

    #[test]
    fn test_sample_part_one() {
        let grid = preamble("sample");
        assert_eq!(get_result_part_one(&grid), 11048);
    }

    #[test]
    fn test_input_part_one() {
        let grid = preamble("input");
        assert_eq!(get_result_part_one(&grid), 72400);
    }

    #[test]
    fn test_sample_part_two() {
        let grid = preamble("sample");
        assert_eq!(get_result_part_two(&grid), 64);
    }

    #[test]
    fn test_input_part_two() {
        let grid = preamble("input");
        assert_eq!(get_result_part_two(&grid), 435);
    }
}
