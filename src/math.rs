use crate::{parse_var, stack::Frame, types::{add, and, div, lsh, mul, not, or, parse_imm, parse_string, rsh, sub, xor, Types}};

// i tried to make macros for these but i cant figure it out
pub fn add_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = add(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = add(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = add(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn add_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

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
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = sub(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = sub(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn sub_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

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
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = mul(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = mul(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn mul_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

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
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn div_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = div(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn and_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = and(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn and_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = and(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn and_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = and(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn and_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = and(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn or_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = or(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn or_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = or(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn or_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = or(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn or_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = or(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn xor_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = xor(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn xor_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = xor(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn xor_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = xor(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn xor_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = xor(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn not_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = not(&parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn not_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = not(&var1); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn lsh_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = lsh(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn lsh_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = lsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn lsh_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = lsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn lsh_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = lsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn rsh_imm_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let sum: Box<Types> = rsh(&parse_imm(program, pc), &parse_imm(program, pc)); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn rsh_var_imm(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_imm(program, pc);

    let sum: Box<Types> = rsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn rsh_imm_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_imm(program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = rsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}

pub fn rsh_var_var(stack: &mut Vec<Frame>, current_frame: usize, program: &Vec<u8>, pc: &mut usize) {
    let var1 = parse_var(stack, current_frame, program, pc);
    let var2 = parse_var(stack, current_frame, program, pc);

    let sum: Box<Types> = rsh(&var1, &var2); // TODO: type cast both into the type of the output var

    let name = parse_string(program, pc);

    stack[current_frame].set_var(name, sum);
}