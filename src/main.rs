use std::{collections::HashMap, env, fs};

use _type::Types;
use frame::Frame;
use function::{Extern, Function};
use scope::Scope;
use parse_scope::{parse_bytecode_string, parse_dyn_number, parse_scope, parse_type};
use exec_scope::{exec_func, exec_scope};
use value::{Value, Values};
use half::f16;

mod scope;
mod parse_scope;
mod exec_scope;
mod instruction;
mod function;
mod _type;
mod frame;
mod value;
mod ffi;

// TODO: better error handling
// TODO: result type
fn main() {
    // let start = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();

    let program = fs::read(args[1].clone()).expect("failed to read program");

    let mut index = 0;
    let global_scope = match parse_scope(&program, &mut index) {
        Ok(scope) => scope,
        Err(error) => panic!("failed to parse program:\n{}", error)
    };

    // println!("parsing took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);

    let mut stack: Vec<Frame> = Vec::new();

    stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });

    match parse_data_section(&program, &mut stack, &mut index) {
        Ok(_) => (),
        Err(error) => panic!("failed to parse data:\n{error}")
    }

    // let exec_start = std::time::Instant::now();
    exec_scope(&global_scope, &global_scope, &mut stack, 0);
    
    if let Some(func) = global_scope.functions.get("main") { // main functions are not required
        exec_func(func, &global_scope, &mut stack);
    }

    // println!("execution took {:.2}ms", exec_start.elapsed().as_secs_f32() * 1000f32);
    // println!("whole program took {:.6}s", start.elapsed().as_secs_f32());

    // println!("{:#?}", stack);
}

fn parse_data_section(bytes: &Vec<u8>, stack: &mut Vec<Frame>, index: &mut usize) -> Result<(), String> {
    if *index == bytes.len() {
        return Ok(());
    }

    while bytes[*index] != 0xFC {
        *index += 1;
    }

    *index += 1;

    while *index < bytes.len() {
        let name = parse_bytecode_string(bytes, index)?;
        let typ = parse_type(bytes, index)?;

        let len = parse_dyn_number(bytes, index)?;

        let i = stack[0].stack.len();
        stack[0].push_var(&name, typ.clone(), Values::POINTER(i + 1, len));

        // TODO: macroize
        match typ.typ[0] {
            Types::POINTER => {
                for _ in 0..len {
                    match typ.typ[1] {
                        Types::I8 => {
                            let val = i8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap());
                            *index += 1;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I16 => {
                            let val = i16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap());
                            *index += 2;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I32 => {
                            let val = i32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I64 => {
                            let val = i64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::U8 => {
                            let val = u8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap());
                            *index += 1;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U16 => {
                            let val = u16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap());
                            *index += 2;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U32 => {
                            let val = u32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U64 => {
                            let val = u64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::F16 => {
                            let val = f16::to_f64(f16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap()));
                            *index += 2;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        Types::F32 => {
                            let val = f32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        Types::F64 => {
                            let val = f64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[0].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        _ => panic!("unsupported data section type {:?}", typ.typ),
                    }
                }
            }
            _ => panic!("unsupported data section type {:?}", typ.typ),
        }
    }

    Ok(())
}

// this function expects the function to exist
// if it doesnt, it will crash
fn get_func<'a>(name: &String, scope: &'a Scope, global_scope: &'a Scope) -> &'a Function {
    if scope.functions.contains_key(name) {
        return scope.functions.get(name).unwrap();
    } else if global_scope.functions.contains_key(name) {
        return global_scope.functions.get(name).unwrap();
    } else {
        panic!("tried to call undefined function {}", name);
    }
}

// this function expects the extern to exist
// if it doesnt, it will crash
fn get_extern<'a>(name: &String, scope: &'a Scope, global_scope: &'a Scope) -> &'a Extern {
    if scope.externs.contains_key(name) {
        return scope.externs.get(name).unwrap();
    } else if global_scope.externs.contains_key(name) {
        return global_scope.externs.get(name).unwrap();
    } else {
        panic!("tried to call undefined function {}", name);
    }
}

fn func_exists(name: &String, scope: &Scope, global_scope: &Scope) -> bool {
    return scope.functions.contains_key(name) || global_scope.functions.contains_key(name);
}

// these functions expect the variable to exist
// if it doesnt, it will crash (it was going to crash later anyways)
fn get_var<'a>(name: &String, stack: &'a mut [Frame], cur_frame: usize) -> &'a Value {
    if stack[0].vars.contains_key(name) {
        return stack[0].get_var(name);
    } else {
        return stack[cur_frame].get_var(name);
    }
}

fn set_var(name: &String, value: &Values, stack: &mut [Frame], cur_frame: usize) {
    if name == "_" {
        return;
    }

    if stack[0].vars.contains_key(name) {
        stack[0].set_var(name, value);
    } else {
        if stack[cur_frame].vars.contains_key(name) {
            stack[cur_frame].set_var(name, value);
        } else {
            panic!("tried to set undefined variable {}", name);
        }
    }
}