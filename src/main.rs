use std::{collections::HashMap, fs};

use frame::Frame;
use scope::{exec_func, exec_scope, parse_scope};
use value::{Value, Values};

mod scope;
mod instruction;
mod function;
mod _type;
mod frame;
mod value;

// TODO: better error handling
// TODO: result type
fn main() {
    let program = fs::read("./counting.rbb").expect("failed to read program");

    let start = std::time::Instant::now();
    let mut index = 0;
    let global_scope = match parse_scope(&program, &mut index) {
        Ok(scope) => scope,
        Err(error) => panic!("failed to parse program:\n{}", error)
    };

    println!("parsing took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);

    let mut stack: Vec<Frame> = Vec::new();

    stack.push(Frame { vars: HashMap::new(), stack: Vec::new() });

    let exec_start = std::time::Instant::now();
    exec_scope(&global_scope, &mut stack, 0);
    
    if let Some(func) = global_scope.functions.get("main") { // main functions are not required
        exec_func(func, &mut stack);
    }
    println!("execution took {:.2}ms", exec_start.elapsed().as_secs_f32() * 1000f32);
    println!("whole program took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);

    println!("{:#?}", stack);
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