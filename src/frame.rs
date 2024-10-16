use std::{collections::HashMap, usize};

use crate::{_type::{Type, Types}, value::{Value, Values}};

#[derive(Debug, Clone)]
pub struct Frame {
    pub vars: HashMap<String, usize>,
    pub stack: Vec<Value>,

    // TODO: the current allocs system will not work if multiple pointers point to one place,
    //       as the new pointer will try to say it owns the space
    //       i could just not track where pointers are pointing to, 
    //       although that does risk null pointers and use-after-free problems
    //       (however null pointers are still possible with this current system,
    //        if someone uses FREE with the right size
    //        the pointer will point to nothing but still exist)
    pub allocs: Vec<String>,
}

impl Frame {
    pub fn len(&self) -> usize {
        return self.stack.len();
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
        self.allocs.push(String::new());
    }
    
    // TODO: have this remove any variables that say they live at this location
    pub fn pop(&mut self) -> Value {
        let alloc = self.allocs.pop().expect("attempted to pop empty stack");
        if alloc.len() > 0 {
            if self.vars.contains_key(&alloc) {
                self.vars.remove(&alloc);
            } else {
                println!("https://github.com/luminous-foundation/Rainbow");
                panic!("something has gone terribly wrong here, create an issue if you see this");
            }
        }

        return self.stack.pop().expect("attempted to pop empty stack");
    }

    pub fn pop_args(&mut self, amnt: usize) -> Vec<Value> {
        if amnt > self.stack.len() {
            panic!("stack overflow while popping {} args off stack", amnt);
        }

        self.allocs.truncate(self.allocs.len() - amnt);
        return self.stack.split_off(self.stack.len() - amnt);
    }

    pub fn get_var(&self, name: &String) -> &Value {
        let index = self.vars.get(name);
        if index.is_none() {
            panic!("tried to get undefined variable `{}`", name);
        }
        return &self.stack[*index.unwrap()];
    }

    pub fn set_var(&mut self, name: &String, value: &Values) {
        let val = *self.vars.get(name).unwrap_or_else(|| panic!("attempted to set value of undefined variable `{}`", name));
        self.stack[val].set(value);
    }

    pub fn set(&mut self, index: usize, value: &Values) {
        self.stack[index].set(value);
    }

    pub fn get(&self, index: usize) -> &Value {
        return &self.stack[index];
    }

    pub fn push_alloc(self: &mut Frame, typ: &Type, alloc: String) {
        let value = Self::get_default_val(typ);

        self.stack.push(Value { typ: typ.clone(), val: value });
        self.allocs.push(alloc);
    }

    pub fn push_type(self: &mut Frame, typ: &Type) {
        let value = Self::get_default_val(typ);
        
        self.stack.push(Value { typ: typ.clone(), val: value });
        self.allocs.push(String::new());
    }

    pub fn push_var(&mut self, name: &String, typ: Type, value: Values) {
        if self.vars.contains_key(name) {
            // TODO: handle this if the type changes
            // todo!();
        } else {
            let index = self.stack.len();
            self.stack.push(Value { typ: typ, val: value });
            self.vars.insert(name.clone(), index);
            self.allocs.push(name.clone());
        }
    }

    pub fn get_default_val(typ: &Type) -> Values {
        match typ.typ[0] {
            Types::VOID => Values::VOID,
            Types::I8 => Values::SIGNED(0),
            Types::I16 => Values::SIGNED(0),
            Types::I32 => Values::SIGNED(0),
            Types::I64 => Values::SIGNED(0),
            Types::U8 => Values::UNSIGNED(0),
            Types::U16 => Values::UNSIGNED(0),
            Types::U32 => Values::UNSIGNED(0),
            Types::U64 => Values::UNSIGNED(0),
            Types::F16 => Values::DECIMAL(0f64),
            Types::F32 => Values::DECIMAL(0f64),
            Types::F64 => Values::DECIMAL(0f64),
            Types::POINTER => Values::POINTER(usize::MAX, 0),
            Types::TYPE => Values::TYPE(Type { typ: vec![Types::VOID] }),
            Types::STRUCT => Values::STRUCT(String::new(), "null".to_string(), usize::MAX),
            Types::NAME => Values::NAME("".to_string()),
        }
    }

    pub fn create_var(&mut self, name: String, typ: Type) {
        if self.vars.contains_key(&name) {
            // TODO: handle this if the type changes
            // todo!();
        } else {
            let val = Self::get_default_val(&typ);
            let value = Value { typ: typ, val: val };
    
            let index = self.stack.len();
            self.stack.push(value);
            self.vars.insert(name.clone(), index);
            self.allocs.push(name.clone());
        }
    }
}