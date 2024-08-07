use std::{collections::HashMap, env, fs};

use frame::Frame;
use function::{Extern, Function};
use scope::Scope;
use parse_scope::parse_scope;
use exec_scope::{exec_func, exec_scope};
use value::{Value, Values};

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
    let start = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();

    let program = fs::read(args[1].clone()).expect("failed to read program");

    let mut index = 0;
    let global_scope = match parse_scope(&program, &mut index) {
        Ok(scope) => scope,
        Err(error) => panic!("failed to parse program:\n{}", error)
    };

    println!("parsing took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);

    let mut stack: Vec<Frame> = Vec::new();

    stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });

    let exec_start = std::time::Instant::now();
    exec_scope(&global_scope, &global_scope, &mut stack, 0);
    
    if let Some(func) = global_scope.functions.get("main") { // main functions are not required
        exec_func(func, &global_scope, &mut stack);
    }
    // println!("execution took {:.2}ms", exec_start.elapsed().as_secs_f32() * 1000f32);
    println!("whole program took {:.6}s", start.elapsed().as_secs_f32());

    println!("{:#?}", stack);
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