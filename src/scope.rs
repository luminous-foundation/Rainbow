use crate::function::Function;
use crate::jump::{
    je_imm_imm_imm, je_imm_imm_var, je_imm_var_imm, je_imm_var_var, je_var_imm_imm, je_var_imm_var,
    je_var_var_imm, je_var_var_var, jg_imm_imm_imm, jg_imm_imm_var, jg_imm_var_imm, jg_imm_var_var,
    jg_var_imm_imm, jg_var_imm_var, jg_var_var_imm, jg_var_var_var, jge_imm_imm_imm,
    jge_imm_imm_var, jge_imm_var_imm, jge_imm_var_var, jge_var_imm_imm, jge_var_imm_var,
    jge_var_var_imm, jge_var_var_var, jl_imm_imm_imm, jl_imm_imm_var, jl_imm_var_imm,
    jl_imm_var_var, jl_var_imm_imm, jl_var_imm_var, jl_var_var_imm, jl_var_var_var,
    jle_imm_imm_imm, jle_imm_imm_var, jle_imm_var_imm, jle_imm_var_var, jle_var_imm_imm,
    jle_var_imm_var, jle_var_var_imm, jle_var_var_var, jmp_imm, jmp_var, jne_imm_imm_imm,
    jne_imm_imm_var, jne_imm_var_imm, jne_imm_var_var, jne_var_imm_imm, jne_var_imm_var,
    jne_var_var_imm, jne_var_var_var,
};

use crate::math::{
    add_imm_imm, add_imm_var, add_var_imm, add_var_var, and_imm_imm, and_imm_var, and_var_imm,
    and_var_var, div_imm_imm, div_imm_var, div_var_imm, div_var_var, lsh_imm_imm, lsh_imm_var,
    lsh_var_imm, lsh_var_var, mul_imm_imm, mul_imm_var, mul_var_imm, mul_var_var, not_imm, not_var,
    or_imm_imm, or_imm_var, or_var_imm, or_var_var, rsh_imm_imm, rsh_imm_var, rsh_var_imm,
    rsh_var_var, sub_imm_imm, sub_imm_var, sub_var_imm, sub_var_var, xor_imm_imm, xor_imm_var,
    xor_var_imm, xor_var_var,
};

use crate::parse_var;
use crate::stack::Frame;
use crate::types::{parse_imm, parse_string, Types};

