#![warn(clippy::all, clippy::pedantic)]
use std::time::Instant;
use utils::get_file_contents;

#[derive(Debug)]
struct Operation {
    opcode: usize,
    operand: usize,
}

#[derive(Debug)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

struct Handheld {
    register: Registers,
    operation: Operation,
}

impl Handheld {
    pub(crate) fn run_program() {
        
    }
    
    fn execute(&mut self, op: &Operation) {
        match op.opcode { 
            0 => self.adv(op),
            1 => self.bxl(op),
            2 => self.bst(op),
            3 => {
                if self.register.a != 0 {

                }
            },
            4 => self.bxl(op),
            5 => self.out(op),
            6 => self.bdv(op),
            7 => self.cdv(op),
            _ => {}
        }
    }
    
    fn adv(&mut self, op: &Operation) {
        self.register.a /= 2_usize.pow(u32::try_from(op.operand).expect("REASON"));
    }
    
    fn bxl(&mut self, op: &Operation) {
        self.register.b ^= op.operand;
    }
    
    fn bst(&mut self, op: &Operation) {
        self.register.b = op.operand % 8;
    }
    
    fn jnz(&mut self, op: &Operation) {
        if self.register.a != 0 {
            todo!()
        }
    }
    
    fn bxc(&mut self, _: &Operation) {
        self.register.b ^= self.register.c;
    }
    
    fn out(&mut self, op: &Operation) {
        op.operand % 8
    }
    
    fn bdv(&mut self, op: &Operation) {
        
    }

    fn cdv(&mut self, op: &Operation) {

    }
}

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let (ops, regs) = preamble(file_path);

    println!("Ops: {ops:?}, regs: {regs:?}");

    // println!("Day 16 Part One answer is {}", get_result_part_one(&grid));
    // println!("Day 16 Part Two answer is {}", get_result_part_two(&grid));

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> (Vec<Operation>, Registers) {
    let file_content: String = get_file_contents(file_path);
    let mut operations: Vec<Operation> = Vec::new();
    let mut lines = file_content.lines();
    let reg_a = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let reg_b = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines.next();

    let ops = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    for i in (0..ops.len()).step_by(2) {
        operations.push(Operation {
            opcode: ops[i].parse::<usize>().unwrap(),
            operand: ops[i + 1].parse::<usize>().unwrap(),
        });
    }

    let regs = Registers {
        a: reg_a,
        b: reg_b,
        c: reg_c,
    };
    (operations, regs)
}

// fn get_result_part_one(grid: &Grid) -> usize {
//     0
// }
//
// fn get_result_part_two(grid: &Grid) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_sample_part_one() {
    //     let grid = preamble("sample");
    //     assert_eq!(get_result_part_one(&grid), 11048);
    // }

    // #[test]
    // fn test_input_part_one() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_one(&grid), 72400);
    // }

    // #[test]
    // fn test_sample_part_two() {
    //     let grid = preamble("sample");
    //     assert_eq!(get_result_part_two(&grid), 64);
    // }

    // #[test]
    // fn test_input_part_two() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_two(&grid), 435);
    // }
}
