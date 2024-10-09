use std::collections::HashMap;

use crate::{_struct::Struct, block::Block, function::{Extern, Function}, instruction::Instruction};

#[derive(Debug, Clone)]
pub struct Scope {
    pub blocks: Vec<Block>,
    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>,
    pub structs: HashMap<String, Struct>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope { blocks: Vec::new(), functions: HashMap::new(), externs: HashMap::new(), structs: HashMap::new() }
    }

    pub fn merge(&mut self, mut other: Scope) {
        self.blocks.append(&mut other.blocks);
        self.functions.extend(other.functions);
        self.externs.extend(other.externs);
        self.structs.extend(other.structs);
    }
}