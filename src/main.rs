use std::fs;

use frame::Frame;
use scope::{exec_func, exec_scope, parse_scope};

mod scope;
mod instruction;
mod function;
mod _type;
mod argument;
mod number;
mod frame;
mod variable;
mod value;

fn main() {
    let program = fs::read("./simple_add.rbb").expect("failed to read program");

    let mut index = 0;
    let global_scope;
    match parse_scope(&program, &mut index) {
        Ok(scope) => global_scope = scope,
        Err(error) => panic!("failed to parse program:\n{}", error)
    }

    let mut stack: Vec<Frame> = Vec::new();

    exec_scope(&global_scope, &mut stack[0]);
    match global_scope.functions.get("main") {
        Some(func) => exec_func(&func, &mut stack),
        None => (), // main functions are not required
    }

    // run the program :)
}