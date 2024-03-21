use std::fs;

use types::parse_string;

use crate::jump::{je_imm_imm_imm, je_imm_imm_var, je_imm_var_imm, je_imm_var_var, je_var_imm_imm, je_var_imm_var, je_var_var_imm, je_var_var_var, jg_imm_imm_imm, jg_imm_imm_var, jg_imm_var_imm, jg_imm_var_var, jg_var_imm_imm, jg_var_imm_var, jg_var_var_imm, jg_var_var_var, jge_imm_imm_imm, jge_imm_imm_var, jge_imm_var_imm, jge_imm_var_var, jge_var_imm_imm, jge_var_imm_var, jge_var_var_imm, jge_var_var_var, jl_imm_imm_imm, jl_imm_imm_var, jl_imm_var_imm, jl_imm_var_var, jl_var_imm_imm, jl_var_imm_var, jl_var_var_imm, jl_var_var_var, jle_imm_imm_imm, jle_imm_imm_var, jle_imm_var_imm, jle_imm_var_var, jle_var_imm_imm, jle_var_imm_var, jle_var_var_imm, jle_var_var_var, jmp_imm, jmp_var, jne_imm_imm_imm, jne_imm_imm_var, jne_imm_var_imm, jne_imm_var_var, jne_var_imm_imm, jne_var_imm_var, jne_var_var_imm, jne_var_var_var};
use crate::math::{add_imm_imm, add_imm_var, add_var_imm, add_var_var, div_imm_imm, div_imm_var, div_var_imm, div_var_var, mul_imm_imm, mul_imm_var, mul_var_imm, mul_var_var, sub_imm_imm, sub_imm_var, sub_var_imm, sub_var_var};
use crate::types::{parse_imm, Types};
use crate::stack::Frame;

mod types;
mod stack;
mod math;
mod jump;

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
            0x00 => {/* do nothing */}
            
            0x01 => push_imm(&mut stack, current_frame, &program, &mut pc),
            0x02 => push_var(&mut stack, current_frame, &program, &mut pc),
            
            0x03 => pop(&mut stack, current_frame, &program, &mut pc),

            0x04 => ldarg_imm(&mut stack, &program, &mut pc),
            0x05 => ldarg_var(&mut stack, &program, &mut pc),
            
            0x08 => add_imm_imm(&mut stack, current_frame, &program, &mut pc),
            0x09 => add_var_imm(&mut stack, current_frame, &program, &mut pc),
            0x0A => add_imm_var(&mut stack, current_frame, &program, &mut pc),
            0x0B => add_var_var(&mut stack, current_frame, &program, &mut pc),
            
            0x0C => sub_imm_imm(&mut stack, current_frame, &program, &mut pc),
            0x0D => sub_var_imm(&mut stack, current_frame, &program, &mut pc),
            0x0E => sub_imm_var(&mut stack, current_frame, &program, &mut pc),
            0x0F => sub_var_var(&mut stack, current_frame, &program, &mut pc),
            
            0x10 => mul_imm_imm(&mut stack, current_frame, &program, &mut pc),
            0x11 => mul_var_imm(&mut stack, current_frame, &program, &mut pc),
            0x12 => mul_imm_var(&mut stack, current_frame, &program, &mut pc),
            0x13 => mul_var_var(&mut stack, current_frame, &program, &mut pc),
            
            0x14 => div_imm_imm(&mut stack, current_frame, &program, &mut pc),
            0x15 => div_var_imm(&mut stack, current_frame, &program, &mut pc),
            0x16 => div_imm_var(&mut stack, current_frame, &program, &mut pc),
            0x17 => div_var_var(&mut stack, current_frame, &program, &mut pc),

            0x18 => jmp_imm(&program, &mut pc),
            0x19 => jmp_var(&mut stack, current_frame, &program, &mut pc),

            // function names about to go wild
            0x1A => jne_imm_imm_imm(&program, &mut pc),
            0x1B => jne_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x1C => jne_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x1D => jne_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x1E => jne_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x1F => jne_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x20 => jne_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x21 => jne_var_var_var(&mut stack, current_frame,&program, &mut pc),
            
            0x22 => je_imm_imm_imm(&program, &mut pc),
            0x23 => je_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x24 => je_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x25 => je_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x26 => je_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x27 => je_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x28 => je_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x29 => je_var_var_var(&mut stack, current_frame,&program, &mut pc),

            0x2A => jge_imm_imm_imm(&program, &mut pc),
            0x2B => jge_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x2C => jge_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x2D => jge_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x2E => jge_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x2F => jge_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x30 => jge_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x31 => jge_var_var_var(&mut stack, current_frame,&program, &mut pc),
            
            0x32 => jg_imm_imm_imm(&program, &mut pc),
            0x33 => jg_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x34 => jg_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x35 => jg_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x36 => jg_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x37 => jg_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x38 => jg_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x39 => jg_var_var_var(&mut stack, current_frame,&program, &mut pc),

            0x3A => jle_imm_imm_imm(&program, &mut pc),
            0x3B => jle_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x3C => jle_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x3D => jle_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x3E => jle_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x3F => jle_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x40 => jle_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x41 => jle_var_var_var(&mut stack, current_frame,&program, &mut pc),
            
            0x42 => jl_imm_imm_imm(&program, &mut pc),
            0x43 => jl_var_imm_imm(&mut stack, current_frame,&program, &mut pc),
            0x44 => jl_imm_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x45 => jl_var_var_imm(&mut stack, current_frame,&program, &mut pc),
            0x46 => jl_imm_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x47 => jl_var_imm_var(&mut stack, current_frame,&program, &mut pc),
            0x48 => jl_imm_var_var(&mut stack, current_frame,&program, &mut pc),
            0x49 => jl_var_var_var(&mut stack, current_frame,&program, &mut pc),

            0x4A => mov_imm(&mut stack, current_frame, &program, &mut pc),
            0x4B => mov_var(&mut stack, current_frame, &program, &mut pc),

            0x5F => create_var_imm(&mut stack, current_frame, &program, &mut pc),
            0x60 => create_var_var(&mut stack, current_frame, &program, &mut pc),
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

