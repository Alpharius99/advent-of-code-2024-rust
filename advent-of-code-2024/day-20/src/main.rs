#![warn(clippy::all)]
use std::time::Instant;
use utils::{get_file_contents, Grid, Tile};

struct Cfg {}

#[cfg(feature = "debug")]
impl Cfg {
    const FILE_PATH: &'static str = "sample";
}

#[cfg(not(feature = "debug"))]
impl Cfg {
    const FILE_PATH: &'static str = "input";
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

fn main() {
    let grid = bench(|| preamble(Cfg::FILE_PATH));
    let p1 = bench(|| p1(&grid));
    match p1 {
        Some(result) => println!("Part 1: {}", result),
        None => println!("Part 1: not found"),
    }
    // println!("Part 2: {:?}", p2);
}

fn preamble(path: &str) -> Grid {
    let file_content: String = get_file_contents(path);
    let grid = Grid::new(file_content);
    grid
}

fn p1(grid: &Grid) -> Option<usize> {
    let start = grid.get_element_by_value('S').unwrap();
    let end = grid.get_element_by_value('E').unwrap();

    let start_tile = Tile {
        row: start.0,
        col: start.1,
        direction: 0,
    };
    let end_tile = Tile {
        row: end.0,
        col: end.1,
        direction: 0,
    };
    let result = grid.get_paths(start_tile, end_tile);
    result.map(|(_, steps)| steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let grid = preamble("sample");
        let result = p1(&grid);
        assert_eq!(result, Some(84));
    }

    // #[test]
    // fn test_input_part_one() {
    //     let (patterns, designs) = preamble("input");
    //     let (p1, _) = count(&patterns, &designs);
    //     assert_eq!(p1, 369);
    // }
    //
    // #[test]
    // fn test_sample_part_two() {
    //     let (patterns, designs) = preamble("sample");
    //     let (_, p2) = count(&patterns, &designs);
    //     assert_eq!(p2, 16);
    // }
    //
    // #[test]
    // fn test_input_part_two() {
    //     let (patterns, designs) = preamble("input");
    //     let (_, p2) = count(&patterns, &designs);
    //     assert_eq!(p2, 761826581538190);
    // }
}
