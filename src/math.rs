use crate::{get_var, stack::Frame, types::{add, div, mul, parse_imm, parse_string, sub, Types}};

// i tried to make macros for these but i cant figure it out
pub fn add_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = add(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = add(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let var1 = parse_imm(program, pc);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = add(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = add(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = sub(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = sub(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let var1 = parse_imm(program, pc);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = sub(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = sub(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = mul(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = mul(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let var1 = parse_imm(program, pc);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = mul(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = mul(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = div(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let var1 = parse_imm(program, pc);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let frame = &mut stack[current_frame];

    let name1 = parse_string(program, pc);
    let var1 = get_var(stack, current_frame, name1);
    let name2 = parse_string(program, pc);
    let var2 = get_var(stack, current_frame, name2);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}