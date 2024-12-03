use std::fs;

pub fn get_file_contents(filename: &str) -> String {
    let mut content = fs::read_to_string(filename)
        .expect("Failed to read file");

    // Check for and remove the BOM
    if content.starts_with('\u{feff}') {
        content = content.trim_start_matches('\u{feff}').to_string();
    }

    content
}

pub fn convert_string_vector_to_integer_vector(vector: Vec<&str>) -> Vec<i32> {
    vector
        .iter()
        .map(|s| s.parse::<i32>().expect("Failed to parse string as integer"))
        .collect()
}
