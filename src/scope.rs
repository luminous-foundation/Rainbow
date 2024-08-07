use std::collections::HashMap;

use crate::{function::{Extern, Function}, instruction::Instruction};

#[derive(Debug)]
pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>
}