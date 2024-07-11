use crate::argument::Argument;

#[derive(Debug)]
pub struct Instruction {
    pub index: usize,

    pub opcode: u8,
    pub args: Vec<Argument>,
}