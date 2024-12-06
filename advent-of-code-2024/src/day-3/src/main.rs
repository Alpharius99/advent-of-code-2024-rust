use regex::Regex;
use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input.txt";
const REGEX_MUL: &str = r"mul\(\d{1,3},\d{1,3}\)";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);

    println!(
        "Day 3 Part One answer is {}",
        calculate_sum(get_matches(&file_content, REGEX_MUL))
    );

    let mut cleaned_file_content: String = file_content
        .replace("\r\n", "")
        .replace('\n', "")
        .replace('\r', "");

    cleaned_file_content = clean_string(&cleaned_file_content, r"don't\(\).*?do\(\)");

    println!(
        "Day 3 Part Two answer is {}",
        calculate_sum(get_matches(&cleaned_file_content, REGEX_MUL))
    );
}

fn get_matches<'a>(file_content: &'a str, pattern: &str) -> Vec<&'a str> {
    let re: Regex = Regex::new(pattern).unwrap();

    let matches: Vec<&str> = re.find_iter(&file_content).map(|m| m.as_str()).collect();

    matches
}

fn calculate_sum(matches: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;

    for item in &matches {
        let a_string = extract_between(item, '(', ',');
        let a: i32 = string_to_int(a_string.expect("Awaited a string"));
        let b_string = extract_between(item, ',', ')');
        let b: i32 = string_to_int(b_string.expect("Awaited a string"));

        sum += a * b;
    }

    sum
}

fn extract_between(input: &str, start: char, end: char) -> Option<&str> {
    let start_index = input.find(start)?;
    let end_index = input[start_index + 1..].find(end)? + start_index + 1;
    Some(&input[start_index + 1..end_index])
}

fn clean_string(string: &str, pattern: &str) -> String {
    let re: Regex = Regex::new(pattern).unwrap();

    re.replace_all(&string, "don't()do()").to_string()
}
