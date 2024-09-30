use std::collections::HashMap;

use crate::{_struct::Struct, function::{Extern, Function}, instruction::Instruction};

#[derive(Debug, Clone)]
pub struct Scope {
    pub instructions: Vec<Instruction>,
    pub scopes: Vec<Scope>,
    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>,
    pub structs: HashMap<String, Struct>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope { instructions: Vec::new(), scopes: Vec::new(), functions: HashMap::new(), externs: HashMap::new(), structs: HashMap::new() }
    }

    pub fn merge(&mut self, mut other: Scope) {
        self.instructions.append(&mut other.instructions);
        self.scopes.append(&mut other.scopes);
        self.functions.extend(other.functions);
        self.externs.extend(other.externs);
        self.structs.extend(other.structs);
    }
}