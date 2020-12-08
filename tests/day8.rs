#[cfg(test)]
use advent_2020::day8;
use advent_2020::day8::Inst;
use advent_2020::day8::Operation;
use advent_2020::day8::Program;
use advent_2020::utils::read_lines;

#[test]
fn test_day8_inst_new() {
    assert_eq!(Inst::new("nop 3").op, Operation::Nop);
    assert_eq!(Inst::new("jmp 3").op, Operation::Jmp);
    assert_eq!(Inst::new("acc 3").op, Operation::Acc);
    assert_eq!(Inst::new("acc 3").arg, 3);
}

#[test]
fn test_day8_program_new() {
    let data = read_lines("./data/day8_test.txt");
    let program = Program::new(&data);
    assert_eq!(program.instructions.len(), 9);
    assert_eq!(program.instructions[0].op, Operation::Nop);
    assert_eq!(program.instructions[2].op, Operation::Jmp);
    assert_eq!(program.instructions[2].arg, 4);
}

#[test]
fn test_day8_program_step() {
    let data = read_lines("./data/day8_test.txt");
    let mut program = Program::new(&data);
    program.step();
    assert_eq!(program.inst_ptr, 1);
    program.step();
    assert_eq!(program.inst_ptr, 2);
    assert_eq!(program.accumulator, 1);
    program.step();
    assert_eq!(program.inst_ptr, 6);
    assert_eq!(program.accumulator, 1);
    program.step();
    assert_eq!(program.inst_ptr, 7);
    assert_eq!(program.accumulator, 2);
}

#[test]
fn test_day8_solve_a() {
    let data = read_lines("./data/day8_test.txt");
    assert_eq!(day8::solve_a(&data), 5);
}

#[test]
fn test_day8_solve_b() {
    let data = read_lines("./data/day8_test.txt");
    assert_eq!(day8::solve_b(&data), 8);
}
