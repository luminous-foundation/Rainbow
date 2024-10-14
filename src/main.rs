use std::{collections::HashMap, env, fs, path::Path, process};

use _struct::Struct;
use _type::Types;
use frame::Frame;
use function::{Extern, Function};
use module::Module;
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
mod _struct;
mod block;
mod module;

// TODO: better error handling
// TODO: result type
// TODO: actual type checking
// TODO: pointers to stack
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage();
        println!("no file or subcommand provided");
        process::exit(1);
    }

    let mut linker_paths: Vec<String> = Vec::new();

    let mut timing = false;
    let mut debug = false;

    let mut i = 1;

    let mut program = String::new();
    while i < args.len() {
        match args[i].as_str() {
            "--time"  | "-t" => timing = true,
            "--link"  | "-l" => {
                if args.len() <= i + 1 {
                    println!("linker path expected");
                    process::exit(1);
                }

                i += 1;
                linker_paths.push(args[i].clone());
            }
            "--debug" | "-d" => {
                debug = true;
            }
            "help" => {
                usage();
                process::exit(0);
            }
            "run" | "r" => {
                if args.len() <= i + 1 {
                    println!(".rbb file expected");
                    process::exit(1);
                }

                i += 1;
                program = args[i].clone();
            }
            _ => {
                program = args[i].clone();
            }
        }
        i += 1;
    }

    if program.is_empty() {
        usage();
        println!("no program provided");
        process::exit(1);
    }

    if !program.ends_with(".rbb") {
        println!(".rbb file expected");
        process::exit(1);
    }

    if !Path::new(&program).exists() {
        println!("program provided does not exist");
        process::exit(1);
    }

    let program = fs::read(program).expect("failed to read program");

    let start = std::time::Instant::now();
    let retval = run_program(&program, linker_paths, debug);
    if timing {
        println!();
        println!("program execution took {:.6}s ({:.4}ms)", start.elapsed().as_secs_f32(), start.elapsed().as_secs_f32() * 1000f32);
    }

    if retval != 0 {
        std::process::exit(retval);
    }
}

pub fn run_program(program: &Vec<u8>, linker_paths: Vec<String>, debug: bool) -> i32 {
    let mut consts: HashMap<String, i32> = HashMap::new();

    init_consts(&mut consts);

    let mut stack: Vec<Frame> = Vec::new();

    let mut global_scope = Scope::new();
    
    parse_program(program, &mut stack, &mut global_scope, &linker_paths, debug, &consts);

    let global_frame = stack.len() - 1;
    
    let retval = exec_scope(&global_scope, &global_scope, &mut stack, global_frame, false, &mut 0, global_frame, global_frame);

    if retval != 0 {
        return retval;
    }
    
    // if let Some(func) = global_scope.functions.get("main") { // main functions are not required
    //     return exec_func(func, &global_scope, &mut stack, global_frame, global_frame);
    // }

    return 0;

    // dbg!(stack);
}

fn init_consts(consts: &mut HashMap<String, i32>) {
    consts.insert("PLATFORM_LINUX".to_string(), 0);
    consts.insert("PLATFORM_WIN32".to_string(), 1);
    consts.insert("PLATFORM_OTHER".to_string(), 2);

    match env::consts::OS {
        "linux" => consts.insert("PLATFORM".to_string(), *consts.get("PLATFORM_LINUX").unwrap()),
        "windows" => consts.insert("PLATFORM".to_string(), *consts.get("PLATFORM_WIN32").unwrap()),
        _ => consts.insert("PLATFORM".to_string(), *consts.get("PLATFORM_OTHER").unwrap()),
    };
}

fn usage() {
    println!("Usage: rainbow [cmd] [flags]\n");
    println!("Flags");
    println!("  --time/-t                       enables execution timing");
    println!("  --link/-l  [path]               provide a linking path");
    println!("  --debug/-d                      enables debug mode");
    println!("Subcommands");
    println!("  help                            prints this subcommand list");
    println!("  run/r      [file]               runs the given program");
    println!("  [file]                          runs the given program");
}

