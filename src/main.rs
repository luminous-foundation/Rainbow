use std::fs;

use scope::{exec_scope, parse_scope};

mod scope;
mod instruction;
mod function;
mod _type;
mod argument;
mod number;
mod frame;
mod variable;

fn main() {
    let program = fs::read("C:\\Users\\Grim\\Desktop\\stuff\\rasm\\tests\\simple_add.rbb").expect("failed to read program");

    let mut index = 0;
    let global_scope;
    match parse_scope(&program, &mut index) {
        Ok(scope) => global_scope = scope,
        Err(error) => panic!("failed to parse program:\n{}", error)
    }

    exec_scope(&global_scope);
    match global_scope.functions.get("main") {
        Some(func) => exec_scope(&func.scope),
        None => panic!("could not find main function")
    }

    // run the program :)
}