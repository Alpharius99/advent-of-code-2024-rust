use utils::{get_file_contents};

const FILE_PATH: &str = "input";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);
    println!("Hello, world!");
}
