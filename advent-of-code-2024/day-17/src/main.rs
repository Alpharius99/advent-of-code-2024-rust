#![warn(clippy::all)]
use std::time::Instant;
use utils::get_file_contents;

fn bench<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let t0 = Instant::now();
    let result = f(); // Call the function and store the result
    println!("time used: {:?}", Instant::now().duration_since(t0));
    result // Return the result of the function
}

#[derive(Debug, Clone)]
struct Operation {
    opcode: usize,
    operand: usize,
}

#[derive(Debug, Clone)]
struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Clone)]
struct Handheld {
    register: Registers,
    operation: Vec<Operation>,
    op_pointer: usize,
    output: Vec<usize>,
    listing: String,
    program: Vec<usize>,
}

impl Handheld {
    pub fn run_program(&mut self) {
        let ops = self.operation.clone();

        while self.op_pointer < self.operation.len() {
            // println!("{:?}", ops[self.op_pointer]);
            self.execute_op(&ops[self.op_pointer]);
        }
    }

    fn execute_op(&mut self, op: &Operation) {
        match op.opcode {
            0 => self.register.a = self.xdv(op.operand),
            1 => self.register.b ^= op.operand,
            2 => self.register.b = self.get_value_combo_operand(op.operand) % 8,
            3 => {
                if self.register.a != 0 {
                    self.op_pointer = op.operand;
                    return;
                }
            }
            4 => self.register.b ^= self.register.c,
            5 => {
                let value = self.get_value_combo_operand(op.operand);
                self.append_to_output(value % 8);
            }
            6 => self.register.b = self.xdv(op.operand),
            7 => self.register.c = self.xdv(op.operand),
            _ => {}
        }

        self.op_pointer += 1;
    }

    fn get_value_combo_operand(&mut self, co: usize) -> usize {
        match co {
            x if x <= 3 => x,
            4 => self.register.a,
            5 => self.register.b,
            6 => self.register.c,
            _ => panic!("Bad combo operand: {co}"),
        }
    }

    fn append_to_output(&mut self, x: usize) {
        self.output.push(x);
    }

    fn xdv(&mut self, x: usize) -> usize {
        let value = self.get_value_combo_operand(x);
        self.register.a / 2_usize.pow(u32::try_from(value).expect("REASON"))
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn calculate_initial_register_a(&mut self) -> Option<usize> {
        let mut stack = vec![(0, self.program.len())];
        println!("Program: {:?}", self.program);
        while let Some((a, depth)) = stack.pop() {
            // If we ever get to the beginning of the program, we have a solution.
            println!("Depth: {depth}");
            if depth == 0 {
                // println!("Stack: {stack:?}");
                return Some(a);
            }

            // Try all possible values for b and push ones that produce the correct
            // result on the stack.
            for b in 0..8 {
                let a = (a << 3) | b;
                println!(
                    "a = {}, prog step {}",
                    self.simulate_loop(a),
                    self.program[depth - 1]
                );
                if self.simulate_loop(a) == self.program[depth - 1] {
                    stack.push((a, depth - 1));
                    println!("Stack after push: {stack:?}");
                }
            }
            println!("Stack: {stack:?}");
        }
        None
    }

    fn simulate_loop(&self, a: usize) -> usize {
        let mut b = a % 8;
        b ^= 3;
        let c = a >> b;
        b ^= c;
        b ^= 3;
        b % 8
    }
}

fn main() {
    let start_time = Instant::now();
    #[cfg(feature = "debug")]
    let file_path: &str = "sample";
    #[cfg(not(feature = "debug"))]
    let file_path: &str = "input";

    let mut handheld = preamble(file_path);

    handheld.run_program();

    println!("Day 17 Part One answer is {}", handheld.get_output());
    println!(
        "Day 17 Part Two answer is {:?}",
        handheld.calculate_initial_register_a().unwrap()
    );

    println!("Execution time: {:.2?}", start_time.elapsed());
}

fn preamble(file_path: &str) -> Handheld {
    let file_content: String = get_file_contents(file_path);
    let mut operations: Vec<Operation> = Vec::new();
    let mut program: Vec<usize> = Vec::new();
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

    let listing = lines.next().unwrap().split(": ").last().unwrap();
    let ops = listing.split(',').collect::<Vec<&str>>();
    for i in (0..ops.len()).step_by(2) {
        operations.push(Operation {
            opcode: ops[i].parse::<usize>().unwrap(),
            operand: ops[i + 1].parse::<usize>().unwrap(),
        });
        program.push(ops[i].parse::<usize>().unwrap());
        program.push(ops[i + 1].parse::<usize>().unwrap());
    }

    let registers = Registers {
        a: reg_a,
        b: reg_b,
        c: reg_c,
    };

    Handheld {
        operation: operations,
        register: registers,
        output: Vec::new(),
        op_pointer: 0,
        listing: listing.to_string(),
        program: program,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part_one() {
        let mut handheld = preamble("sample");

        handheld.run_program();
        assert_eq!(handheld.get_output(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_input_part_one() {
        let mut handheld = preamble("input");
        handheld.run_program();
        assert_eq!(handheld.get_output(), "3,1,4,3,1,7,1,6,3");
    }

    #[test]
    fn test_sample_part_two() {
        let mut handheld = preamble("sample");

        handheld.run_program();
        assert_eq!(handheld.calculate_initial_register_a().unwrap(), 117_440);
    }

    // #[test]
    // fn test_input_part_two() {
    //     let grid = preamble("input");
    //     assert_eq!(get_result_part_two(&grid), 44_354);
    // }
}
