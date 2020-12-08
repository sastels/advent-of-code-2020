use crate::utils::join_lines;
use regex::Regex;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Nop,
    Jmp,
    Acc,
}

pub struct Inst {
    pub op: Operation,
    pub arg: i32,
}

impl Inst {
    pub fn new(op: &str, arg: &str) -> Self {
        let arg = arg.parse::<i32>().unwrap();
        match op {
            "acc" => Inst {
                op: Operation::Acc,
                arg,
            },
            "jmp" => Inst {
                op: Operation::Jmp,
                arg,
            },
            "nop" => Inst {
                op: Operation::Nop,
                arg,
            },
            _ => panic!(),
        }
    }
}

pub struct Program {
    pub instructions: Vec<Inst>,
    pub inst_ptr: usize,
    pub accumulator: i32,
    pub finished: bool,
    pub loop_detected: bool,
}

impl Program {
    pub fn new(data: &[String]) -> Self {
        let data = &join_lines(data)[0];
        let re = Regex::new(r"(\w\w\w) ([+-]?\d+)").unwrap();

        let instructions: Vec<Inst> = re
            .captures_iter(data)
            .map(|cap| Inst::new(&cap[1], &cap[2]))
            .collect();

        Program {
            instructions,
            inst_ptr: 0,
            accumulator: 0,
            finished: false,
            loop_detected: false,
        }
    }

    pub fn reset(&mut self) {
        self.inst_ptr = 0;
        self.accumulator = 0;
        self.finished = false;
        self.loop_detected = false;
    }

    pub fn step(&mut self) {
        let inst = &self.instructions[self.inst_ptr];
        match inst.op {
            Operation::Acc => {
                self.accumulator += inst.arg;
                self.inst_ptr += 1;
            }
            Operation::Jmp => {
                self.inst_ptr = (self.inst_ptr as i32 + inst.arg).try_into().unwrap();
            }
            Operation::Nop => {
                self.inst_ptr += 1;
            }
        }
    }

    pub fn run(&mut self) {
        let mut inst_ptrs_seen: HashSet<usize> = HashSet::new();
        loop {
            if inst_ptrs_seen.contains(&self.inst_ptr) {
                self.loop_detected = true;
                break;
            }
            if self.inst_ptr >= self.instructions.len() {
                self.finished = true;
                break;
            }
            inst_ptrs_seen.insert(self.inst_ptr);
            self.step();
        }
    }
}

pub fn solve_a(data: &[String]) -> i32 {
    let mut program = Program::new(&data);
    program.run();
    program.accumulator
}

pub fn solve_b(data: &[String]) -> i32 {
    let mut program = Program::new(&data);

    let mut index = 0;
    loop {
        program.reset();
        match program.instructions[index].op {
            Operation::Jmp => {
                program.instructions[index].op = Operation::Nop;
                program.run();
                if program.finished {
                    break;
                }
                program.instructions[index].op = Operation::Jmp;
            }
            Operation::Nop => {
                program.instructions[index].op = Operation::Jmp;
                program.run();
                if program.finished {
                    break;
                }
                program.instructions[index].op = Operation::Nop;
            }
            _ => {}
        }
        index += 1;
    }
    program.accumulator
}
