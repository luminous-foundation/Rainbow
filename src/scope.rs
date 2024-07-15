use std::collections::HashMap;

use crate::{_type::{Type, Types}, argument::Argument, function::Function, instruction::Instruction};

#[derive(Debug)]
pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
}

// TODO: make scopes in scopes preserve instruction order
// example:
// ...
// {
//   these instructions should be executed where they are
//   but as it stands they are executed last
// }
// ...
pub fn exec_scope(scope: &Scope) {
    for instr in &scope.instructions {
        match instr.opcode {
            _ => panic!("unknown instruction {:#04x} at {:#06x}", instr.opcode, instr.index)
        }
    }
}

// expects `index` to be at the start of the scope body
pub fn parse_scope(bytes: &Vec<u8>, index: &mut usize) -> Result<Scope, String> {
    let mut scope: Scope = Scope {instructions: Vec::new(), scopes: Vec::new(), functions: HashMap::new()};

    while *index < bytes.len() {
        println!("{:#06x}: {:#04x}", *index, bytes[*index]);
        match bytes[*index] {
            0xFF => {
                *index = *index + 1;

                let func = parse_function(bytes, index)?;
                scope.functions.insert(func.name.clone(), func);
            }
            0xFE => {
                *index = *index + 1;
                scope.scopes.push(parse_scope(bytes, index)?);
            }
            0xFD => break,
            _ => {
                scope.instructions.push(parse_instruction(bytes, index)?);
            }
        }
    }

    return Ok(scope);
}

// expects `index` to be at the start of the instruction
// leaves `index` to be the byte after the instruction
pub fn parse_instruction(bytes: &Vec<u8>, index: &mut usize) -> Result<Instruction, String> {
    let opcode = bytes[*index];

    let start_index = *index;

    let mut args: Vec<Argument> = Vec::new();
    match opcode {
        0x00 | 0x64 => {
            *index = *index + 1;
        }
        0x05 | 0x06 | 0x07 | 0x65 => {
            *index = *index + 1;
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        _ => return Err(format!("unknown instruction {:#04x} at {:#06x}", opcode, index))
    }

    return Ok(Instruction { index: start_index, opcode: opcode, args: args });
}

// expects `index` to be at the start of the function definition
// leaves `index` to be the byte after the function
pub fn parse_function(bytes: &Vec<u8>, index: &mut usize) -> Result<Function, String> {
    let ret_type = parse_type(bytes, index)?;

    let name = parse_bytecode_string(bytes, index)?;

    println!("found function named {}", name);

    let mut arg_types: Vec<Type> = Vec::new();
    let mut arg_names: Vec<String> = Vec::new();
    while bytes[*index] != 0xFE {
        arg_types.push(parse_type(bytes, index)?);
        arg_names.push(parse_bytecode_string(bytes, index)?);
    }

    *index = *index + 1;
    let scope = parse_scope(bytes, index)?;

    return Ok(Function { name: name, ret_type: ret_type, arg_types: arg_types, arg_names: arg_names, scope: scope });
}

// expects `index` to be at the start of the type
// leaves `index` at the byte after the type
pub fn parse_type(bytes: &Vec<u8>, index: &mut usize) -> Result<Type, String> {
    let mut typ = Type {typ: Vec::new()};

    while bytes[*index] == 0x0C {
        typ.typ.push(Types::POINTER);
        *index = *index + 1;
    }

    typ.typ.push(Types::from_u8(bytes[*index]));
    *index = *index + 1;

    return Ok(typ);
}

// expects `index` to be at the start of the string
// leaves `index` at byte after end of string
pub fn parse_bytecode_string(bytes: &Vec<u8>, index: &mut usize) -> Result<String, String> {
    let len = bytes[*index] as usize;

    *index = *index + 1;

    match String::from_utf8(bytes[*index..*index+len].to_vec()) {
        Ok(s) => {
            *index = *index + len;

            Ok(s)
        }
        Err(error) => Err(error.to_string())
    }
}