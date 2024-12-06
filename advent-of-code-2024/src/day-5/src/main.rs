use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input.txt";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);
    let sections: Vec<&str> = file_content.split("\n\n").collect();
    let rules: Vec<&str> = sections[0].lines().collect();
    let updates: Vec<&str> = sections[1].lines().collect();
    let mut sum_one: i32 = 0;
    let mut sum_two: i32 = 0;

    for update in updates {
        let mut pages: Vec<&str> = get_pages(update);
        let mut violated_rule_index: Option<usize> = find_violated_rule(&pages, &rules);
        let mut is_initial_valid: bool = true;

        while violated_rule_index != None {
            is_initial_valid = false;
            let rule = rules[violated_rule_index.unwrap()];
            let indexes: Vec<i32> = get_indexes_from_rule(&pages, rule);
            shift_pages(&mut pages, indexes[0], indexes[1]);
            violated_rule_index = find_violated_rule(&pages, &rules);
        }

        if is_initial_valid {
            sum_one += string_to_int(get_middle_pages(&pages));
        }
        else {
            sum_two += string_to_int(get_middle_pages(&pages));
        }
    }
    
    println!("Day 3 Part One answer is {}", sum_one);
    println!("Day 3 Part Two answer is {}", sum_two);
}

fn shift_pages<'a>(pages: &mut Vec<&'a str>, from_index: i32, to_index: i32) -> Vec<&'a str> {
    if from_index < pages.len() as i32 && to_index < pages.len() as i32 {
        // Remove the element and store it
        let element = pages.remove(from_index as usize);

        // Insert the element at the new position
        pages.insert(to_index as usize, element);
    }
    pages.to_owned()
}

fn get_pages(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn find_violated_rule(pages: &Vec<&str>, rules: &Vec<&str>) -> Option<usize> {
    for i_start in 0..pages.len() {
        // check back
        if i_start > 0 {
            for i in 0..(i_start - 1) {
                let index: Option<usize> = get_rule(&pages[i], &pages[i_start], &rules);

                if index != None {
                    return index;
                }
            }
        }

        // check for
        if i_start < pages.len() {
            for i in i_start..pages.len() {
                let index: Option<usize> = get_rule(&pages[i_start], &pages[i], &rules);

                if index != None {
                    return index;
                }
            }
        }
    }
    None
}

fn get_rule(left: &str, right: &str, rules: &Vec<&str>) -> Option<usize> {
    if let Some(found) = rules.iter().position(|&s| s == (format!("{}|{}", right, left))) {
        return Option::from(found);
    }
    None
}

fn get_indexes_from_rule(pages: &Vec<&str>, rule: &str) -> Vec<i32> {
    let rule: Vec<&str> = rule.split('|').collect::<Vec<&str>>();
    let left: Option<usize> = pages.iter().position(|&s| s == rule[0]);
    let right: Option<usize> = pages.iter().position(|&s| s == rule[1]);
    vec![left.unwrap() as i32,right.unwrap() as i32]
}

fn get_middle_pages<'a>(input: &Vec<&'a str>) -> &'a str {
    let index_of_middle: usize = input.len()/2;
    input[index_of_middle]
}
