#![warn(clippy::all)]
use ndarray::Array2;
use std::time::Instant;
use utils::{find_coords_of_char, get_2d_array_char, get_file_contents, Point, DIRECTIONS};

#[derive(Debug)]
struct World {
    array: Array2<char>,
    robot: Point,
}

impl World {
    pub(crate) fn robot(&self) -> Point {
        find_coords_of_char(&self.array, '@').expect("Expected a start position")
    }

    pub(crate) fn next_move(&mut self, target: Point, direction: (isize, isize)) -> bool {
        let next_pos: Point = Point {
            row: (target.row + direction.0).rem_euclid(self.array.nrows() as isize),
            col: (target.col + direction.1).rem_euclid(self.array.ncols() as isize),
        };
        let target_char: char = self.array[(target.row as usize, target.col as usize)];

        match self.array[[next_pos.row as usize, next_pos.col as usize]] {
            '.' => {
                self.array[[target.row as usize, target.col as usize]] = '.';
                self.array[[next_pos.row as usize, next_pos.col as usize]] = target_char;
                true
            }
            '#' => false,
            '[' => {
                if direction.0 == 0 {
                    if let Some(value) = self.move_char(target, direction, next_pos, target_char) {
                        return value;
                    }
                    return false;
                } else if self.next_move(next_pos, direction)
                    && self.next_move(
                        Point {
                            row: next_pos.row,
                            col: next_pos.col + 1,
                        },
                        direction,
                    )
                {
                    self.array[[target.row as usize, target.col as usize]] = '.';
                    self.array[[next_pos.row as usize, next_pos.col as usize]] = target_char;
                    return true;
                }
                false
            }
            ']' => {
                if direction.0 == 0 {
                    if let Some(value) = self.move_char(target, direction, next_pos, target_char) {
                        return value;
                    }
                    return false;
                } else if self.next_move(next_pos, direction)
                    && self.next_move(
                        Point {
                            row: next_pos.row,
                            col: next_pos.col - 1,
                        },
                        direction,
                    )
                {
                    self.array[[target.row as usize, target.col as usize]] = '.';
                    self.array[[next_pos.row as usize, next_pos.col as usize]] = target_char;
                    return true;
                }
                false
            }
            _ => {
                if self.next_move(next_pos, direction) {
                    self.array[[target.row as usize, target.col as usize]] = '.';
                    self.array[[next_pos.row as usize, next_pos.col as usize]] = target_char;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn move_char(&mut self, target: Point, direction: (isize, isize), next_pos: Point, target_char: char) -> Option<bool> {
        if self.next_move(next_pos, direction) {
            self.array[[target.row as usize, target.col as usize]] = '.';
            self.array[[next_pos.row as usize, next_pos.col as usize]] = target_char;
            return Some(true);
        }
        None
    }

    pub(crate) fn checksum(&self) -> usize {
        let mut result = 0;

        let points = self
            .array
            .indexed_iter() // Iterate over indices and elements
            .filter_map(|(index, &elem)| {
                if elem == 'O' || elem == ']' {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>()
            .into_iter()
            .map(|(row, col)| Point {
                row: row as isize,
                col: col as isize,
            });

        // println!("{:?}", points);

        for point in points {
            result += Self::calc_sum(point);
        }

        result
    }

    fn calc_sum(target: Point) -> usize {
        (target.row * 100 + target.col) as usize
    }
}

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let (grid, moves) = preamble(file_path);

    println!(
        "Day 13 Part One answer is {}",
        get_result_part_one(&mut grid.clone(), &moves)
    );
    println!(
        "Day 13 Part Two answer is {}",
        get_result_part_two(&grid.clone(), &moves)
    );

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> (Array2<char>, Vec<char>) {
    let file_content: String = get_file_contents(file_path);
    let sources: Vec<&str> = file_content.split("\n\n").collect();
    let grid = get_2d_array_char(sources[0]);
    let movements = sources[1].chars().collect();
    (grid, movements)
}

fn get_result_part_one(array: &mut Array2<char>, moves: &Vec<char>) -> usize {
    let robot: Point = find_coords_of_char(array, '@').expect("Expected a start position");
    let mut world: World = World {
        array: array.clone(),
        robot,
    };

    movement_loop(moves, &mut world);
    world.checksum()
}

fn get_result_part_two(array: &Array2<char>, moves: &Vec<char>) -> usize {
    let expanded_map = expand_map(array.clone());
    let robot: Point = find_coords_of_char(&expanded_map, '@').expect("Expected a start position");
    let mut world: World = World {
        array: expanded_map,
        robot,
    };
    movement_loop(moves, &mut world);
    world.checksum()
}

fn movement_loop(moves: &Vec<char>, world: &mut World) {
    for movement in moves {
        match movement {
            '<' => {
                world.next_move(world.robot(), DIRECTIONS[1]);
            }
            '>' => {
                world.next_move(world.robot(), DIRECTIONS[2]);
            }
            '^' => {
                world.next_move(world.robot(), DIRECTIONS[0]);
            }
            'v' => {
                world.next_move(world.robot(), DIRECTIONS[3]);
            }
            _ => {}
        }
    }
}

fn expand_map(old_array: Array2<char>) -> Array2<char> {
    let (rows, cols) = old_array.dim();
    let mut expanded_array = Array2::default((rows, cols * 2));

    for ((row, col), value) in old_array.indexed_iter() {
        if *value == '@' {
            expanded_array[(row, col * 2)] = '@';
            expanded_array[(row, col * 2 + 1)] = '.';
        } else if *value == 'O' {
            expanded_array[(row, col * 2)] = '[';
            expanded_array[(row, col * 2 + 1)] = ']';
        } else {
            expanded_array[(row, col * 2)] = *value;
            expanded_array[(row, col * 2 + 1)] = *value;
        }
    }
    expanded_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let array: Array2<char> = Array2::from_shape_vec(
            (9, 9),
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', //
                '#', '.', '.', '.', '.', '.', '.', '.', '#', //
                '#', '.', '.', '[', ']', '.', '.', '.', '#', //
                '#', '.', '[', ']', '.', '.', '.', '.', '#', //
                '#', '.', '.', '@', '.', '.', '.', '.', '#', //
                '#', '.', '.', 'O', '.', '.', '.', '.', '#', //
                '#', '.', '.', 'O', '.', '.', '.', '.', '#', //
                '#', '.', '.', '.', '.', '.', '.', '.', '#', //
                '#', '#', '#', '#', '#', '#', '#', '#', '#', //
            ],
        )
        .expect("Invalid shape!");

        let mut world = World {
            array,
            robot: Point { row: 4, col: 3 },
        };
        let result = world.next_move(Point { row: 4, col: 3 }, (-1, 0));
        println!("{:#?}", world);
        assert!(result);
    }

    #[test]
    fn test_sample_part_one() {
        let (grid, moves) = preamble("sample");
        assert_eq!(get_result_part_one(&mut grid.clone(), &moves), 10092);
    }

    #[test]
    fn test_input_part_one() {
        let (grid, moves) = preamble("input");
        assert_eq!(get_result_part_one(&mut grid.clone(), &moves), 1538871);
    }

    #[test]
    fn test_sample_part_two() {
        let (grid, moves) = preamble("sample");
        assert_eq!(get_result_part_two(&mut grid.clone(), &moves), 9021);
    }

    #[test]
    fn test_input_part_two() {
        let (grid, moves) = preamble("input");
        assert_eq!(get_result_part_two(&mut grid.clone(), &moves), 0);
    }
}
