use std::fs;

use crate::stack::Frame;
use crate::types::{parse_string, Types};
use crate::scope::Scope;

mod jump;
mod math;
mod stack;
mod types;
mod scope;
mod function;

fn main() {
    // load the program to be run
    let program = fs::read("./program.rbb").expect("the file no exist :(");

    // create the stack
    let mut stack: Vec<Frame> = Vec::new();

    // program counter and current stack frame
    let mut pc: usize = 0;
    let mut current_frame = 0;

    // for timing how long the program takes to run
    let start_time = std::time::Instant::now();

    // the global scope, parent of everything
    let mut global_scope = Scope::new();
    global_scope.parse(&program);

    let mut end_time = std::time::Instant::now();
    let mut result_ms = end_time.duration_since(start_time).as_millis();

    println!("parsed {} bytes in {}ms", program.len(), result_ms);

    global_scope.exec();

    end_time = std::time::Instant::now();
    result_ms = end_time.duration_since(start_time).as_millis();

    println!("executed {} bytes in {}ms", program.len(), result_ms);

    // stack debug code
    for frame in stack.iter() {
        for value in (*frame.stack).iter() {
            match **value {
                Types::I8(inner_value) => println!("Value in stack: {} i8", inner_value),
                Types::I16(inner_value) => println!("Value in stack: {} i16", inner_value),
                Types::I32(inner_value) => println!("Value in stack: {} i32", inner_value),
                Types::I64(inner_value) => println!("Value in stack: {} i64", inner_value),
                Types::U8(inner_value) => println!("Value in stack: {} u8", inner_value),
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

// variables inside parent scopes arent seen here
// although, maybe it's not a problem since new stack frames are only created through function calls....
// so in reality...
// TODO: add scoping to variables so we dont end up with a scope-everywhere kind of thing
fn get_var(stack: &mut Vec<Frame>, current_frame: usize, name: String) -> Box<Types> {
    let frame = &mut stack[current_frame];
    if frame.has_var(name.clone()) {
        return frame.get_var(&name);
    }
    return stack[0].get_var(&name);
}

fn parse_var(
    stack: &mut Vec<Frame>,
    current_frame: usize,
    program: &Vec<u8>,
    pc: &mut usize,
) -> Box<Types> {
    let name = parse_string(program, pc);
    return get_var(stack, current_frame, name);
}

fn get_instruction_length(code: &Vec<u8>, pc: usize) -> usize {
    let mut offset = pc;

    let byte = code[offset];
    offset += 1;

    match byte {
        0x00 => { /* do nothing */ },

        _ => panic!("cannot parse instruction {}", format!("0x{:02x}", byte)),
    }

    return offset - pc;
}
