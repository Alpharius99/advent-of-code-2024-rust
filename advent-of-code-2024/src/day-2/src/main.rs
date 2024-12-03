use utils::{convert_string_vector_to_integer_vector, get_file_contents};

const FILE_PATH: &str = "input.txt";

fn main() {

    let file_content: String = get_file_contents(FILE_PATH);

    let lines: Vec<&str> = get_lines_from_file_content(&file_content);

    let report_vectors = get_report_integer_vectors(get_report_string_vectors(lines));
    
    let mut counter = 0;
    
    for item in &report_vectors {
        if is_vector_safe(&item) {  
            counter += 1;
        }
    }
    
    println!("Day 2 answer is {}", counter);
}

fn get_lines_from_file_content(file_content: &str) -> Vec<&str> {
    file_content.lines().collect()
}

fn get_report_string_vectors(lines: Vec<&str>) -> Vec<Vec<&str>> {
    lines.into_iter().map(|line| line.split_whitespace().collect()).collect()
}

fn get_report_integer_vectors(string_vectors: Vec<Vec<&str>>) -> Vec<Vec<i32>> {
    string_vectors.into_iter().map(|string_vector| convert_string_vector_to_integer_vector(string_vector)).collect()
}

fn is_decreasing(vector: &Vec<i32>) -> bool {
    vector.windows(2).all(|pair| pair[0] > pair[1])
}

fn is_increasing(vector: &Vec<i32>) -> bool {
    vector.windows(2).all(|pair| pair[0] < pair[1])
}

fn is_step_valid(vector: &Vec<i32>) -> bool {
    vector.windows(2).all(|pair| (pair[0] - pair[1]).abs() <= 3)
}

fn is_vector_safe(vector: &Vec<i32>) -> bool {
    (is_increasing(vector) || is_decreasing(vector)) && is_step_valid(vector)
}