fn parse_program(program: &Vec<u8>, stack: &mut Vec<Frame>, scope: &mut Scope, linker_paths: &Vec<String>, debug: bool, consts: &HashMap<String, i32>) {
    let mut index = 0;

    *scope = match parse_scope(&program, stack, &mut index, linker_paths, debug, consts) {
        Ok(scope) => scope,
        Err(error) => panic!("failed to parse program:\n{error}")
    };

    let global_frame = stack.len();
    stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });
    
    match parse_data_section(&program, stack, &mut index, global_frame) {
        Ok(_) => (),
        Err(error) => panic!("failed to parse data:\n{error}")
    }

    if debug {
        println!("global scope: ");
        println!("{scope}");
    }
}

fn parse_data_section(bytes: &Vec<u8>, stack: &mut Vec<Frame>, index: &mut usize, global_frame: usize) -> Result<(), String> {
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

        let i = stack[global_frame].stack.len();
        stack[global_frame].push_var(&name, typ.clone(), Values::POINTER(i + 1, len));

        // TODO: macroize
        match typ.typ[0] {
            Types::POINTER => {
                for _ in 0..len {
                    match typ.typ[1] {
                        Types::I8 => {
                            let val = i8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap());
                            *index += 1;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I16 => {
                            let val = i16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap());
                            *index += 2;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I32 => {
                            let val = i32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::I64 => {
                            let val = i64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::SIGNED(val as i64) });
                        }
                        Types::U8 => {
                            let val = u8::from_be_bytes(bytes[*index..*index+1].try_into().unwrap());
                            *index += 1;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U16 => {
                            let val = u16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap());
                            *index += 2;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U32 => {
                            let val = u32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::U64 => {
                            let val = u64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::UNSIGNED(val as u64) });
                        }
                        Types::F16 => {
                            let val = f16::to_f64(f16::from_be_bytes(bytes[*index..*index+2].try_into().unwrap()));
                            *index += 2;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        Types::F32 => {
                            let val = f32::from_be_bytes(bytes[*index..*index+4].try_into().unwrap());
                            *index += 4;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        Types::F64 => {
                            let val = f64::from_be_bytes(bytes[*index..*index+8].try_into().unwrap());
                            *index += 8;

                            stack[global_frame].push(Value { typ: typ.clone().pop(), val: Values::DECIMAL(val as f64) });
                        }
                        _ => panic!("unsupported data section type `{:?}`", typ.typ),
                    }
                }
            }
            _ => panic!("unsupported data section type `{:?}`", typ.typ),
        }
    }

    Ok(())
}

// this function expects the function to exist
// if it doesnt, it will crash
fn get_func<'a>(name: &String, scope: &'a Scope, global_scope: &'a Scope, module_frame: usize, global_frame: usize) -> (usize, Function) {
    if scope.func_exists(name, false) {
        return (module_frame, scope.get_func(name));
    } else if global_scope.func_exists(name, false) {
        return (global_frame, global_scope.get_func(name));
    } else {
        if name.contains(".") {
            let split = name.split(".").collect::<Vec<&str>>();
            
            let module_name = &split[0].to_string();
            let module = get_module(module_name, scope, global_scope);

            let name = split[1..].to_vec().join(".");
            let scope = &module.scope;

            return get_func(&name, scope, global_scope, module.frame, global_frame);
        } else {
            panic!("tried to call undefined function `{}`", name);
        }
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
        if name.contains(".") {
            let split = name.split(".").collect::<Vec<&str>>();
            
            let module_name = &split[0].to_string();
            let module = get_module(module_name, scope, global_scope);

            let name = split[1..].to_vec().join(".");
            let scope = &module.scope;

            return get_extern(&name, scope, global_scope);
        } else {
            panic!("tried to call undefined function `{}`", name);
        }
    }
}

