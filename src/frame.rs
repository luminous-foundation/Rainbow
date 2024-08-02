use std::collections::HashMap;

use crate::{_type::{Type, Types}, value::{Value, Values}};

#[derive(Debug, Clone)]
pub struct Frame {
    pub vars: HashMap<String, usize>,
    pub stack: Vec<Value>,
}

impl Frame {
    pub fn push(self: &mut Frame, val: Value) {
        self.stack.push(val);
    }
    
    // TODO: have this remove any variables that say they live at this location
    pub fn pop(&mut self) -> Value {
        return self.stack.pop().expect("attempted to pop empty stack");
    }

    pub fn get_var(self: &Frame, name: &String) -> &Value {
        let index = self.vars.get(name);
        if index.is_none() {
            panic!("tried to get undefined variable {}", name);
        }
        return &self.stack[*index.expect("unreachable")];
    }

    pub fn set_var(self: &mut Frame, name: &String, value: &Values) {
        let val = *self.vars.get(name).unwrap_or_else(|| panic!("attempted to set value of undefined variable {}", name));
        self.stack[val].set(value);
    }

    pub fn push_var(self: &mut Frame, name: String, typ: Type) {
        let value = match typ.typ[0] {
            Types::VOID => Value { typ: typ, val: Values::VOID },
            Types::I8 => Value { typ: typ, val: Values::SIGNED(0) },
            Types::I16 => Value { typ: typ, val: Values::SIGNED(0) },
            Types::I32 => Value { typ: typ, val: Values::SIGNED(0) },
            Types::I64 => Value { typ: typ, val: Values::SIGNED(0) },
            Types::U8 => Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U16 => Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U32 => Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U64 => Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::F16 => Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::F32 => Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::F64 => Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::POINTER => Value { typ: typ, val: Values::POINTER(0) },
            Types::TYPE => Value { typ: typ, val: Values::TYPE(Type { typ: todo!() }) },
            Types::STRUCT => Value { typ: typ, val: todo!() },
            Types::NAME => Value { typ: typ, val: Values::NAME("".to_string()) },
        };

        let index = self.stack.len();
        self.stack.push(value);
        self.vars.insert(name.clone(), index);
    }
}