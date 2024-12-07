use std::time::Instant;
use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiple,
    Concatenation,
}

pub fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);

    let lines: Vec<&str> = file_content.split("\n").collect();

    // main loop
    let mut sum = 0;
    lines.iter().for_each(|line| {
        // let result = check_line(line);
        // match result {
        //     Some(v) => sum += v,
        //     None => (),
        // }

        if let Some(v) = check_line_backward(line) {
            println!("Passed {}", v);
            sum += v;
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
        return Some(x);
    }

    // all multipliers only
    if get_product(&numbers) == x {
        return Some(x);
    }

    // get all permutations
    let operations = vec![
        Operation::Add,
        Operation::Multiple,
        Operation::Concatenation,
    ];
    let mut current = Vec::new();
    let mut permutations = Vec::new();
    generate_permutations(
        numbers.len() - 1,
        &operations,
        &mut current,
        &mut permutations,
    );

    for p in permutations {
        let mut result = numbers[0];

        for j in 1..numbers.len() {
            match p[j - 1] {
                Operation::Add => {
                    result += numbers[j];
                }
                Operation::Multiple => {
                    result *= numbers[j];
                }
                Operation::Concatenation => {
                    result = concatenate_numbers(result, numbers[j]);
                }
            }
        }
        if result == x {
            return Some(x);
        }
    }
    None
}

fn check_line_backward(line: &str) -> Option<i64> {
    let x = get_expected_result(line);
    let mut numbers: Vec<i64> = get_numbers(line);

    println!("-----------------------------------------------{}", line);

    if check_step(x, &mut numbers, true) {
        return Some(x);
    }

    None
}

fn check_step(mut expected: i64, numbers: &mut Vec<i64>, mut flag: bool) -> bool {
    if numbers.len() == 0 {
        return true;
    }

    if numbers.len() == 1 {
        println!("{}?{}", expected, numbers[0]);
        return expected == numbers[0];
    }

    println!("{:?} ? {:?}", numbers, expected);

    if ends_with_digits(expected, *numbers.last().unwrap()) && flag {
        println!("Ends with {}", numbers.last().unwrap());
        expected = truncate_digits(&mut expected, *numbers.last().unwrap());
        flag = false;
    } else if expected % numbers.last().unwrap() == 0 {
        println!("/ {}", numbers.last().unwrap());
        expected /= numbers.last().unwrap();
        flag = true;
    } else {
        println!("- {}", numbers.last().unwrap());
        expected -= numbers.last().unwrap();
        flag = true;
    }

    numbers.truncate(numbers.len() - 1);
    check_step(expected, numbers, flag)
}

fn ends_with_digits(num: i64, last: i64) -> bool {
    let num_str = num.to_string();
    let last_str = last.to_string();
    num_str.ends_with(&last_str)
}

fn truncate_digits(num: &mut i64, suffix: i64) -> i64 {
    (*num - suffix) / (10 * suffix.to_string().len() as i64)
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

fn concatenate_numbers(a: i64, b: i64) -> i64 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<i64>().unwrap()
}
