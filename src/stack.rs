use std::collections::HashMap;

use crate::types::{cast_type, from_type, Types};

pub struct Frame {
    pub stack: Vec<Box<Types>>,
    pub var_locs: HashMap<String, usize>,
    pub var_types: HashMap<String, u8>
}

impl Frame {
    pub fn new() -> Frame {
        Frame { stack: Vec::new(), var_locs: HashMap::new(), var_types: HashMap::new() }
    }
    
    pub fn create_var(&mut self, name: String, t: u8) {
        self.var_locs.insert(name.clone(), self.stack.len());
        self.var_types.insert(name.clone(), t);
        self.stack.push(from_type(t));
    }

    pub fn get_var(self, name: String) -> Box<Types> {
        return self.stack[*self.var_locs.get(&name).unwrap()].clone();
    }

    pub fn set_var(&mut self, name: String, value: Box<Types>) {
        let casted = cast_type(value, *self.var_types.get(&name).unwrap());

        self.stack[*self.var_locs.get(&name).unwrap()] = casted;
    }
}