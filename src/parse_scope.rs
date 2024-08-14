use std::collections::HashMap;

use half::f16;

use crate::{_type::{Type, Types}, function::{Extern, Function}, instruction::{Instruction, Opcode}, scope::Scope, value::{Value, Values}};

// expects `index` to be at the start of the scope body
pub fn parse_scope(bytes: &Vec<u8>, index: &mut usize) -> Result<Scope, String> {
    let mut scope: Scope = Scope { instructions: Vec::new(), scopes: Vec::new(), functions: HashMap::new(), externs: HashMap::new() };

    while *index < bytes.len() {
        match bytes[*index] {
            0xFF => {
                *index += 1;

                let func = parse_function(bytes, index)?;
                scope.functions.insert(func.name.clone(), func);
            }
            0xFE => {
                *index += 1;
                scope.scopes.push(parse_scope(bytes, index)?);
            }
            0xFD => {
                *index += 1;
                break;
            }
            0xFC => {
                break;
            }
            0xF9 => {
                *index += 1;

                let func = parse_extern(bytes, index)?;
                scope.externs.insert(func.name.clone(), func);
            }
            _ => {
                scope.instructions.push(parse_instruction(bytes, index)?);
            }
        }
    }

    return Ok(scope);
}

// expects `index` to be at the start of the extern
// leaves `index` to be the byte after the extern
pub fn parse_extern(bytes: &Vec<u8>, index: &mut usize) -> Result<Extern, String> {
    let ret_type = parse_type(bytes, index)?;

    let name = parse_bytecode_string(bytes, index)?;

    let mut arg_types: Vec<Type> = Vec::new();
    while bytes[*index] != 0xF8 {
        arg_types.push(parse_type(bytes, index)?);
        let _ = parse_bytecode_string(bytes, index)?; // :)
    }

    *index += 1;

    let dll = parse_bytecode_string(bytes, index)?;

    return Ok(Extern { name, ret_type, arg_types, dll });
}

