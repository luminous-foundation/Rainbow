use std::fs;

use types::add;

use crate::types::{parse_imm, Types};
use crate::stack::Frame;

mod types;
mod stack;

fn main() {
    let program = fs::read("./program.rbb").expect("the file no exist :(");

    let mut stack: Vec<Frame> = Vec::new();

    let mut pc: usize = 0;
    let mut current_frame = 0;

    let start_time = std::time::Instant::now();

    stack.push(Frame::new());

    while pc < program.len() {
        let byte = program[pc];
        pc += 1;

        match byte {
            0x01 => push_imm(&mut stack, current_frame, &program, &mut pc),
            0x08 => add_imm_imm(&mut stack, current_frame, &program, &mut pc),
            0x74 => create_var(&mut stack, current_frame, &program, &mut pc),
            _ => {
                panic!("unknown instruction {}", format!("0x{:02x}", byte));
            }
        }
    }
    let end_time = std::time::Instant::now();
    let result_ms = end_time.duration_since(start_time).as_millis();

    println!("executed {} bytes in {}ms", program.len(), result_ms);

    for frame in stack.iter() {
        for value in (*frame.stack).iter() {
            match **value {
                Types::I8(inner_value)   => println!("Value in stack: {} i8", inner_value),
                Types::I16(inner_value) => println!("Value in stack: {} i16", inner_value),
                Types::I32(inner_value) => println!("Value in stack: {} i32", inner_value),
                Types::I64(inner_value) => println!("Value in stack: {} i64", inner_value),
                Types::U8(inner_value)   => println!("Value in stack: {} u8", inner_value),
                Types::U16(inner_value) => println!("Value in stack: {} u16", inner_value),
                Types::U32(inner_value) => println!("Value in stack: {} u32", inner_value),
                Types::U64(inner_value) => println!("Value in stack: {} u64", inner_value),
                Types::F16(inner_value) => println!("Value in stack: {} f16", inner_value),
                Types::F32(inner_value) => println!("Value in stack: {} f32", inner_value),
                Types::F64(inner_value) => println!("Value in stack: {} f64", inner_value),
                _ => println!("Value of unexpected type in stack"),
            }
        }
    }

    println!("program done :)");
}

fn push_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    stack[current_frame].stack.push(parse_imm(program, pc));
}

fn add_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = add(&parse_imm(program, pc), &parse_imm(program, pc));

    let length = program[*pc] as usize;
    *pc += 1;

    let name = String::from_utf8(program[*pc..(*pc+length)].try_into().unwrap()).unwrap();
    *pc += length;

    stack[current_frame].set_var(name, sum);
}

fn create_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let t = program[*pc];
    let length = program[*pc + 1] as usize;
    *pc += 2;

    let name = String::from_utf8(program[*pc..(*pc+length)].try_into().unwrap()).unwrap();

    println!("attempting to create variable named {}", name);

    stack[current_frame].create_var(name, t);

    *pc += length;
}