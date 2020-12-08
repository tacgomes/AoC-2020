use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, PartialEq)]
enum BootCodeResult {
    Terminated(i32),
    Cyclic(i32),
}

fn jmp_ip(ip: usize, jmp: i32) -> usize {
    (ip as isize + jmp as isize) as usize
}

struct BootCode {
    instructions: Vec<Instruction>,
}

impl BootCode {
    fn from_file(file_name: impl AsRef<Path>) -> Self {
        let file = File::open(file_name).unwrap();
        let lines = BufReader::new(file).lines();

        let mut instructions = vec![];

        for line in lines {
            let line = line.unwrap();
            let tokens: Vec<_> = line.split_whitespace().collect();
            let (op, val) = (tokens[0], tokens[1].parse::<i32>().unwrap());
            let ins = match op {
                "nop" => Instruction::Nop(val),
                "acc" => Instruction::Acc(val),
                "jmp" => Instruction::Jmp(val),
                _ => panic!("Invalid operation: {}", op),
            };
            instructions.push(ins);
        }

        BootCode { instructions }
    }

    fn from_instructions(instructions: Vec<Instruction>) -> Self {
        BootCode { instructions }
    }

    fn run(&self) -> BootCodeResult {
        let mut acc = 0;
        let mut ip = 0;
        let mut executed = vec![false; self.instructions.len()];

        while ip != self.instructions.len() {
            match executed[ip] {
                false => executed[ip] = true,
                true => return BootCodeResult::Cyclic(acc),
            }

            match self.instructions[ip] {
                Instruction::Nop(_) => {
                    ip += 1;
                }
                Instruction::Acc(val) => {
                    acc += val;
                    ip += 1;
                }
                Instruction::Jmp(val) => {
                    ip = jmp_ip(ip, val);
                }
            }
        }

        BootCodeResult::Terminated(acc)
    }

    fn run_with_fix(&self) -> BootCodeResult {
        for (ip, ins) in self.instructions.iter().enumerate() {
            match ins {
                Instruction::Nop(val) => {
                    let mut new_instructions = self.instructions.clone();
                    new_instructions[ip] = Instruction::Jmp(*val);
                    let r = BootCode::from_instructions(new_instructions).run();
                    if let BootCodeResult::Terminated(_) = r {
                        return r;
                    }
                }
                Instruction::Jmp(val) => {
                    let mut new_instructions = self.instructions.clone();
                    new_instructions[ip] = Instruction::Nop(*val);
                    let r = BootCode::from_instructions(new_instructions).run();
                    if let BootCodeResult::Terminated(_) = r {
                        return r;
                    }
                }
                Instruction::Acc(_) => (),
            }
        }
        self.run()
    }
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let boot_code = BootCode::from_file(env::args().nth(1).unwrap());
    let acc = boot_code.run();
    let run_with_fix = boot_code.run_with_fix();
    println!("Result (Part 1): {:?}", acc);
    println!("Result (Part 2): {:?}", run_with_fix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let boot_code = BootCode::from_file("example.txt");
        assert_eq!(boot_code.run(), BootCodeResult::Cyclic(5));
        assert_eq!(boot_code.run_with_fix(), BootCodeResult::Terminated(8));
    }

    #[test]
    fn test_puzzle_input() {
        let boot_code = BootCode::from_file("input.txt");
        assert_eq!(boot_code.run(), BootCodeResult::Cyclic(1810));
        assert_eq!(boot_code.run_with_fix(), BootCodeResult::Terminated(969));
    }
}
