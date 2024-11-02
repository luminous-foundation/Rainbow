use std::{collections::HashMap, fs, path::Path, usize};

use half::f16;

use crate::{_struct::Struct, _type::{Type, Types}, block::Block, frame::Frame, function::{Extern, Function}, instruction::{Instruction, Opcode}, module::Module, parse_program, scope::Scope, value::{Value, Values}};

// expects `index` to be at the start of the scope body
pub fn parse_scope(bytes: &Vec<u8>, stack: &mut Vec<Frame>, index: &mut usize, linker_paths: &Vec<String>, debug: bool, consts: &HashMap<String, i32>) -> Result<Scope, String> {
    let mut scope: Scope = Scope::new();

    while *index < bytes.len() {
        match bytes[*index] {
            0xFF => {
                *index += 1;

                let func = parse_function(bytes, stack, index, linker_paths, debug, consts)?;
                scope.functions.insert(func.name.clone(), func);
            }
            0xFE => {
                *index += 1;

                scope.add_block(Block::SCOPE(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
            }
            0xFD => {
                *index += 1;
                break;
            }
            0xFC => {
                break;
            }
            0xFB => {
                *index += 1;

                let strct = parse_struct(bytes, index)?;
                scope.structs.insert(strct.name.clone(), strct);
            }
            0xFA => {
                *index += 1;
                parse_import(bytes, stack, &mut scope, index, linker_paths, consts)?;
            }
            0xF9 => {
                *index += 1;
                let func = parse_extern(bytes, index)?;
                scope.externs.insert(func.name.clone(), func);
            }
            0xF7 => {
                *index += 1;

                let s = eval_conditional(bytes, stack, index, linker_paths, debug, consts)?;
                if s.is_some() {
                    let s = s.unwrap(); 
                    scope.merge(s);
                }
            }
            0xF6 => {
                *index += 1;
                let name = parse_bytecode_string(bytes, index)?;
                *index += 1;
                let module_scope = parse_scope(bytes, stack, index, linker_paths, debug, consts)?;

                let module = Module { name: name.clone(), scope: module_scope, frame: stack.len() };
                scope.modules.insert(name, module);

                stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });
            }
            _ => {
                if scope.blocks.len() == 0 {
                    scope.add_block(Block::CODE(Vec::new()));
                }

                let len = scope.blocks.len();
                match &mut scope.blocks[len-1] {
                    Block::CODE(vec) => vec.push(parse_instruction(bytes, index)?),
                    _ => scope.add_block(Block::CODE(Vec::new()))
                }
            }
        }
    }

    let clone = scope.clone();
    for block in &mut scope.blocks {
        match block {
            Block::SCOPE(s) => {
                s.parent_scope = Some(Box::new(clone.clone()));
            }
            _ => {}
        } 
    }

    for (_, func) in &mut scope.functions {
        func.scope.parent_scope = Some(Box::new(clone.clone()));
    }

    for (_, module) in &mut scope.modules {
        module.scope.parent_scope = Some(Box::new(clone.clone()));
    }

    return Ok(scope);
}

// this just skips past a scope
// throws away everything it parses
// not the most performant but whatever
fn skip_scope(bytes: &Vec<u8>, index: &mut usize) {
    let mut stack = Vec::new();
    let linker_paths = Vec::new();
    let debug = false;
    let consts = HashMap::new();

    while *index < bytes.len() {
        match bytes[*index] {
            0xFF => {
                *index += 1;
                let _ = parse_function(bytes, &mut stack, index, &linker_paths, debug, &consts);
            }
            0xFE => {
                *index += 1;
                skip_scope(bytes, index);
            }
            0xFD => {
                *index += 1;
                break;
            }
            0xFC => {
                break;
            }
            0xFB => {
                *index += 1;
                let _ = parse_struct(bytes, index);
            }
            0xFA => {
                *index += 1;
                skip_import(bytes, index);
            }
            0xF9 => {
                *index += 1;
                let _ = parse_extern(bytes, index);
            }
            0xF7 => {
                *index += 1;
                let _ = eval_conditional(bytes, &mut stack, index, &linker_paths, debug, &consts);
            }
            0xF6 => {
                *index += 1;
                let _ = parse_bytecode_string(bytes, index);
                *index += 1;
                skip_scope(bytes, index);
            }
            _ => {
                let _ = parse_instruction(bytes, index);
            }
        }
    }
}

