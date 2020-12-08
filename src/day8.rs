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
    pub fn new(line: &str) -> Self {
        let re = Regex::new(r"(\w\w\w) (.+)").unwrap();
        let cap = re.captures(&line).unwrap();
        let arg = cap[2].parse::<i32>().unwrap();
        match &cap[1] {
            "acc" => {
                return Inst {
                    op: Operation::Acc,
                    arg,
                }
            }
            "jmp" => {
                return Inst {
                    op: Operation::Jmp,
                    arg,
                }
            }
            "nop" => {
                return Inst {
                    op: Operation::Nop,
                    arg,
                }
            }
            _ => panic!(),
        }
    }
}

pub struct Program {
    pub instructions: Vec<Inst>,
    pub inst_ptr: usize,
    pub accumulator: i32,
    pub finished: bool,
}

impl Program {
    pub fn new(data: &[String]) -> Self {
        Program {
            instructions: data.iter().map(|s| Inst::new(s)).collect(),
            inst_ptr: 0,
            accumulator: 0,
            finished: false,
        }
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
}

pub fn solve_a(data: &[String]) -> i32 {
    let mut program = Program::new(&data);
    let mut inst_ptrs_seen: HashSet<usize> = HashSet::new();

    loop {
        if inst_ptrs_seen.contains(&program.inst_ptr) {
            break;
        }
        inst_ptrs_seen.insert(program.inst_ptr);
        program.step();
    }
    program.accumulator
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