pub struct Scope {
    pub code: Vec<u8>,
    pub children: Vec<Scope>,
    pub functions: Vec<Function>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {code: Vec::new(), children: Vec::new(), functions: Vec::new()}
    }

    pub fn parse(self: &mut Scope, bytes: &Vec<u8>) {
        let mut index: usize = 0;

        while index < bytes.len() { 
            match bytes[index] {
                0xFF => panic!("function parsing is not yet implemented"),
                0xFE => panic!("scope parsing is not yet implemented"),
                0xFD => panic!("scope parsing is not yet implemented"),
                _ => panic!("instruction parsing is not yet implemented"),
            }
        }
    }

    pub fn exec(self: &mut Scope) {
        todo!();
    }

    // run one instruction
    pub fn step(self: &mut Scope, stack: &mut Vec<Frame>, current_frame: usize, pc: &mut usize) {
        // im _pretty_ sure this should not be here, it should be at the start of the exec function (that doesnt exist yet)
        stack.push(Frame::new());

        // bounds check
        if *pc < self.code.len() {
            let byte = self.code[*pc]; // get the current instruction
            *pc += 1; // increment program counter

            // run the instruction
            match byte {
                0x00 => { /* do nothing */ }

                0x01 => Self::push_imm(stack, current_frame, &self.code, pc),
                0x02 => Self::push_var(stack, current_frame, &self.code, pc),

                0x03 => Self::pop(stack, current_frame, &self.code, pc),

                0x04 => Self::ldarg_imm(stack, &self.code, pc),
                0x05 => Self::ldarg_var(stack, &self.code, pc),

                0x08 => add_imm_imm(stack, current_frame, &self.code, pc),
                0x09 => add_var_imm(stack, current_frame, &self.code, pc),
                0x0A => add_imm_var(stack, current_frame, &self.code, pc),
                0x0B => add_var_var(stack, current_frame, &self.code, pc),

                0x0C => sub_imm_imm(stack, current_frame, &self.code, pc),
                0x0D => sub_var_imm(stack, current_frame, &self.code, pc),
                0x0E => sub_imm_var(stack, current_frame, &self.code, pc),
                0x0F => sub_var_var(stack, current_frame, &self.code, pc),

                0x10 => mul_imm_imm(stack, current_frame, &self.code, pc),
                0x11 => mul_var_imm(stack, current_frame, &self.code, pc),
                0x12 => mul_imm_var(stack, current_frame, &self.code, pc),
                0x13 => mul_var_var(stack, current_frame, &self.code, pc),

                0x14 => div_imm_imm(stack, current_frame, &self.code, pc),
                0x15 => div_var_imm(stack, current_frame, &self.code, pc),
                0x16 => div_imm_var(stack, current_frame, &self.code, pc),
                0x17 => div_var_var(stack, current_frame, &self.code, pc),

                0x18 => jmp_imm(&self.code, pc),
                0x19 => jmp_var(stack, current_frame, &self.code, pc),

                // function names about to go wild
                0x1A => jne_imm_imm_imm(&self.code, pc),
                0x1B => jne_var_imm_imm(stack, current_frame, &self.code, pc),
                0x1C => jne_imm_var_imm(stack, current_frame, &self.code, pc),
                0x1D => jne_var_var_imm(stack, current_frame, &self.code, pc),
                0x1E => jne_imm_imm_var(stack, current_frame, &self.code, pc),
                0x1F => jne_var_imm_var(stack, current_frame, &self.code, pc),
                0x20 => jne_imm_var_var(stack, current_frame, &self.code, pc),
                0x21 => jne_var_var_var(stack, current_frame, &self.code, pc),

                0x22 => je_imm_imm_imm(&self.code, pc),
                0x23 => je_var_imm_imm(stack, current_frame, &self.code, pc),
                0x24 => je_imm_var_imm(stack, current_frame, &self.code, pc),
                0x25 => je_var_var_imm(stack, current_frame, &self.code, pc),
                0x26 => je_imm_imm_var(stack, current_frame, &self.code, pc),
                0x27 => je_var_imm_var(stack, current_frame, &self.code, pc),
                0x28 => je_imm_var_var(stack, current_frame, &self.code, pc),
                0x29 => je_var_var_var(stack, current_frame, &self.code, pc),

                0x2A => jge_imm_imm_imm(&self.code, pc),
                0x2B => jge_var_imm_imm(stack, current_frame, &self.code, pc),
                0x2C => jge_imm_var_imm(stack, current_frame, &self.code, pc),
                0x2D => jge_var_var_imm(stack, current_frame, &self.code, pc),
                0x2E => jge_imm_imm_var(stack, current_frame, &self.code, pc),
                0x2F => jge_var_imm_var(stack, current_frame, &self.code, pc),
                0x30 => jge_imm_var_var(stack, current_frame, &self.code, pc),
                0x31 => jge_var_var_var(stack, current_frame, &self.code, pc),

                0x32 => jg_imm_imm_imm(&self.code, pc),
                0x33 => jg_var_imm_imm(stack, current_frame, &self.code, pc),
                0x34 => jg_imm_var_imm(stack, current_frame, &self.code, pc),
                0x35 => jg_var_var_imm(stack, current_frame, &self.code, pc),
                0x36 => jg_imm_imm_var(stack, current_frame, &self.code, pc),
                0x37 => jg_var_imm_var(stack, current_frame, &self.code, pc),
                0x38 => jg_imm_var_var(stack, current_frame, &self.code, pc),
                0x39 => jg_var_var_var(stack, current_frame, &self.code, pc),

                0x3A => jle_imm_imm_imm(&self.code, pc),
                0x3B => jle_var_imm_imm(stack, current_frame, &self.code, pc),
                0x3C => jle_imm_var_imm(stack, current_frame, &self.code, pc),
                0x3D => jle_var_var_imm(stack, current_frame, &self.code, pc),
                0x3E => jle_imm_imm_var(stack, current_frame, &self.code, pc),
                0x3F => jle_var_imm_var(stack, current_frame, &self.code, pc),
                0x40 => jle_imm_var_var(stack, current_frame, &self.code, pc),
                0x41 => jle_var_var_var(stack, current_frame, &self.code, pc),

                0x42 => jl_imm_imm_imm(&self.code, pc),
                0x43 => jl_var_imm_imm(stack, current_frame, &self.code, pc),
                0x44 => jl_imm_var_imm(stack, current_frame, &self.code, pc),
                0x45 => jl_var_var_imm(stack, current_frame, &self.code, pc),
                0x46 => jl_imm_imm_var(stack, current_frame, &self.code, pc),
                0x47 => jl_var_imm_var(stack, current_frame, &self.code, pc),
                0x48 => jl_imm_var_var(stack, current_frame, &self.code, pc),
                0x49 => jl_var_var_var(stack, current_frame, &self.code, pc),

                0x4A => Self::mov_imm(stack, current_frame, &self.code, pc),
                0x4B => Self::mov_var(stack, current_frame, &self.code, pc),

                0x4C => and_imm_imm(stack, current_frame, &self.code, pc),
                0x4D => and_var_imm(stack, current_frame, &self.code, pc),
                0x4E => and_imm_var(stack, current_frame, &self.code, pc),
                0x4F => and_var_var(stack, current_frame, &self.code, pc),

                0x50 => or_imm_imm(stack, current_frame, &self.code, pc),
                0x51 => or_var_imm(stack, current_frame, &self.code, pc),
                0x52 => or_imm_var(stack, current_frame, &self.code, pc),
                0x53 => or_var_var(stack, current_frame, &self.code, pc),

                0x54 => xor_imm_imm(stack, current_frame, &self.code, pc),
                0x55 => xor_var_imm(stack, current_frame, &self.code, pc),
                0x56 => xor_imm_var(stack, current_frame, &self.code, pc),
                0x57 => xor_var_var(stack, current_frame, &self.code, pc),

                0x58 => not_imm(stack, current_frame, &self.code, pc),
                0x59 => not_var(stack, current_frame, &self.code, pc),

                0x5A => lsh_imm_imm(stack, current_frame, &self.code, pc),
                0x5B => lsh_var_imm(stack, current_frame, &self.code, pc),
                0x5C => lsh_imm_var(stack, current_frame, &self.code, pc),
                0x5D => lsh_var_var(stack, current_frame, &self.code, pc),

                0x5E => rsh_imm_imm(stack, current_frame, &self.code, pc),
                0x5F => rsh_var_imm(stack, current_frame, &self.code, pc),
                0x60 => rsh_imm_var(stack, current_frame, &self.code, pc),
                0x61 => rsh_var_var(stack, current_frame, &self.code, pc),

                0x62 => Self::create_var_imm(stack, current_frame, &self.code, pc),
                0x63 => Self::create_var_var(stack, current_frame, &self.code, pc),

                0x64 => Self::ret_imm(stack, current_frame, &self.code, pc),
                0x65 => Self::ret_var(stack, current_frame, &self.code, pc),

                _ => panic!("unknown instruction {}", format!("0x{:02x}", byte)),
            }
        }
    }

    // push an immediate value onto the stack
    fn push_imm(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        stack[current_frame].push(parse_imm(code, pc));
    }

    // push a variable value onto the stack
    fn push_var(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let var = parse_var(stack, current_frame, code, pc);

        stack[current_frame].push(var);
    }

    // pop a value off the stack
    // puts the value into a variable
    fn pop(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let frame = &mut stack[current_frame];

        let name = parse_string(code, pc);
        let value = (*frame).pop();

        (*frame).set_var(name, value);
    }

    // load an argument to be passed to a function with an immediate value
    fn ldarg_imm(stack: &mut Vec<Frame>, code: &Vec<u8>, pc: &mut usize) {
        let frame = &mut stack[0]; // global frame

        let value = parse_imm(code, pc);

        frame.push(value);
    }

    // load an argument to be passed to a function with an variable value
    fn ldarg_var(stack: &mut Vec<Frame>, code: &Vec<u8>, pc: &mut usize) {
        let frame = &mut stack[0]; // global frame

        let value = parse_imm(code, pc);

        frame.push(value);
    }

    // move an immediate value into a variable
    fn mov_imm(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let frame = &mut stack[current_frame];

        let value = parse_imm(code, pc);

        let name = parse_string(code, pc);
        frame.set_var(name, value);
    }

    // move a variable value into a variable
    fn mov_var(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let var = parse_var(stack, current_frame, code, pc);

        let frame = &mut stack[current_frame];

        let name = parse_string(code, pc);
        frame.set_var(name, var);
    }

    // create a variable with an immediate value initializer
    fn create_var_imm(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let t = code[*pc];
        *pc += 1;

        let name = parse_string(code, pc);

        println!("attempting to create variable named {}", name);

        stack[current_frame].create_var(name, t);
    }

    // create a variable with a variable value initializer
    fn create_var_var(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        let t = *parse_var(stack, current_frame, code, pc);

        let name2 = parse_string(code, pc);

        println!("attempting to create variable named {}", name2);
        match t {
            Types::U8(value) => stack[current_frame].create_var(name2, value),
            _ => println!("cannot use non-type typed variable to create variable"), // one hell of an error message
        }
    }

    // return immediate value
    fn ret_imm(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        if current_frame == 0 {
            panic!("cannot return from the global scope");
        }

        let value = parse_imm(code, pc);

        let frame = &mut stack[current_frame - 1];

        match *value {
            Types::Void(_) => return,
            _ => frame.push(value),
        }
    }

    // return variable value
    fn ret_var(stack: &mut Vec<Frame>, current_frame: usize, code: &Vec<u8>, pc: &mut usize) {
        if current_frame == 0 {
            panic!("cannot return from the global scope");
        }

        let value = parse_var(stack, current_frame, code, pc);

        let frame = &mut stack[current_frame - 1];

        match *value {
            Types::Void(_) => return,
            _ => frame.push(value),
        }
    }
}