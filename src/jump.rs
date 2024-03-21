use crate::{parse_var, stack::Frame, types::{parse_imm, Types}};

pub fn jmp(value: Types, pc: &mut usize) {
    match value {
        Types::I8(v) => *pc = v as usize,
        Types::I16(v) => *pc = v as usize,
        Types::I32(v) => *pc = v as usize,
        Types::I64(v) => *pc = v as usize,
        Types::U8(v) => *pc = v as usize,
        Types::U16(v) => *pc = v as usize,
        Types::U32(v) => *pc = v as usize,
        Types::U64(v) => *pc = v as usize,
        Types::F16(v) => *pc = v.to_f32() as usize,
        Types::F32(v) => *pc = v as usize,
        Types::F64(v) => *pc = v as usize,
        _ => panic!("invalid jump address type")
    }
}

pub fn jne(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 != val2 {
        jmp(addr, pc);
    }
}

pub fn je(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 == val2 {
        jmp(addr, pc);
    }
}

pub fn jge(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 >= val2 {
        jmp(addr, pc);
    }
}

pub fn jg(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 > val2 {
        jmp(addr, pc);
    }
}

pub fn jle(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 <= val2 {
        jmp(addr, pc);
    }
}

pub fn jl(val1: Types, val2: Types, addr: Types, pc: &mut usize) {
    if val1 < val2 {
        jmp(addr, pc);
    }
}

// if ONLY i could use macros (save me)
pub fn jmp_imm(program: &Vec<u8>, pc: &mut usize) {
    let value = *parse_imm(program, pc);

    jmp(value, pc);
}

pub fn jmp_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let value = *parse_var(stack, current_frame, program, pc);

    jmp(value, pc);
}

pub fn jne_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jne(val1, val2, addr, pc);
}

pub fn jne_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jne(val1, val2, addr, pc);
}

pub fn je_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    je(val1, val2, addr, pc);
}

pub fn je_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    je(val1, val2, addr, pc);
}

pub fn jge_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jge_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jge(val1, val2, addr, pc);
}

pub fn jg_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jg_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jg(val1, val2, addr, pc);
}

pub fn jle_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jle_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jle(val1, val2, addr, pc);
}

pub fn jl_imm_imm_imm(program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_var_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_imm(program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_imm_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_var_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_imm(program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_imm_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_var_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_imm(program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_imm_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_imm(program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jl(val1, val2, addr, pc);
}

pub fn jl_var_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let val1 = *parse_var(stack, current_frame, program, pc);
    let val2 = *parse_var(stack, current_frame, program, pc);

    let addr = *parse_var(stack, current_frame, program, pc);

    jl(val1, val2, addr, pc);
}