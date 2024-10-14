use std::{collections::HashMap, fmt::{self}};

use crate::{_struct::Struct, block::Block, function::{Extern, Function}, module::Module};

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent_scope: Option<Box<Scope>>,

    pub blocks: Vec<Block>,
    pub block_starts: Vec<usize>,

    pub functions: HashMap<String, Function>,
    pub externs: HashMap<String, Extern>,
    pub structs: HashMap<String, Struct>,

    pub modules: HashMap<String, Module>,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string(0, 0))
    }
}

impl Scope {
    pub fn new() -> Scope {
        Scope { parent_scope: None, blocks: Vec::new(), block_starts: Vec::new(), functions: HashMap::new(), externs: HashMap::new(), structs: HashMap::new(), modules: HashMap::new() }
    }

    pub fn func_exists(&self, name: &String, check_module: bool) -> bool {
        if check_module {
            if name.contains(".") {
                let split = name.split(".").collect::<Vec<&str>>();
                
                let module_name = &split[0].to_string();

                if self.modules.contains_key(module_name) {
                    let module = self.get_module(module_name);

                    let name = split[1..].to_vec().join(".");
                    let scope = &module.scope;

                    return scope.func_exists(&name, true);
                }
            }
        }

        if self.functions.contains_key(name) {
            return true;
        } else if self.parent_scope.is_some() {
            return (*self.parent_scope.clone().unwrap()).func_exists(name, check_module);
        }

        return false;
    }

    pub fn get_func<'a>(&'a self, name: &String) -> Function {
        if self.func_exists(name, false) {
            if self.functions.contains_key(name) {
                return self.functions.get(name).unwrap().clone();
            } else {
                if self.parent_scope.is_some() {
                    return (*self.parent_scope.clone().unwrap()).get_func(name);
                } else {
                    unreachable!();
                }
            }
        } else {
            panic!("tried to get undefined function `{name}`");
        }
    }

    pub fn get_module<'a>(&'a self, name: &String) -> &'a Module{
        if self.modules.contains_key(name) {
            return self.modules.get(name).unwrap();
        } else {
            panic!("tried to get undefined module `{}`", name);
        }
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
        self.modules.extend(other.modules);
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

    pub fn to_string(&self, depth: usize, index: usize) -> String {
        let mut indentation = String::new();
        for _ in 0..depth {
            indentation += "    ";
        }

        let mut str = String::new();

        for (name, module) in &self.modules {
            str += &indentation;
            str += "module ";
            str += name;
            str += "\n";
            str += &module.scope.to_string(depth + 1, 0);
        }

        str += &indentation;
        str += "{";
        str += " - #";
        str += &index.to_string();
        str += "\n";

        let mut index = 0;
        let mut i = 0;
        while i < self.blocks.len() {
            let block = &self.blocks[i];

            match block {
                Block::CODE(vec) => {
                    // str += "    ";
                    // str += &indentation;
                    // str += "start: ";
                    // str += &self.block_starts[i].to_string();
                    // str += "\n";

                    for instr in vec {
                        str += "    ";
                        str += &indentation;

                        str += &instr.to_string();

                        str += " - #";
                        str += &index.to_string();

                        str += "\n";

                        index += 1;
                    }
                },
                Block::SCOPE(scope) => {
                    if i > 0 {
                        str += "\n";
                    }

                    str += &scope.to_string(depth + 1, index);

                    if i < self.blocks.len() - 1{
                        str += "\n";
                    }

                    index += 1;
                }
            }

            i += 1;
        }

        str += &indentation;
        str += "}\n\n";

        for (_, func) in &self.functions {
            str += &indentation;

            str += &func.ret_type.to_string();
            str += " ";
            str += &func.name;
            str += "(";

            let mut i = 0;
            while i < func.arg_names.len() {
                str += &func.arg_types[i].to_string();
                str += " ";
                str += &func.arg_names[i];

                i += 1;
                
                if i < func.arg_names.len() {
                    str += ", ";
                }
            }

            str += ")\n";
            str += &func.scope.to_string(depth + 1, 0);
        }

        return str;
    }
}