// expects `index` to be at the byte after start of the conditional
// leaves `index` to be the byte after the conditional
fn eval_conditional(bytes: &Vec<u8>, stack: &mut Vec<Frame>, index: &mut usize, linker_paths: &Vec<String>, debug: bool, consts: &HashMap<String, i32>) -> Result<Option<Scope>, String> {
    while *index < bytes.len() {
        match bytes[*index] {
            0x00 | 0x01 => {
                *index += 1;
                let left_name = parse_bytecode_string(bytes, index)?;
                let condition = bytes[*index];
                *index += 1;
                let right_name = parse_bytecode_string(bytes, index)?;
                *index += 1;

                if !consts.contains_key(&left_name) {
                    if consts.contains_key(&left_name.to_uppercase()) {
                        return Err(format!("attempted to get unknown const `{left_name}`\na similarly named one `{}` exists", left_name.to_uppercase()));
                    }

                    return Err(format!("attempted to get unknown const `{left_name}`"));
                }

                let left = consts.get(&left_name).unwrap();

                if !consts.contains_key(&right_name) {
                    if consts.contains_key(&right_name.to_uppercase()) {
                        return Err(format!("attempted to get unknown const `{right_name}`\na similarly named one `{}` exists", right_name.to_uppercase()));
                    }

                    return Err(format!("attempted to get unknown const `{right_name}`"));
                }

                let right = consts.get(&right_name).unwrap();

                match condition {
                    0x00 => {
                        if left == right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    0x01 => {
                        if left != right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    0x02 => {
                        if left >= right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    0x03 => {
                        if left > right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    0x04 => {
                        if left <= right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    0x05 => {
                        if left < right {
                            return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
                        } else {
                            skip_scope(bytes, index);
                        }
                    }
                    _ => return Err(format!("unknown conditional `{:#04x}`", bytes[*index]))
                }
                
                *index += 1;
            },
            0x02 => {
                *index += 2;

                return Ok(Some(parse_scope(bytes, stack, index, linker_paths, debug, consts)?));
            }
            0x03 => {
                *index += 1;
                break
            }
            _ => return Err(format!("unknown conditional block type `{:#04x}`", bytes[*index]))
        }
    }

    return Ok(None);
}

// expects `index` to be at the start of the struct definition
// leaves `index` to be the byte after the struct
fn parse_struct(bytes: &Vec<u8>, index: &mut usize) -> Result<Struct, String> {
    let name = parse_bytecode_string(bytes, index)?;

    let mut strct = Struct { name, size: 0, var_names: Vec::new(), var_types: Vec::new(), var_offsets: HashMap::new() };

    *index += 1;

    let mut offset = 0;
    while bytes[*index] != 0xFD {
        let typ = parse_type(bytes, index)?;
        let name = parse_bytecode_string(bytes, index)?;
        
        // TODO: replace this with the size in bytes when byte-wise memory accesses are implemented
        strct.size += 1; //typ.get_size(); // TODO: structs are of unknown size! should we even handle this?

        strct.var_types.push(typ);
        strct.var_names.push(name.clone());
        strct.var_offsets.insert(name, offset);

        offset += 1;
    }

    *index += 1;

    return Ok(strct);
}

fn skip_import(bytes: &Vec<u8>, index: &mut usize) {
    let _ = parse_bytecode_string(bytes, index);
}

// expects `index` to be at the start of the import
// leaves `index` to be the byte after the import
fn parse_import(bytes: &Vec<u8>, stack: &mut Vec<Frame>, scope: &mut Scope, index: &mut usize, linker_paths: &Vec<String>, consts: &HashMap<String, i32>) -> Result<(), String> {
    let import = parse_bytecode_string(bytes, index)?;

    let mut import_path = String::new();
    if Path::exists(Path::new(&import)) {
        import_path = import.clone();
    }

    for path in linker_paths {
        let paths = get_paths(path).unwrap();

        for path in paths {
            if path.ends_with(&import) {
                if import_path == "" {
                    import_path = path;
                } else {
                    return Err(format!("ambiguous import {import}"));
                }
            }
        }
    }
    let mut new_scope = Scope::new();

    let program = fs::read(import_path.clone()).expect(&format!("failed to read import `{import}`"));
    parse_program(&program, stack, &mut new_scope, linker_paths, false, consts);

    scope.merge(new_scope);

    return Ok(());
}

fn get_paths(path: &String) -> Result<Vec<String>, String> {
    let mut path_queue: Vec<String> = Vec::new();
    let mut res = Vec::new();
    
    path_queue.push(path.to_string());

    while path_queue.len() > 0 {
        let path = path_queue.remove(0);
        let paths = match fs::read_dir(path.clone()) {
            Ok(p) => p,
            Err(e) => return Err(e.to_string()),
        };

        for path in paths {
            let dir_entry = path.unwrap();

            if dir_entry.metadata().unwrap().is_dir() {
                path_queue.push(dir_entry.path().as_os_str().to_str().unwrap().to_string());
            } else if dir_entry.metadata().unwrap().is_file() {
                res.push(dir_entry.path().as_os_str().to_str().unwrap().to_string());
            }
        }
    }

    return Ok(res);
}

// expects `index` to be at the start of the extern
// leaves `index` to be the byte after the extern
pub fn parse_extern(bytes: &Vec<u8>, index: &mut usize) -> Result<Extern, String> {
    let ret_type = parse_type(bytes, index)?;

    let name = parse_bytecode_string(bytes, index)?;

    let mut arg_types: Vec<Type> = Vec::new();
    while bytes[*index] != 0xF8 {
        arg_types.push(parse_type(bytes, index)?);
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
        // nop
        0x00 => {
            Opcode::NOP
        }

        // stack operations
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

        // function calling
        0x06 => {
            Opcode::CALL_FUNC(parse_bytecode_string(bytes, index)?)
        }
        0x07 => {
            Opcode::CALL_VAR(parse_bytecode_string(bytes, index)?)
        }

        // math operations
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

        // jumps
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
        
        // move instructions
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

        // bitwise operations
        0x50 => {
            Opcode::AND_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x51 => {
            Opcode::AND_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x52 => {
            Opcode::AND_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x53 => {
            Opcode::AND_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        
        0x54 => {
            Opcode::OR_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x55 => {
            Opcode::OR_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x56 => {
            Opcode::OR_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x57 => {
            Opcode::OR_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        
        0x58 => {
            Opcode::NOT_IMM(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x59 => {
            Opcode::NOT_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x5A => {
            Opcode::XOR_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x5B => {
            Opcode::XOR_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x5C => {
            Opcode::XOR_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x5D => {
            Opcode::XOR_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        
        0x5E => {
            Opcode::LSH_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x5F => {
            Opcode::LSH_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x60 => {
            Opcode::LSH_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x61 => {
            Opcode::LSH_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        
        0x62 => {
            Opcode::RSH_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x63 => {
            Opcode::RSH_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x64 => {
            Opcode::RSH_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x65 => {
            Opcode::RSH_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }

        // variable instructions
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

        // return
        0x6A => {
            Opcode::RET
        }
        0x6B => {
            Opcode::RET_IMM(parse_immediate(bytes, index)?)
        }
        0x6C => {
            Opcode::RET_VAR(parse_bytecode_string(bytes, index)?)
        }

        // pointer instructions
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

        // struct instantiation
        0x71 => {
            Opcode::INST_NAME(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x72 => {
            Opcode::INST_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        // modulo
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

        // more pointer instructions
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

        // callc instructions
        0x84 => {
            Opcode::CALLC_I_T_I(parse_immediate(bytes, index)?,
            parse_type(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x85 => {
            Opcode::CALLC_V_T_I(parse_bytecode_string(bytes, index)?,
            parse_type(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x86 => {
            Opcode::CALLC_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x87 => {
            Opcode::CALLC_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x88 => {
            Opcode::CALLC_I_T_V(parse_immediate(bytes, index)?,
            parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x89 => {
            Opcode::CALLC_V_T_V(parse_bytecode_string(bytes, index)?,
            parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x8A => {
            Opcode::CALLC_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x8B => {
            Opcode::CALLC_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        // compare instruction
        0x8C => {
            Opcode::CMP_I_I_I(parse_immediate(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x8D => {
            Opcode::CMP_V_I_I(parse_bytecode_string(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x8E => {
            Opcode::CMP_I_V_I(parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x8F => {
            Opcode::CMP_V_V_I(parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x90 => {
            Opcode::CMP_I_I_V(parse_immediate(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x91 => {
            Opcode::CMP_V_I_V(parse_bytecode_string(bytes, index)?, 
            parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x92 => {
            Opcode::CMP_I_V_V(parse_immediate(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }
        0x93 => {
            Opcode::CMP_V_V_V(parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?, 
            parse_bytecode_string(bytes, index)?)
        }

        _ => return Err(format!("unknown instruction {:#04x} at {:#06x}", opcode_byte, start_index))
    };

    return Ok(Instruction { index: start_index, opcode: opcode });
}

// expects `index` to be at the start of the function definition
// leaves `index` to be the byte after the function
pub fn parse_function(bytes: &Vec<u8>, stack: &mut Vec<Frame>, index: &mut usize, linker_paths: &Vec<String>, debug: bool, consts: &HashMap<String, i32>) -> Result<Function, String> {
    let ret_type = parse_type(bytes, index)?;

    let name = parse_bytecode_string(bytes, index)?;

    let mut arg_types: Vec<Type> = Vec::new();
    let mut arg_names: Vec<String> = Vec::new();
    while bytes[*index] != 0xFE {
        arg_types.push(parse_type(bytes, index)?);
        arg_names.push(parse_bytecode_string(bytes, index)?);
    }

    *index += 1;
    let scope = parse_scope(bytes, stack, index, linker_paths, debug, consts)?;

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
        Err(error) => Err(format!("error at {:#06x}: {}", *index-1, error.to_string()))
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
        0x0C => {
            value = Values::POINTER(usize::from_be_bytes(bytes[*index..*index+(usize::BITS/8) as usize].try_into().expect("immediate was incorrect length")), 0);
        }
        0x0D => return Err("`TYPE` is unsupported as an immediate value".to_string()),
        0x0E => return Err("`STRUCT` is unsupported as an immediate value".to_string()),
        0x0F => {
            value = Values::NAME(parse_bytecode_string(bytes, index)?);
        }
        _ => return Err(format!("unknown type {:#04x}", typ))
    }
    
    return Ok(Value { typ: Type { typ: vec![Types::from_u8(typ)] } , val: value });
}