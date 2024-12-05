use utils::{get_file_contents, string_to_int};

const FILE_PATH: &str = "input.txt";

fn main() {
    let file_content: String = get_file_contents(FILE_PATH);
    let sections: Vec<&str> = file_content.split("\n\n").collect();
    let rules: Vec<&str> = sections[0].lines().collect();
    let updates: Vec<&str> = sections[1].lines().collect();
    let mut sum: i32 = 0;

    for update in updates {
        let pages: Vec<&str> = get_pages(update);
        let is_valid: bool = is_update_valid(&pages, &rules);
        let middle_page_number: i32 = string_to_int(get_middle_pages(&pages));
        //println!("{} -> {} ({})", update, is_valid, middle_page_number);
        
        if is_valid {
            sum += middle_page_number;
        }
    }
    
    println!("Day 3 Part One answer is {}", sum);
}

fn get_pages(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn is_update_valid(pages: &Vec<&str>, rules: &Vec<&str>) -> bool {
    for i in 0..pages.len() {
        // check back
        if !is_valid_back(i as i32, &pages, &rules) {
            return false;
        }
        
        // check for
        if !is_valid_for(i as i32, &pages, &rules) {
            return false;
        }
        
        if i == 0 {
            
        }
    }
    
    true
}

fn is_valid_back(i_start: i32, pages: &Vec<&str>, rules: &Vec<&str>) -> bool {
    if i_start <= 0 {
        return true;
    }
    
    for i in 0..(i_start - 1) as usize {
        if is_a_rule_violated(&pages[i], &pages[i_start as usize], &rules) {
            return false;
        }
    }
    true
}

fn is_valid_for(i_start: i32, pages: &Vec<&str>, rules: &Vec<&str>) -> bool {
    if i_start >= pages.len() as i32 {
        return true;
    }

    for i in i_start as usize..pages.len() {
        if is_a_rule_violated(&pages[i_start as usize], &pages[i], &rules) {
            return false;
        }
    }
    true
}

fn is_a_rule_violated(left: &str, right: &str, rules: &Vec<&str>) -> bool {
    if let Some(found) = rules.iter().find(|&&s| s == (format!("{}|{}", right, left))) {
        //println!("A violated rule found: {}", found);
        return true
    }
    
    false
}

fn get_middle_pages<'a>(input: &Vec<&'a str>) -> &'a str {
    let index_of_middle: usize = input.len()/2;
    
    input[index_of_middle]
}
