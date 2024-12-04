use regex::Regex;
use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input.txt";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);

    let pattern: &str = r"mul\(\d{1,3},\d{1,3}\)";

    let re: Regex = Regex::new(pattern).unwrap();

    let matches: Vec<&str> = re.find_iter(&file_content).map(|m| m.as_str()).collect();
    
    let mut sum: i32 = 0;

    for item in &matches {
        let a_string = extract_between(item, '(', ',');
        let a: i32 = string_to_int(a_string.expect("Awaited a string"));
        let b_string = extract_between(item, ',', ')');
        let b: i32 = string_to_int(b_string.expect("Awaited a string"));
        
        sum += a * b;
    }
    
    println!("Day 3 answer is {}", sum);
}

fn extract_between(input: &str, start: char, end: char) -> Option<&str> {
    let start_index = input.find(start)?;
    let end_index = input[start_index + 1..].find(end)? + start_index + 1;
    Some(&input[start_index + 1..end_index])
}
