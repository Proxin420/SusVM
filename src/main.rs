#![allow(warnings)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn push(mut stack: Vec<u64>, value: &str, destination: &str, registers: Vec<u64>) -> Vec<u64> {
    if check(destination) != 69 {
        let destination = check(destination);
        stack.push(registers[destination]);
    } else {
        stack.push(value.parse().unwrap());
    }
    return stack;
}

fn pop(mut stack: Vec<u64>, destination: &str, mut registers: Vec<u64>) -> (Vec<u64>, Vec<u64>) {
    let destination = check(destination);
    registers[destination] = stack.pop().unwrap();
    return (stack, registers);
}

fn mov(source: &str, destination: &str, mut registers: Vec<u64>) -> Vec<u64> {
    let source = check(source);
    let destination = check(destination);
    registers[destination] = registers[source];
    registers[source] = 0;
    return registers.clone();
}

fn cmp(source1: &str, source2: &str, mut registers: Vec<u64>, mut stack: Vec<u64>) -> Vec<u64> {
    let source1 = check(source1);
    let source2 = check(source2);
    if registers[source1] == registers[source2] {
        stack.push(1);
    } else if registers[source1] != registers[source2] {
        if registers[source1] > registers[source2] {
            stack.push(2);
        } else if registers[source1] < registers[source2] {
            stack.push(3);
        }
    } 
    return stack;
}

fn add(source: &str, destination: &str, mut registers: Vec<u64>) -> Vec<u64> {
    let source = check(source);
    let destination = check(destination);
    registers[destination] = registers[destination] + registers[source];
    return registers;
}

fn sub(source: &str, destination: &str, mut registers: Vec<u64>) -> Vec<u64> {
    let source = check(source);
    let destination = check(destination);
    registers[destination] = registers[destination] - registers[source];
    registers[source] = 0;
    return registers;
}

fn mul(source: &str, destination: &str, mut registers: Vec<u64>) -> Vec<u64> {
    let source = check(source);
    let destination = check(destination);
    registers[destination] = registers[destination] * registers[source];
    registers[source] = 0;
    return registers;
}

fn check(to_check: &str) -> usize {
    if to_check == "rax" {
        return 0;
    } else if to_check == "rcx" {
        return 1;
    } else if to_check == "rdx" {
        return 2;
    } else if to_check == "rbx" {
        return 3;
    } else if to_check == "rsi" {
        return 4;
    } else if to_check == "stack" {
        return 420;
    } else {
        return 69;
    }
}

fn parse(program_path: &str, mut program: Vec<(String, String, String)>) -> Vec<(String, String, String)> {
    let file = File::open(program_path).unwrap();
    let reader = BufReader::new(file);
    let mut line_count = 1;
    let mut section: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let mut comment = false;
        let mut ctr = 0;
        let mut arg0: &str = "";
        let mut arg1: &str = "";
        let mut arg2: &str = "";
        let f = line.unwrap();
        for op in f.split(" ") {
            if op != "" && comment == false {
                match ctr {
                    0 => { arg0 = op; },
                    1 => { arg1 = op; },
                    2 => { arg2 = op; },
                    _ => { panic!("Err: Max 3 arguments => (Line: {line_count}, token: {op})"); },
                }
            }
            if op.starts_with("//") {
                comment = true;
            }
            if arg0 == "mov" {
                if arg1 != "" && arg2 != "" {
                    program.push(("mov".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "push" {
                if arg1 != "" {
                    program.push(("push".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "pop" {
                if arg1 != "" {
                    if check(arg1) != 69 {
                        program.push(("pop".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                    }
                }
            } else if arg0 == "add" {
                if arg1 != "" && arg2 != "" {
                    program.push(("add".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "sub" {
                if arg1 != "" && arg2 != "" {
                    program.push(("sub".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "mul" {
                if arg1 != "" && arg2 != "" {
                    program.push(("mul".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "cmp" {
                if arg1 != "" && arg2 != "" {
                    program.push(("cmp".to_string(), arg1.clone().to_string(), arg2.clone().to_string()));
                }
            } else if arg0 == "jmp" {
                if arg1 != "" {
                    program.push(("jmp".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                }
            } else if arg0 == "exit" {
                program.push(("exit".to_string(), "Empty".to_string(), "Empty".to_string()));
            } else if arg0 == "je" {
                if arg1 != "" {
                    program.push(("je".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                }
            } else if arg0 == "jne" {
                if arg1 != "" {
                    program.push(("jne".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                }
            } else if arg0 == "jb" {
                if arg1 != "" {
                    program.push(("jb".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                }
            } else if arg0 == "js" {
                if arg1 != "" {
                    program.push(("js".to_string(), arg1.clone().to_string(), "Empty".to_string()));
                }
            }
            ctr = ctr + 1;
        }
        line_count = line_count + 1;
    }
    return program;
}

fn execute(program: Vec<(String, String, String)>, mut registers: Vec<u64>, mut stack: Vec<u64>, mut memory: Vec<u64>) {
    let mut ip = 0; // Instruction Pointer
    while ip < program.len() {
        let op = program[ip].0.as_str();
        let op_arg1 = program[ip].1.as_str();
        let op_arg2 = program[ip].2.as_str();
        println!("{}", op);
        if op == "mov" {
            registers = mov(op_arg2, op_arg1, registers.clone());
        } else if op == "push" {
            stack = push(stack.clone(), op_arg1, op_arg1, registers.clone());
        } else if op == "pop" {
            (stack, registers) = pop(stack.clone(), op_arg1, registers.clone());
        } else if op == "add" {
            registers = add(op_arg1, op_arg2, registers.clone());
        } else if op == "sub" {
            registers = sub(op_arg1, op_arg2, registers.clone());
        } else if op == "mul" {
            registers = mul(op_arg1, op_arg2, registers.clone());
        } else if op == "cmp" {
            stack = cmp(op_arg1, op_arg2, registers.clone(), stack.clone());
        } else if op == "jmp" {
            ip = op_arg1.parse().unwrap();
        } else if op == "exit" {
            ip = program.len();
        } else if op == "je" {
            if stack.pop().unwrap() == 1 {
                ip = op_arg1.parse().unwrap();
            }
        } else if op == "jne" {
            if stack.pop().unwrap() != 1 {
                ip = op_arg1.parse().unwrap();
            }
        } else if op == "jb" {
            if stack.pop().unwrap() == 2 {
                ip = op_arg1.parse().unwrap();
            }
        } else if op == "js" {
            if stack.pop().unwrap() == 3 {
                ip = op_arg1.parse().unwrap();
            }
        }

        ip = ip + 1;
        println!("Registers: {:?}", registers);
        println!("Stack: {:?}", stack);
    }
}

fn main() {
    // Registers
    // rax: 0, rcx: 1, rdx: 2, rbx: 3, rsi: 4
    let registers: Vec<u64> = vec![0; 5];

    // Memory
    let memory: Vec<u64> = vec![0; 10000];
    let stack: Vec<u64> = Vec::new();
    let mut program: Vec<(String, String, String)> = Vec::new();

    program = parse("program.rvm", program);
    println!("{:?}", program);
    execute(program, registers, stack, memory);
}
