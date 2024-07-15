use std::collections::HashMap;

use half::f16;

use crate::{_type::{Type, Types}, argument::Argument, frame::Frame, function::Function, get_var, instruction::Instruction, set_var, value::{Value, Values}, variable::Variable};

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
// TODO: variable scoping that's more precise than function scope
pub fn exec_scope(scope: &Scope, stack: &mut Vec<Frame>, cur_frame: usize) {
    for instr in &scope.instructions {
        match instr.opcode {
            0x02 => { // PUSH [name]
                let name;
                match &instr.args[0] {
                    Argument::NAME(n) => name = n,
                    _ => panic!("expected NAME, got {:?}", instr.args[0]),
                }

                let var = get_var(name.clone(), stack, cur_frame).clone();
                stack[cur_frame].push(var);
            }
            0x08 => { // ADD [imm] [imm] [name]
                let a;
                match &instr.args[0] {
                    Argument::IMM(v) => a = v,
                    _ => panic!("expected IMM, got {:?}", instr.args[0])
                }
                let b;
                match &instr.args[1] {
                    Argument::IMM(v) => b = v,
                    _ => panic!("expected IMM, got {:?}", instr.args[1])
                }

                let out;
                match &instr.args[2] {
                    Argument::NAME(name) => out = name,
                    _ => panic!("expected NAME, got {:?}", instr.args[2]),
                }

                // TODO: types :why:
                // this is just a temporary thing to get it working
                let var = get_var(out.clone(), stack, cur_frame);
                let new_val = a.val.add(&b.val);

                set_var(out.clone(), Value { main_type: var.value.main_type.clone(), val: new_val }, stack, cur_frame)
            }
            0x62 => { // VAR [type] [name]
                let typ;
                match &instr.args[0] {
                    Argument::TYPE(t) => typ = t,
                    _ => panic!("expected TYPE, got {:?}", instr.args[0]),
                }
                let name;
                match &instr.args[1] {
                    Argument::NAME(n) => name = n,
                    _ => panic!("expected NAME, got {:?}", instr.args[1]),
                }
                stack[cur_frame].push_var(name.clone(), typ.clone());
            }
            0x63 => { // VAR [name] [name]
                let type_var;
                match &instr.args[0] {
                    Argument::NAME(name) => type_var = get_var(name.clone(), stack, cur_frame),
                    _ => panic!("expected NAME, got {:?}", instr.args[0]),
                }

                let typ;
                match &type_var.value.val {
                    Values::TYPE(t) => typ = t.clone(),
                    _ => panic!("tried to create variable with dynamic type stored in variable, but given variable had type {:?}", type_var.value.main_type)
                }
                let name;
                match &instr.args[1] {
                    Argument::NAME(n) => name = n,
                    _ => panic!("expected NAME, got {:?}", instr.args[1]),
                }
                stack[cur_frame].push_var(name.clone(), typ);
            }
            _ => panic!("unknown instruction {:#04x} at {:#06x}", instr.opcode, instr.index)
        }
    }
}

pub fn exec_func(func: &Function, stack: &mut Vec<Frame>) {
    stack.push(Frame { vars: HashMap::new(), stack: Vec::new() });

    let len = stack.len(); // borrow checker woes
    exec_scope(&func.scope, stack, len - 1);
}

// expects `index` to be at the start of the scope body
pub fn parse_scope(bytes: &Vec<u8>, index: &mut usize) -> Result<Scope, String> {
    let mut scope: Scope = Scope {instructions: Vec::new(), scopes: Vec::new(), functions: HashMap::new()};

    while *index < bytes.len() {
        println!("{:#06x}: {:#04x}", *index, bytes[*index]);
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

    *index += 1;

    let mut args: Vec<Argument> = Vec::new();
    match opcode {
        0x00 => (),
        0x01 => {
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
        }
        0x02 | 0x03 => {
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x04 => {
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
        }
        0x05 | 0x06 | 0x07 => {
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x08 => {
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x09 => {
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x0A => {
            args.push(Argument::IMM(parse_immediate(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x0B => {
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x62 => {
            args.push(Argument::TYPE(parse_type(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x63 => {
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
            args.push(Argument::NAME(parse_bytecode_string(bytes, index)?));
        }
        0x65 => {
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

    *index += 1;
    let scope = parse_scope(bytes, index)?;

    return Ok(Function { name: name, ret_type: ret_type, arg_types: arg_types, arg_names: arg_names, scope: scope });
}

// expects `index` to be at the start of the type
// leaves `index` at the byte after the type
pub fn parse_type(bytes: &Vec<u8>, index: &mut usize) -> Result<Type, String> {
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
pub fn parse_bytecode_string(bytes: &Vec<u8>, index: &mut usize) -> Result<String, String> {
    let len = bytes[*index] as usize;

    *index += 1;

    match String::from_utf8(bytes[*index..*index+len].to_vec()) {
        Ok(s) => {
            *index += len;

            Ok(s)
        }
        Err(error) => Err(error.to_string())
    }
}

// expects `index` to be at the start of the immediate
// leaves `index` at byte after end of immediate
pub fn parse_immediate(bytes: &Vec<u8>, index: &mut usize) -> Result<Value, String> {
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
            value = Values::SIGNED(i64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")) as i64);
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
            value = Values::UNSIGNED(u64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")) as u64);
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
            value = Values::DECIMAL(f64::from_be_bytes(bytes[*index..*index+8].try_into().expect("immediate was incorrect length")) as f64);
            *index += 8;

        }
        0x0C => return Err("`POINTER` is unsupported as an immediate value".to_string()),
        0x0D => return Err("`TYPE` is unsupported as an immediate value".to_string()),
        0x0E => return Err("`STRUCT` is unsupported as an immediate value".to_string()),
        0x0F => return Err("`NAME` is unsupported as an immediate value".to_string()),
        _ => return Err(format!("unknown type {:#04x}", typ))
    }
    
    return Ok(Value { main_type: Types::from_u8(typ), val: value });
}