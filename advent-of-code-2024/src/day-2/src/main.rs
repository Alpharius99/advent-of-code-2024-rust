use utils::{convert_string_vector_to_integer_vector, get_file_contents};

const FILE_PATH: &str = "input_mock.txt";

fn main() {

    let file_content: String = get_file_contents(FILE_PATH);

    let lines: Vec<&str> = get_lines_from_file_content(&file_content);

    let report_vectors = get_report_integer_vectors(get_report_string_vectors(lines));

    println!("{:?}", report_vectors);
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
