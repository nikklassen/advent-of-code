use crate::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day08");
}

enum Instruction {
    Nop(i32),
    Jump(i32),
    Acc(i32),
}

fn read_instructions() -> Vec<Instruction> {
    INPUT.iter().map(parse_instruction).collect()
}

fn parse_instruction<S: AsRef<str>>(s: S) -> Instruction {
    let parts = s.as_ref().split(' ').collect::<Vec<_>>();
    let val = parts[1].parse().unwrap();
    match parts[0] {
        "nop" => Instruction::Nop(val),
        "acc" => Instruction::Acc(val),
        "jmp" => Instruction::Jump(val),
        _ => unreachable!(),
    }
}

fn run_instructions(insts: &[Instruction]) -> i32 {
    let mut acc = 0i32;
    let mut pc = 0;
    let mut visited = vec![false; insts.len()];
    while pc < insts.len() {
        if visited[pc] {
            return acc;
        }
        visited[pc] = true;
        match insts[pc] {
            Instruction::Jump(offset) => {
                pc = ((pc as i32) + offset) as usize;
            }
            Instruction::Acc(val) => {
                acc += val;
                pc += 1;
            }
            Instruction::Nop(_) => {
                pc += 1;
            }
        }
    }
    acc
}

pub fn part1() -> i32 {
    let insts = read_instructions();
    run_instructions(&insts)
}

#[derive(Clone)]
struct StackFrame {
    acc: i32,
    pc: usize,
    flipped: bool,
}

fn flip(insts: &mut [Instruction], pc: usize) {
    if let Instruction::Acc(_) = insts[pc] {
        return;
    }
    insts[pc] = match &insts[pc] {
        Instruction::Jump(offset) => Instruction::Nop(*offset),
        Instruction::Nop(offset) => Instruction::Jump(*offset),
        _ => unreachable!(),
    };
}

fn rewind(insts: &mut [Instruction], stack: &mut Vec<StackFrame>, visited: &mut [bool]) {
    let mut find_next = !stack.iter().any(|s| s.flipped);
    loop {
        let frame = stack.pop().unwrap();
        if find_next {
            if let Instruction::Acc(_) = insts[frame.pc] {
                continue;
            }
            flip(insts, frame.pc);
            stack.push(StackFrame {
                acc: frame.acc,
                pc: frame.pc,
                flipped: true,
            });
            return;
        }
        visited[frame.pc] = false;
        if frame.flipped {
            flip(insts, frame.pc);
            find_next = true;
        }
    }
}

fn run_instructions2(insts: &mut [Instruction]) -> i32 {
    let mut exec_stack = vec![StackFrame {
        acc: 0,
        pc: 0,
        flipped: false,
    }];
    let mut visited = vec![false; insts.len()];
    loop {
        let frame = exec_stack[exec_stack.len() - 1].clone();
        visited[frame.pc] = true;
        let new_frame = match insts[frame.pc] {
            Instruction::Jump(offset) => StackFrame {
                acc: frame.acc,
                pc: ((frame.pc as i32) + offset) as usize,
                flipped: false,
            },
            Instruction::Acc(val) => StackFrame {
                acc: frame.acc + val,
                pc: frame.pc + 1,
                flipped: false,
            },
            Instruction::Nop(_) => StackFrame {
                acc: frame.acc,
                pc: frame.pc + 1,
                flipped: false,
            },
        };
        if new_frame.pc >= insts.len() {
            break;
        }
        if visited[new_frame.pc] {
            rewind(insts, &mut exec_stack, &mut visited);
        } else {
            exec_stack.push(new_frame);
        }
    }
    exec_stack.last().unwrap().acc
}

pub fn part2() -> i32 {
    let mut insts = read_instructions();
    run_instructions2(&mut insts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 1317);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 1033);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
