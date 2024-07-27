use std::collections::HashMap;

use half::f16;

use crate::{_type::{Type, Types}, frame::Frame, function::Function, get_var, instruction::Instruction, set_var, value::{Value, Values}};
use crate::instruction::Opcode;

#[derive(Debug)]
pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
}

// instruction macros
macro_rules! add {
    ($a:expr, $b:expr, $out:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.add(&$b.val);
        set_var($out, &val, $stack, $cur_frame);
    };
}

macro_rules! compare {
    ($a_val:expr, $b:expr, $op:tt, $pc:expr, $new_pc:expr) => {
        match $b {
            Values::SIGNED(b_val) => {
                if ($a_val as i64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            Values::UNSIGNED(b_val) => {
                if ($a_val as u64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            Values::DECIMAL(b_val) => {
                if ($a_val as f64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            _ => panic!("expected a number for comparison, got {:?}", $b),
        }
    }
}
macro_rules! get_pc {
    ($c:expr, $new_pc:expr) => {
        match $c {
            Values::UNSIGNED(c_val) => $new_pc = c_val as usize,
            Values::SIGNED(c_val) => {
                if c_val < 0 {
                    panic!("cannot jump to negative address");
                } else {
                    $new_pc = c_val as usize;
                }
            }
            _ => panic!("expected integer address value")
        }
    }
}

macro_rules! jne {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! je {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}

macro_rules! jge {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jg {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jle {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jl {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
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
    let mut pc = 0;

    let mut times: [f32; 256] = [0f32; 256];
    let mut counts: [u32; 256] = [0; 256];

    let scope_stack_start = stack[cur_frame].stack.len();

    let start = std::time::Instant::now();
    while pc < scope.instructions.len() {
        let instr = &scope.instructions[pc];

        let instr_start = std::time::Instant::now();
        match &instr.opcode {
            Opcode::NOP => { // NOP
                // do nothing
            }

            Opcode::PUSH_IMM(val) => { // PUSH [imm]
                stack[cur_frame].push(val.clone());
            }
            Opcode::PUSH_VAR(name) => { // PUSH [name]
                let var = get_var(name, stack, cur_frame);

                let val = var.clone(); // borrow checker :(
                stack[cur_frame].push(val);
            }

            Opcode::POP(name) => { // POP [name]
                set_var(name, &stack[cur_frame].pop().val, stack, cur_frame);
            }

            Opcode::ADD_I_I(a, b, out) => { // ADD [imm] [imm] [name]
                add!(a, b, out, stack, cur_frame);
            }
            Opcode::ADD_V_I(a_name, b, out) => { // ADD [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();

                add!(a, b, out, stack, cur_frame);
            }
            Opcode::ADD_I_V(a, b_name, out) => { // ADD [imm] [name] [name]                
                let b = get_var(b_name, stack, cur_frame).clone();

                add!(a, b, out, stack, cur_frame);
            }
            Opcode::ADD_V_V(a_name, b_name, out) => { // ADD [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                add!(a, b, out, stack, cur_frame);
            }

            Opcode::JMP_IMM(new_pc_val) => {
                let new_pc: usize;
                get_pc!(new_pc_val.val.clone(), new_pc);

                pc = new_pc - 1;
            }
            Opcode::JMP_VAR(new_pc_name) => {
                let new_pc_var = get_var(new_pc_name, stack, cur_frame).val.clone();
                let new_pc: usize;
                get_pc!(new_pc_var, new_pc);

                pc = new_pc - 1;
            }

            Opcode::JNE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }

            Opcode::JE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                je!(a, b, c, pc);
            }
            Opcode::JE_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }

            Opcode::JGE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }

            Opcode::JG_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jg!(a, b, c, pc);
            }
            Opcode::JG_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }

            Opcode::JLE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }

            Opcode::JL_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jl!(a, b, c, pc);
            }
            Opcode::JL_V_I_I(a_name, b, c) => { // JLE [name] [imm] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_V_I(a_name, b_name, c) => { // JLE [name] [name] [imm]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [name]
                let c = get_var(c_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_I_V(a_name, b, c_name) => { // JLE [name] [imm] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [name]
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_V_V(a_name, b_name, c_name) => { // JLE [name] [name] [name]
                let a = get_var(a_name, stack, cur_frame).clone();
                let b = get_var(b_name, stack, cur_frame).clone();
                let c = get_var(c_name, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }

            Opcode::VAR_TYPE_NAME(typ, name) => { // VAR [type] [name]
                stack[cur_frame].push_var(name.clone(), typ.clone());
            }
            Opcode::VAR_VAR_NAME(type_var, name) => { // VAR [var] [name]
                let type_var = get_var(type_var, stack, cur_frame);

                let typ;
                match &type_var.val {
                    Values::TYPE(t) => typ = t.clone(),
                    _ => panic!("tried to create variable with dynamic type stored in variable, but given variable had type {:?}", type_var.typ)
                }
                
                stack[cur_frame].push_var(name.clone(), typ);
            }
            Opcode::VAR_TYPE_VAR(typ, name_var) => { // VAR [type] [var]
                let name_var = get_var(name_var, stack, cur_frame);

                let name;
                match &name_var.val {
                    Values::NAME(n) => name = n.clone(),
                    _ => panic!("tried to create variable with dynamic name stored in variable, but given variable had type {:?}", name_var.typ)
                }

                stack[cur_frame].push_var(name, typ.clone())
            }
            Opcode::VAR_VAR_VAR(type_var, name_var) => { // VAR [var] [var]
                let type_var = get_var(type_var, stack, cur_frame);

                let typ;
                match &type_var.val {
                    Values::TYPE(t) => typ = t.clone(),
                    _ => panic!("tried to create variable with dynamic type stored in variable, but given variable had type {:?}", type_var.typ)
                }

                let name_var = get_var(name_var, stack, cur_frame);

                let name;
                match &name_var.val {
                    Values::NAME(n) => name = n.clone(),
                    _ => panic!("tried to create variable with dynamic name stored in variable, but given variable had type {:?}", name_var.typ)
                }

                stack[cur_frame].push_var(name, typ);
            }

            _ => panic!("unknown instruction {:#04x} at {:#06x}", instr.opcode.to_u8(), instr.index)
        }
        
        times[instr.opcode.to_u8() as usize] += instr_start.elapsed().as_secs_f32() * 1000f32;
        counts[instr.opcode.to_u8() as usize] += 1;
        
        pc += 1;
    }

    // TODO: i want to clear everything created by the scope
    //       but this on its own leaves dangling variables which will be null refs!
    //       also this probably shouldn't clear if it's working with the global space...
    // stack[cur_frame].stack = stack[cur_frame].stack[0..scope_stack_start].to_vec();
    
    println!("scope took {:.2}ms", start.elapsed().as_secs_f32() * 1000f32);

    for x in 0x00..0xff {
        if counts[x] > 0 {
            println!("{:#04x}: {:.4}ms avg | {:.4}ms total", x, times[x] / counts[x] as f32, times[x]);
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
    let opcode_byte = bytes[*index];

    let start_index = *index;

    *index += 1;

    let opcode: Opcode;

    match opcode_byte {
        0x00 => {
            opcode = Opcode::NOP
        }

        0x01 => {
            opcode = Opcode::PUSH_IMM(parse_immediate(bytes, index)?)
        }
        0x02 => {
            opcode = Opcode::PUSH_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x03 => {
            opcode = Opcode::POP(parse_bytecode_string(bytes, index)?)
        }

        0x04 => {
            opcode = Opcode::LDARG_IMM(parse_immediate(bytes, index)?)
        }
        0x05 => {
            opcode = Opcode::LDARG_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x08 => {
            opcode = Opcode::ADD_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x09 => {
            opcode = Opcode::ADD_V_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0A => {
            opcode = Opcode::ADD_I_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x0B => {
            opcode = Opcode::ADD_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }

        0x18 => {
            opcode = Opcode::JMP_IMM(parse_immediate(bytes, index)?)
        }
        0x19 => {
            opcode = Opcode::JMP_VAR(parse_bytecode_string(bytes, index)?)
        }

        0x1A => {
            opcode = Opcode::JNE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1B => {
            opcode = Opcode::JNE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1C => {
            opcode = Opcode::JNE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1D => {
            opcode = Opcode::JNE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x1E => {
            opcode = Opcode::JNE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x1F => {
            opcode = Opcode::JNE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x20 => {
            opcode = Opcode::JNE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x21 => {
            opcode = Opcode::JNE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x22 => {
            opcode = Opcode::JE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x23 => {
            opcode = Opcode::JE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x24 => {
            opcode = Opcode::JE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x25 => {
            opcode = Opcode::JE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x26 => {
            opcode = Opcode::JE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x27 => {
            opcode = Opcode::JE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x28 => {
            opcode = Opcode::JE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x29 => {
            opcode = Opcode::JE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x2A => {
            opcode = Opcode::JGE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2B => {
            opcode = Opcode::JGE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2C => {
            opcode = Opcode::JGE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2D => {
            opcode = Opcode::JGE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x2E => {
            opcode = Opcode::JGE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x2F => {
            opcode = Opcode::JGE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x30 => {
            opcode = Opcode::JGE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x31 => {
            opcode = Opcode::JGE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x32 => {
            opcode = Opcode::JG_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x33 => {
            opcode = Opcode::JG_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x34 => {
            opcode = Opcode::JG_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x35 => {
            opcode = Opcode::JG_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x36 => {
            opcode = Opcode::JG_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x37 => {
            opcode = Opcode::JG_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x38 => {
            opcode = Opcode::JG_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x39 => {
            opcode = Opcode::JG_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x3A => {
            opcode = Opcode::JLE_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3B => {
            opcode = Opcode::JLE_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3C => {
            opcode = Opcode::JLE_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3D => {
            opcode = Opcode::JLE_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x3E => {
            opcode = Opcode::JLE_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x3F => {
            opcode = Opcode::JLE_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x40 => {
            opcode = Opcode::JLE_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x41 => {
            opcode = Opcode::JLE_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x42 => {
            opcode = Opcode::JL_I_I_I(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x43 => {
            opcode = Opcode::JL_V_I_I(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x44 => {
            opcode = Opcode::JL_I_V_I(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x45 => {
            opcode = Opcode::JL_V_V_I(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?)
        }
        0x46 => {
            opcode = Opcode::JL_I_I_V(parse_immediate(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x47 => {
            opcode = Opcode::JL_V_I_V(parse_bytecode_string(bytes, index)?,
            parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x48 => {
            opcode = Opcode::JL_I_V_V(parse_immediate(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x49 => {
            opcode = Opcode::JL_V_V_V(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        
        0x63 => {
            opcode = Opcode::VAR_TYPE_NAME(parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x64 => {
            opcode = Opcode::VAR_VAR_NAME(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x65 => {
            opcode = Opcode::VAR_TYPE_VAR(parse_type(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        0x66 => {
            opcode = Opcode::VAR_VAR_VAR(parse_bytecode_string(bytes, index)?,
            parse_bytecode_string(bytes, index)?)
        }
        _ => return Err(format!("unknown instruction {:#04x} at {:#06x}", opcode_byte, index))
    }

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
    
    return Ok(Value { typ: Type { typ: vec![Types::from_u8(typ)] } , val: value });
}