fn get_var(stack: &mut Vec<Frame>, current_frame: usize, name: String) -> Box<Types> {
    let frame = &mut stack[current_frame];
    if frame.has_var(name.clone()) {
        return frame.get_var(&name);
    }
    return stack[0].get_var(&name);
}

fn parse_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) -> Box<Types> {
    let name = parse_string(program, pc);
    return get_var(stack, current_frame, name);
}

fn push_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    stack[current_frame].push(parse_imm(program, pc));
}

fn push_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var = parse_var(stack, current_frame, program, pc);

    stack[current_frame].push(var);
}

fn pop(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name = parse_string(program, pc);
    let value = (*frame).pop();

    (*frame).set_var(name, value);
}

fn ldarg_imm(stack: &mut Vec<Frame>, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[0]; // global frame

    let value = parse_imm(program, pc);

    frame.push(value);
}

fn ldarg_var(stack: &mut Vec<Frame>, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[0]; // global frame

    let value = parse_imm(program, pc);

    frame.push(value);
}

fn mov_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let value = parse_imm(program, pc);

    let name = parse_string(program, pc);
    frame.set_var(name, value);
}

fn mov_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var = parse_var(stack, current_frame, program, pc);

    let frame = &mut stack[current_frame];

    let name = parse_string(program, pc);
    frame.set_var(name, var);
}

fn create_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let t = program[*pc];
    *pc += 1;

    let name = parse_string(program, pc);

    println!("attempting to create variable named {}", name);

    stack[current_frame].create_var(name, t);
}

fn create_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let t = *parse_var(stack, current_frame, program, pc);

    let name2 = parse_string(program, pc);

    println!("attempting to create variable named {}", name2);
    match t {
        Types::U8(value) => stack[current_frame].create_var(name2, value),
        _ => println!("cannot use non-type typed variable to create variable"), // one hell of an error message
    }
}