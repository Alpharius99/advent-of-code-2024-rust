use std::time::Instant;
use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input";

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiple,
}

pub fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);

    let lines: Vec<&str> = file_content.split("\n").collect();

    // println!("{:?}", lines);

    // main loop
    let mut sum = 0;
    lines.iter().for_each(|line| {
        let result = check_line(line);
        match result {
            Some(v) => sum += v,
            None => print!(""),
        }
    });

    println!("Day 6 Part One answer is {}", sum);
    println!("Day 6 Part Two answer is {}", sum);

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn get_expected_result(input: &str) -> i64 {
    let s: Vec<&str> = input.split(":").collect();

    match string_to_int::<i64>(s[0]) {
        Ok(v) => v,
        Err(_e) => 0,
    }
}

fn get_numbers(input: &str) -> Vec<i64> {
    let mut s: Vec<&str> = input.split(" ").collect();

    s.drain(0..1);

    s.iter()
        .map(|s| s.parse::<i64>().expect("Failed to parse string as integer"))
        .collect()
}

fn check_line(line: &str) -> Option<i64> {
    let x = get_expected_result(line);
    let numbers: Vec<i64> = get_numbers(line);

    // all additions only
    if get_sum(&numbers) == x {
        // println!("Pass {} = {}", join_integers(&numbers, " + "), x);
        return Some(x);
    }

    // all multipliers only
    if get_product(&numbers) == x {
        // println!("Pass {} = {}", join_integers(&numbers, " * "), x);
        return Some(x);
    }

    // get all permutations
    let operations = vec![Operation::Add, Operation::Multiple];
    let mut current = Vec::new();
    let mut permutations = Vec::new();
    generate_permutations(
        numbers.len() - 1,
        &operations,
        &mut current,
        &mut permutations,
    );

    // println!("Line {}", line);
    // println!("Total permutations: {}", permutations.len());

    // test all permutations
    for p in permutations {
        // println!("{:?}", p);

        let mut result: i64 = numbers[0];
        let mut s = String::new();
        s.push_str(&numbers[0].to_string());

        for j in 1..numbers.len() {
            match p[j - 1] {
                Operation::Multiple => {
                    result *= numbers[j];
                    s.push_str(" * ");
                }
                Operation::Add => {
                    result += numbers[j];
                    s.push_str(" + ");
                }
            }
            s.push_str(&numbers[j].to_string());
        }
        if result == x {
            // println!("Pass {} = {}", s, x);
            return Some(x);
        }
    }
    None
}

fn get_sum(numbers: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    for i in 0..numbers.len() {
        result += numbers[i];
    }
    result
}

fn get_product(numbers: &Vec<i64>) -> i64 {
    let mut result: i64 = numbers[0];
    for i in 1..numbers.len() {
        result *= numbers[i];
    }
    result
}

fn generate_permutations(
    n: usize,
    values: &[Operation],
    current: &mut Vec<Operation>,
    results: &mut Vec<Vec<Operation>>,
) {
    if current.len() == n {
        results.push(current.clone());
        return;
    }

    for &value in values {
        current.push(value);
        generate_permutations(n, values, current, results);
        current.pop();
    }
}