// expects `index` to be at the start of the instruction
// leaves `index` to be the byte after the instruction
pub fn parse_instruction(bytes: &Vec<u8>, index: &mut usize) -> Result<Instruction, String> {
    let opcode_byte = bytes[*index];

    let start_index = *index;

    *index += 1;

    let opcode = match opcode_byte {
        0x00 => {
            Opcode::NOP
        }

        0x01 => {
            Opcode::PUSH_IMM(parse_immediate(bytes, index)?)
        }
        0x02 => {
            Opcode::PUSH_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x03 => {
            Opcode::POP(parse_bytecode_string(bytes, index)?)
        }

        0x04 => {
            Opcode::PEEK_IMM(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x05 => {
            Opcode::PEEK_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x06 => {
            Opcode::CALL_FUNC(parse_bytecode_string(bytes, index)?)
        }
        0x07 => {
            Opcode::CALL_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x08 => {
            Opcode::ADD_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x09 => {
            Opcode::ADD_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0A => {
            Opcode::ADD_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0B => {
            Opcode::ADD_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x0C => {
            Opcode::SUB_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0D => {
            Opcode::SUB_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0E => {
            Opcode::SUB_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0F => {
            Opcode::SUB_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x10 => {
            Opcode::MUL_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x11 => {
            Opcode::MUL_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x12 => {
            Opcode::MUL_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x13 => {
            Opcode::MUL_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x14 => {
            Opcode::DIV_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x15 => {
            Opcode::DIV_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x16 => {
            Opcode::DIV_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x17 => {
            Opcode::DIV_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x18 => {
            Opcode::JMP_IMM(parse_immediate(bytes, index)?)
        }
        0x19 => {
            Opcode::JMP_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x1A => {
            Opcode::JNE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1B => {
            Opcode::JNE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1C => {
            Opcode::JNE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1D => {
            Opcode::JNE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1E => {
            Opcode::JNE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x1F => {
            Opcode::JNE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x20 => {
            Opcode::JNE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x21 => {
            Opcode::JNE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x22 => {
            Opcode::JE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x23 => {
            Opcode::JE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x24 => {
            Opcode::JE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x25 => {
            Opcode::JE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x26 => {
            Opcode::JE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x27 => {
            Opcode::JE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x28 => {
            Opcode::JE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x29 => {
            Opcode::JE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x2A => {
            Opcode::JGE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2B => {
            Opcode::JGE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2C => {
            Opcode::JGE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2D => {
            Opcode::JGE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2E => {
            Opcode::JGE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x2F => {
            Opcode::JGE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x30 => {
            Opcode::JGE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x31 => {
            Opcode::JGE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x32 => {
            Opcode::JG_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x33 => {
            Opcode::JG_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x34 => {
            Opcode::JG_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x35 => {
            Opcode::JG_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x36 => {
            Opcode::JG_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x37 => {
            Opcode::JG_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x38 => {
            Opcode::JG_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x39 => {
            Opcode::JG_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x3A => {
            Opcode::JLE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3B => {
            Opcode::JLE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3C => {
            Opcode::JLE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3D => {
            Opcode::JLE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3E => {
            Opcode::JLE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x3F => {
            Opcode::JLE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x40 => {
            Opcode::JLE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x41 => {
            Opcode::JLE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x42 => {
            Opcode::JL_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x43 => {
            Opcode::JL_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x44 => {
            Opcode::JL_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x45 => {
            Opcode::JL_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x46 => {
            Opcode::JL_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x47 => {
            Opcode::JL_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x48 => {
            Opcode::JL_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x49 => {
            Opcode::JL_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x4A => {
            Opcode::MOV_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x4B => {
            Opcode::MOV_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x4C => {
            Opcode::MOV_VV_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x4D => {
            Opcode::MOV_I_VV(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x4E => {
            Opcode::MOV_V_VV(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x4F => {
            Opcode::MOV_VV_VV(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x66 => {
            Opcode::VAR_TYPE_NAME(parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x67 => {
            Opcode::VAR_VAR_NAME(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x68 => {
            Opcode::VAR_TYPE_VAR(parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x69 => {
            Opcode::VAR_VAR_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x6A => {
            Opcode::RET
        }
        0x6B => {
            Opcode::RET_IMM(parse_immediate(bytes, index)?)
        }
        0x6C => {
            Opcode::RET_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x6D => {
            Opcode::DEREF_IMM(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x6E => {
            Opcode::DEREF_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x6F => {
            Opcode::REF_IMM(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x70 => {
            Opcode::REF_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x73 => {
            Opcode::MOD_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x74 => {
            Opcode::MOD_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x75 => {
            Opcode::MOD_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x76 => {
            Opcode::MOD_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x77 => {
            Opcode::PMOV_IMM_IMM(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x78 => {
            Opcode::PMOV_VAR_IMM(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x79 => {
            Opcode::PMOV_IMM_VAR(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x7A => {
            Opcode::PMOV_VAR_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x7B => {
            Opcode::ALLOC_TYPE_IMM(parse_type(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x7C => {
            Opcode::ALLOC_VAR_IMM(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x7D => {
            Opcode::ALLOC_TYPE_VAR(parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x7E => {
            Opcode::ALLOC_VAR_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x7F => {
            Opcode::FREE_VAR(parse_bytecode_string(bytes, index)?)
        }
        0x80 => {
            Opcode::FREE_IMM_IMM(parse_immediate(bytes, index)?, 
            parse_immediate(bytes, index)?)
        }
        0x81 => {
            Opcode::FREE_VAR_IMM(parse_bytecode_string(bytes, index)?, 
            parse_immediate(bytes, index)?)
        }
        0x82 => {
            Opcode::FREE_IMM_VAR(parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x83 => {
            Opcode::FREE_VAR_VAR(parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }

        _ => return Err(format!("unknown instruction {:#04x} at {:#06x}", opcode_byte, start_index))
    };

    return Ok(Instruction { index: start_index, opcode: opcode });
}

// expects `index` to be at the start of the function definition
// leaves `index` to be the byte after the function
pub fn parse_function(bytes: &Vec<u8>, index: &mut usize) -> Result<Function, String> {
    let ret_type = parse_type(bytes, index)?;

    let name = parse_bytecode_string(bytes, index)?;

    let mut arg_types: Vec<Type> = Vec::new();
    let mut arg_names: Vec<String> = Vec::new();
    while bytes[*index] != 0xFE {
        arg_types.push(parse_type(bytes, index)?);
        arg_names.push(parse_bytecode_string(bytes, index)?);
    }

    *index += 1;
    let scope = parse_scope(bytes, index)?;

    return Ok(Function { name: name, ret_type: ret_type, arg_types: arg_types, arg_names: arg_names, scope: scope });
}

// expects `index` to be at the start of the type
// leaves `index` at the byte after the type
pub fn parse_type(bytes: &[u8], index: &mut usize) -> Result<Type, String> {
    let mut typ = Type {typ: Vec::new()};

    while bytes[*index] == 0x0C {
        typ.typ.push(Types::POINTER);
        *index += 1;
    }

    typ.typ.push(Types::from_u8(bytes[*index]));
    *index += 1;

    return Ok(typ);
}

// expects `index` to be at the start of the string
// leaves `index` at byte after end of string
pub fn parse_bytecode_string(bytes: &[u8], index: &mut usize) -> Result<String, String> {
    let len = bytes[*index] as usize;

    *index += 1;

    if *index + len > bytes.len() {
        return Err(format!("bytecode string length at {:#06x} went out of bounds (length: {len})\neither the parser is incorrectly reading a string,\nor the length is set too high", *index-1));
    }

    match String::from_utf8(bytes[*index..*index+len].to_vec()) {
        Ok(s) => {
            *index += len;

            Ok(s)
        }
        Err(error) => Err(error.to_string())
    }
}

pub fn parse_dyn_number(bytes: &[u8], index: &mut usize) -> Result<usize, String> {
    let typ = bytes[*index];
    *index += 1;

    let res;
    match typ {
        1 => {
            res = Ok(i8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap()) as usize);
            *index += 1;
        }
        2 => {
            res = Ok(i16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap()) as usize);
            *index += 2;
        }
        3 => {
            res = Ok(i32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap()) as usize);
            *index += 4;
        }
        4 => {
            res = Ok(i64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap()) as usize);
            *index += 8;
        }
        5 => {
            res = Ok(u8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap()) as usize);
            *index += 1;
        }
        6 => {
            res = Ok(u16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap()) as usize);
            *index += 2;
        }
        7 => {
            res = Ok(u32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap()) as usize);
            *index += 4;
        }
        8 => {
            res = Ok(u64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap()) as usize);
            *index += 8;
        }
        _ => res = Err(format!("unsupported byte length {typ}"))
    }

    return res;
}

// expects `index` to be at the start of the immediate
// leaves `index` at byte after end of immediate
pub fn parse_immediate(bytes: &[u8], index: &mut usize) -> Result<Value, String> {
    let typ = bytes[*index];
    
    *index += 1;

    let value;
    match typ {
        0x00 => return Err("`VOID` is unsupported as an immediate value".to_string()),
        0x01 => {
            value = Values::SIGNED(i8::from_be_bytes(bytes[*index..*index+1].try_into().expect("immediate was incorrect length")) as i64);
            *index += 1;

        }
        0x02 => {
            value = Values::SIGNED(i16::from_be_bytes(bytes[*index..*index+2].try_into().expect("immediate was incorrect length")) as i64);
            *index += 2;

        }
        0x03 => {
            value = Values::SIGNED(i32::from_be_bytes(bytes[*index..*index+4].try_into().expect("immediate was incorrect length")) as i64);
            *index += 4;

        }
        0x04 => {
            value = Values::SIGNED(i64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")));
            *index += 8;

        }
        0x05 => {
            value = Values::UNSIGNED(u8::from_be_bytes(bytes[*index..*index+1].try_into().expect("immediate was incorrect length")) as u64);
            *index += 1;

        }
        0x06 => {
            value = Values::UNSIGNED(u16::from_be_bytes(bytes[*index..*index+2].try_into().expect("immediate was incorrect length")) as u64);
            *index += 2;

        }
        0x07 => {
            value = Values::UNSIGNED(u32::from_be_bytes(bytes[*index..*index+4].try_into().expect("immediate was incorrect length")) as u64);
            *index += 4;

        }
        0x08 => {
            value = Values::UNSIGNED(u64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")));
            *index += 8;

        }
        0x09 => {
            value = Values::DECIMAL(f16::to_f64(f16::from_be_bytes(bytes[*index..*index+2].try_into().expect("immediate was incorrect length"))));
            *index += 2;

        }
        0x0A => {
            value = Values::DECIMAL(f32::from_be_bytes(bytes[*index..*index+4].try_into().expect("immediate was incorrect length")) as f64);
            *index += 4;

        }
        0x0B => {
            value = Values::DECIMAL(f64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")));
            *index += 8;

        }
        0x0C => return Err("`POINTER` is unsupported as an immediate value".to_string()), // TODO: why?
        0x0D => return Err("`TYPE` is unsupported as an immediate value".to_string()),
        0x0E => return Err("`STRUCT` is unsupported as an immediate value".to_string()),
        0x0F => return Err("`NAME` is unsupported as an immediate value".to_string()), // TODO: why?
        _ => return Err(format!("unknown type {:#04x}", typ))
    }
    
    return Ok(Value { typ: Type { typ: vec![Types::from_u8(typ)] } , val: value });
}