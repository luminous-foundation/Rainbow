use std::collections::HashMap;

use crate::{_struct::Struct, block::Block, function::{Extern, Function}, instruction::Instruction};

#[derive(Debug, Clone)]
pub struct Scope {
    pub blocks: Vec<Block>,
    pub block_starts: Vec<usize>,

    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>,
    pub structs: HashMap<String, Struct>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope { blocks: Vec::new(), block_starts: Vec::new(), functions: HashMap::new(), externs: HashMap::new(), structs: HashMap::new() }
    }

    pub fn merge(&mut self, mut other: Scope) {
        self.blocks.append(&mut other.blocks);

        self.block_starts = Vec::new();

        let mut len = 0;
        for block in &self.blocks {
            self.block_starts.push(len);
            
            match block {
                Block::CODE(vec) => len += vec.len(),
                Block::SCOPE(_) => len += 1,
            }
        }

        self.functions.extend(other.functions);
        self.externs.extend(other.externs);
        self.structs.extend(other.structs);
    }

    pub fn add_block(&mut self, block: Block) {
        if self.block_starts.len() == 0 {
            self.block_starts.push(0);
        } else {
            let len = match &block {
                Block::CODE(vec) => vec.len(),
                Block::SCOPE(_) => 1,
            };
            
            self.block_starts.push(self.block_starts[self.block_starts.len()-1] + len);
        }

        self.blocks.push(block);
    }
}