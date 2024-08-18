use std::collections::HashMap;

use crate::{function::{Extern, Function}, instruction::Instruction};

#[derive(Debug, Clone)]
pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>
}

impl Scope {
    pub fn new() -> Scope {
        Scope { instructions: Vec::new(), scopes: Vec::new(), functions: HashMap::new(), externs: HashMap::new() }
    }

    pub fn merge(&mut self, mut other: Scope) {
        self.instructions.append(&mut other.instructions);
        self.scopes.append(&mut other.scopes);
        self.functions.extend(other.functions);
        self.externs.extend(other.externs);
    }
}