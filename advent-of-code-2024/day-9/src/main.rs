use std::time::Instant;
use utils::{get_file_contents};

const FILE_PATH: &str = "input";

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);
    
    let control_line: Vec<char> = file_content.chars().collect();
    let mut file_map: Vec<Option<usize>> = Vec::new();

    expansion(control_line, &mut file_map);
    
    println!("Len of map: {}", file_map.len());
    // println!("{:?}", vector_of_some_to_string(&file_map));
    
    // shift
    shift_bytes(&mut file_map);
    // println!("{:?}", vector_of_some_to_string(&file_map));
    
    println!("Day 9 Part One answer is {}", calc_checksum(&file_map));
    println!("Day 9 Part Two answer is {}", 2);

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn expansion(control_line: Vec<char>, file_map: &mut Vec<Option<usize>>) {
    let mut file_id: usize = 0;
    let mut byte_value: usize;

    // expansion
    for i in (0..control_line.len()).step_by(2) {
        // occupied byte loop
        byte_value = control_line[i].to_digit(10).unwrap() as usize;
        for _j in 0..byte_value {
            file_map.push(Some(file_id));
        }
        file_id += 1;

        // free space loop
        if i < control_line.len() - 1 {
            byte_value = control_line[i + 1].to_digit(10).unwrap() as usize;
            for _j in 0..byte_value {
                file_map.push(None);
            }
        }
    }
}

fn shift_bytes(file_map: &mut Vec<Option<usize>>) {
    loop {
        for i in 0..file_map.len() - 1 {
            // println!("i = {}: {:?}", i, file_map[i]);
            if file_map[i] == None {
                for j in (0..file_map.len()).rev() {
                    // println!("j = {}: {:?}", j, file_map[j]);
                    if i >= file_map.len() {
                        return;
                    }

                    if file_map[j] != None {
                        file_map[i] = file_map.pop().unwrap();
                        break;
                    } else {
                        file_map.pop();
                    }
                }
            }

            if !file_map.contains(&None) {
                return;
            }
        }
    }
}

fn calc_checksum(file_map: &Vec<Option<usize>>) -> usize {
    let mut checksum: usize = 0;
    for i in 0..file_map.len() {
        if let Some(file_id) = file_map[i] {
            checksum += file_id * i;
        }
    }
    checksum
}

fn vector_of_some_to_string(vector: &Vec<Option<usize>>) -> String {
    vector
        .iter()
        .map(|opt| match opt {
            Some(value) => value.to_string(),
            None => ".".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}