fn get_module<'a>(name: &String, scope: &'a Scope, global_scope: &'a Scope) -> &'a Module {
    if scope.modules.contains_key(name) {
        return scope.modules.get(name).unwrap();
    } else if global_scope.modules.contains_key(name) {
        return global_scope.modules.get(name).unwrap();
    } else {
        panic!("tried to get undefined module `{}`", name);
    }
}

fn func_exists(name: &String, scope: &Scope, global_scope: &Scope) -> bool {
    return scope.func_exists(name, true) || global_scope.func_exists(name, true);
}

// these functions expect the variable to exist
// if it doesnt, it will crash (it was going to crash later anyways)
fn get_var<'a>(name: &String, global_scope: &'a Scope, stack: &'a mut [Frame], cur_frame: usize, module_frame: usize, global_frame: usize) -> &'a Value {
    if stack[cur_frame].vars.contains_key(name) {
        return stack[cur_frame].get_var(name);
    } else {
        if name.contains(".") {
            let split = name.split(".").collect::<Vec<&str>>();
            
            let struct_name = &split[0].to_string();
            let parent_struct = get_var(struct_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

            return get_struct_var(&parent_struct, &split[1].to_string(), global_scope, stack, cur_frame);
        }

        if stack[module_frame].vars.contains_key(name) {
            return stack[module_frame].get_var(name);
        } else {
            return stack[global_frame].get_var(name);
        }
    }
}

fn set_var(name: &String, value: &Values, global_scope: &Scope, stack: &mut [Frame], cur_frame: usize, module_frame: usize, global_frame: usize) {
    if name == "_" {
        return;
    }

    if stack[cur_frame].vars.contains_key(name) {
        stack[cur_frame].set_var(name, value);
    } else {
        if stack[global_frame].vars.contains_key(name) {
            stack[global_frame].set_var(name, value);
        } else {
            if name.contains(".") {
                let split = name.split(".").collect::<Vec<&str>>();
                
                let struct_name = &split[0].to_string();
                let parent_struct = get_var(struct_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                set_struct_var(&parent_struct, &split[1].to_string(), value, global_scope, stack, cur_frame);
                return;
            }

            if stack[module_frame].vars.contains_key(name) {
                stack[module_frame].set_var(name, value);
            } else {
                panic!("tried to set undefined variable `{name}`");
            }
        }
    }
}

// TODO: structs that arent in the global scope
fn get_struct<'a>(name: &String, scope: &'a Scope) -> &'a Struct {
    if scope.structs.contains_key(name) {
        return scope.structs.get(name).unwrap();
    } else {
        panic!("tried to get value from struct that somehow doesn't exist");
    }
}

fn set_struct_var(parent_struct: &Value, name: &String, value: &Values, global_scope: &Scope, stack: &mut [Frame], cur_frame: usize) {
    let struct_val = match &parent_struct.val {
        Values::STRUCT(name, index) => (name, index),
        _ => panic!("cannot set a variable in a value that is not a struct"),
    };

    let _struct = get_struct(&struct_val.0, global_scope);

    let var_offset = _struct.var_offsets.get(name).
                            expect(format!("attempted to set non-existant variable `{name}` in struct `{}`", _struct.name).as_str());

    // TODO: but what if the struct does *not* exist on the current frame?
    stack[cur_frame].set(struct_val.1+var_offset, value);
}

fn get_struct_var<'a>(parent_struct: &Value, name: &String, global_scope: &'a Scope, stack: &'a mut [Frame], cur_frame: usize) -> &'a Value {    
    let struct_val = match &parent_struct.val {
        Values::STRUCT(name, index) => (name, index),
        _ => panic!("cannot set a variable in a value that is not a struct"),
    };

    let _struct = get_struct(&struct_val.0, global_scope);

    let var_offset = _struct.var_offsets.get(name).
                            expect(format!("attempted to get non-existant variable `{name}` in struct `{}`", _struct.name).as_str());

    // TODO: but what if the struct does *not* exist on the current frame?
    return stack[cur_frame].get(struct_val.1+var_offset);
}