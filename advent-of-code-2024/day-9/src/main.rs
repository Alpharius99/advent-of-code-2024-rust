use std::time::Instant;
use utils::get_file_contents;

const FILE_PATH: &str = "input";

fn main() {
    let start_time = Instant::now();
    let file_content: String = get_file_contents(FILE_PATH);

    let control_line: Vec<char> = file_content.chars().collect();
    let mut file_map: Vec<Option<usize>> = Vec::new();

    expansion(control_line, &mut file_map);

    #[cfg(feature = "debug")]
    println!("Len of map: {}", file_map.len());

    // Part One
    println!("Day 9 Part One answer is {}", calc_checksum(&shift_bytes_part1(&file_map)));

    // Part Two
    println!("Day 9 Part Two answer is {}", calc_checksum(&shift_bytes_part2(&file_map)));

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

fn shift_bytes_part1(file_map: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut file_map_inner: Vec<Option<usize>> = file_map.clone();
    loop {
        for i in 0..file_map_inner.len() - 1 {
            if file_map_inner[i] == None {
                for j in (0..file_map_inner.len()).rev() {
                    if i >= file_map_inner.len() {
                        return file_map_inner;
                    }

                    if file_map_inner[j] != None {
                        file_map_inner[i] = file_map_inner.pop().unwrap();
                        break;
                    } else {
                        file_map_inner.pop();
                    }
                }
            }

            if !file_map_inner.contains(&None) {
                return file_map_inner;
            }
        }
    }
}

fn shift_bytes_part2(file_map: &Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut file_map_inner: Vec<Option<usize>> = file_map.clone();
    let mut before_i = None;
    let mut file: Option<(usize, usize)>;
    let mut space: Option<(usize, usize)>;
    loop {
        #[cfg(feature = "debug")]
        println!("{:?}", vector_of_some_to_string(&file_map_inner));
        file = find_last_file(&file_map_inner, before_i);
        match file {
            Some((_start, _end)) => {
                before_i = Some(file.unwrap().0);
                // break, if the beginning of the content is arrived
                if before_i.unwrap() == 0 {
                    return file_map_inner;
                }
                // find the first space with length longer or equal to the length of file
                space = find_first_space_fitting_file(
                    &file_map_inner,
                    file.unwrap().1 - file.unwrap().0 + 1,
                );
                match space {
                    Some((_start, _end)) => {
                        move_file_into_space(&mut file_map_inner, file.unwrap(), space.unwrap());
                    }
                    None => {}
                }
            }

            None => {
                return file_map_inner; // break, if no files fitting spaces anymore
            }
        }
    }
}

fn find_last_file(vec: &Vec<Option<usize>>, before_i: Option<usize>) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;
    let mut file_id = find_last_file_id(vec, before_i);
    #[cfg(feature = "debug")]
    println!("Looking for a file ID {:?}", file_id);

    for i in (0..vec.len()).rev() {
        if vec[i] == file_id {
            file_id = vec[i];
            if end.is_none() {
                end = Some(i);
            }
            start = Some(i);
        } else if start != None {
            break;
        }
    }

    if let (Some(start), Some(end)) = (start, end) {
        #[cfg(feature = "debug")]
        println!("Found last file at {:?}", (start, end));
        Some((start, end))
    } else {
        None
    }
}

fn find_last_file_id(vec: &Vec<Option<usize>>, before_i: Option<usize>) -> Option<usize> {
    let mut start_i: usize = vec.len();

    if !before_i.is_none() {
        start_i = before_i.unwrap_or(0);
    }
    for i in (0..start_i).rev() {
        if vec[i] != None {
            #[cfg(feature = "debug")]
            println!("Last file ID {:?}", vec[i]);
            return vec[i];
        }
    }
    None
}

fn find_first_space_fitting_file(
    vec: &Vec<Option<usize>>,
    length: usize,
) -> Option<(usize, usize)> {
    let mut start = None;
    let mut end = None;

    for (i, &value) in vec.iter().enumerate() {
        if value == None {
            if start.is_none() {
                start = Some(i);
            }
            end = Some(i);
        } else if end != None {
            let space_length = end.unwrap() - start.unwrap() + 1;
            if space_length >= length {
                #[cfg(feature = "debug")]
                println!(
                    "The found space {:?} ({}) matches {}",
                    (start, end),
                    space_length,
                    length
                );
                return Some((start.unwrap(), end.unwrap()));
            } else {
                #[cfg(feature = "debug")]
                println!(
                    "The found space {:?} ({}) is shorter, than {}",
                    (start, end),
                    space_length,
                    length
                );
                start = None;
                end = None;
            }
        }
    }

    #[cfg(feature = "debug")]
    println!("No fitting space found");
    None
}

fn move_file_into_space(vec: &mut Vec<Option<usize>>, file: (usize, usize), space: (usize, usize)) {
    #[cfg(feature = "debug")]
    println!(
        "Move file {:?} with ID {:?} to space {:?}",
        file, vec[file.0], space
    );

    // only the left shift allowed
    if file.0 < space.0 {
        #[cfg(feature = "debug")]
        println!(
            "Skipping {:?}, since the space is on the right {:?}",
            file, space
        );
        return;
    }

    let file_length = (file.1 - file.0) + 1;

    for i in space.0..(space.0 + file_length) {
        let source_i: usize = file.1 + space.0 - i;
        vec[i] = vec[source_i];
        vec[source_i] = None;
    }
}

fn calc_checksum(vec: &Vec<Option<usize>>) -> usize {
    let mut checksum: usize = 0;
    for i in 0..vec.len() {
        if let Some(file_id) = vec[i] {
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
