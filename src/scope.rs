use std::{collections::HashMap, fmt::{self}};

use crate::{_struct::Struct, block::Block, function::{Extern, Function}};

#[derive(Debug, Clone)]
pub struct Scope {
    pub blocks: Vec<Block>,
    pub block_starts: Vec<usize>,

    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>,
    pub structs: HashMap<String, Struct>,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string(0))
    }
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
            let len = match &self.blocks[self.blocks.len()-1] {
                Block::CODE(vec) => vec.len(),
                Block::SCOPE(_) => 1,
            };
            
            self.block_starts.push(self.block_starts[self.block_starts.len()-1] + len);
        }

        self.blocks.push(block);
    }

    pub fn to_string(&self, depth: usize) -> String {        
        let mut indentation = String::new();
        for _ in 0..depth {
            indentation += "    ";
        }

        let mut str = String::new();

        str += &indentation;
        str += "{\n";

        for block in &self.blocks {
            match block {
                Block::CODE(vec) => {
                    for instr in vec {
                        str += "    ";
                        str += &indentation;

                        str += &instr.to_string();

                        str += "\n";
                    }
                },
                Block::SCOPE(scope) => {
                    str += "\n";
                    str += &scope.to_string(depth + 1);
                    str += "\n";
                }
            }
        }

        str += &indentation;
        str += "}\n";

        return str;
    }
}