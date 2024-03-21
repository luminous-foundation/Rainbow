use std::collections::HashMap;

use crate::types::{cast_type, from_type, Types};

pub struct Frame {
    pub stack: Vec<Box<Types>>,
    pub locs: HashMap<usize, String>,
    pub var_locs: HashMap<String, usize>,
    pub var_types: HashMap<String, u8>
}

impl Frame {
    pub fn new() -> Frame {
        Frame { stack: Vec::new(), locs: HashMap::new(), var_locs: HashMap::new(), var_types: HashMap::new() }
    }
    
    pub fn create_var(&mut self, name: String, t: u8) {
        self.locs.insert(self.stack.len(), name.clone());
        self.var_locs.insert(name.clone(), self.stack.len());
        self.var_types.insert(name.clone(), t);
        self.stack.push(from_type(t));
    }

    pub fn get_var(&self, name: String) -> Box<Types> {
        let loc = *self.var_locs.get(&name).expect(format!("tried to get undefined variable `{}`", name).as_str());

        return self.stack[loc].clone();
    }

    pub fn set_var(&mut self, name: String, value: Box<Types>) {
        let casted = cast_type(value, *self.var_types.get(&name).unwrap());

        let loc = *self.var_locs.get(&name).expect(format!("tried to set undefined variable `{}`", name).as_str());

        self.stack[loc] = casted;
    }

    pub fn pop(&mut self) -> Box<Types> {
        let last = self.stack.len() - 1;

        if self.locs.contains_key(&last) {
            self.var_locs.remove(self.locs.get(&last).unwrap());
        }

        return self.stack.pop().unwrap();
    }
}