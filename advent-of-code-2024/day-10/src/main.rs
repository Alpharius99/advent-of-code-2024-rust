use std::time::Instant;
use utils::{get_file_contents};

const FILE_PATH: &str = "input";

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);
    
    println!("Day 10 Part One answer is {}", 1);
    println!("Day 10 Part Two answer is {}", 2);

    println!("Execution time: {:.2?}", start_time.elapsed());
}
