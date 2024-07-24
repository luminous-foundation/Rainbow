use std::{collections::HashMap};

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
        let val = self.vars.get(name).expect(&format!("attempted to set value of undefined variable {}", name)).clone();
        self.stack[val].set(value);
    }

    pub fn push_var(self: &mut Frame, name: String, typ: Type) {
        let value;
        match typ.typ[0] {
            Types::VOID => value = Value { typ: typ, val: Values::VOID },
            Types::I8 => value = Value { typ: typ, val: Values::SIGNED(0) },
            Types::I16 => value = Value { typ: typ, val: Values::SIGNED(0) },
            Types::I32 => value = Value { typ: typ, val: Values::SIGNED(0) },
            Types::I64 => value = Value { typ: typ, val: Values::SIGNED(0) },
            Types::U8 => value = Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U16 => value = Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U32 => value = Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::U64 => value = Value { typ: typ, val: Values::UNSIGNED(0) },
            Types::F16 => value = Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::F32 => value = Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::F64 => value = Value { typ: typ, val: Values::DECIMAL(0f64) },
            Types::POINTER => value = Value { typ: typ, val: Values::POINTER(0) },
            Types::TYPE => value = Value { typ: typ, val: Values::TYPE(Type { typ: todo!() }) },
            Types::STRUCT => value = Value { typ: typ, val: todo!() },
            Types::NAME => value = Value { typ: typ, val: Values::NAME("".to_string()) },
        }

        let index = self.stack.len();
        self.stack.push(value);
        self.vars.insert(name.clone(), index);
    }
}