#![warn(clippy::all)]
use regex::Regex;
use std::time::Instant;
use utils::get_file_contents;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Button {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct ClawMachine {
    a: Button,
    b: Button,
    prize: Button,
}

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";
    
    let data = preamble(file_path);

    println!("Day 13 Part One answer is {}", get_result_part_one(&data));
    println!("Day 13 Part Two answer is {}", get_result_part_two(&data));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> Vec<ClawMachine> {
    let file_content: String = get_file_contents(file_path);
    let lines: Vec<&str> = file_content.lines().collect();
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let re_bx = Regex::new(r"X\+(\d+)").unwrap();
    let re_by = Regex::new(r"Y\+(\d+)").unwrap();
    let re_px = Regex::new(r"X=(\d+)").unwrap();
    let re_py = Regex::new(r"Y=(\d+)").unwrap();

    for chunk in lines.chunks(4) {
        claw_machines.push(ClawMachine {
            a: Button {
                x: get_value(chunk[0], &re_bx),
                y: get_value(chunk[0], &re_by),
            },
            b: Button {
                x: get_value(chunk[1], &re_bx),
                y: get_value(chunk[1], &re_by),
            },
            prize: Button {
                x: get_value(chunk[2], &re_px),
                y: get_value(chunk[2], &re_py),
            },
        });
    }

    fn get_value(s: &str, re: &Regex) -> isize {
        re.captures(s)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<isize>()
            .unwrap()
    }

    claw_machines
}

fn get_result_part_one(data: &[ClawMachine]) -> usize {
    let mut num_tokens = 0;
    for x in data.iter() {
        if let Some(t) = solve_machine(x, 0) {
            #[cfg(feature = "debug")]
            println!("{x:?}: {t}");
            num_tokens += t
        }
    }
    num_tokens as usize
}

fn get_result_part_two(data: &[ClawMachine]) -> usize {
    let mut num_tokens = 0;
    for x in data.iter() {
        if let Some(t) = solve_machine(x, 10_000_000_000_000) {
            #[cfg(feature = "debug")]
            println!("{x:?}: {t}");
            num_tokens += t
        }
    }
    num_tokens as usize
}

fn solve_machine(cm: &ClawMachine, offset: isize) -> Option<isize> {
    let prize = (cm.prize.x + offset, cm.prize.y + offset);
    let det = cm.a.x * cm.b.y - cm.a.y * cm.b.x;
    let a = (prize.0 * cm.b.y - prize.1 * cm.b.x) / det;
    let b = (cm.a.x * prize.1 - cm.a.y * prize.0) / det;
    if (cm.a.x * a + cm.b.x * b, cm.a.y * a + cm.b.y * b) == (prize.0, prize.1) {
        Some(a * 3 + b)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let data = preamble("sample");
        assert_eq!(get_result_part_one(&data), 480);
    }

    #[test]
    fn test_sample_part_two() {
        let data = preamble("sample");
        assert_eq!(get_result_part_two(&data), 875318608908);
    }

    #[test]
    fn test_input_part_one() {
        let data = preamble("input");
        assert_eq!(get_result_part_one(&data), 36838);
    }

    #[test]
    fn test_input_part_two() {
        let data = preamble("input");
        assert_eq!(get_result_part_two(&data), 83029436920891);
    }
}
