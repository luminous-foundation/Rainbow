use crate::{instruction::Instruction, scope::Scope};

#[derive(Debug, Clone)]
pub enum Block {
    CODE(Vec<Instruction>),
    SCOPE(Scope),
}