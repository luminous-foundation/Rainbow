use std::{collections::HashMap};

use crate::{_type::{Type, Types}, value::{Value, Values}, variable::Variable};

#[derive(Debug, Clone)]
pub struct Frame {
    pub vars: HashMap<String, Variable>,
    pub stack: Vec<Value>,
}

impl Frame {
    pub fn push(self: &mut Frame, val: Value) {
        self.stack.push(val);
    }
    
    pub fn pop(&mut self) -> Value {
        return self.stack.pop().expect("attempted to pop empty stack");
    }

    pub fn get_var(self: &Frame, name: &String) -> Option<&Variable> {
        return self.vars.get(name);
    }

    pub fn set_var(self: &mut Frame, name: &String, value: Value) {
        let mut temp = self.vars.get(name).expect(&format!("attempted to set value of undefined variable {}", name)).clone();
        temp.value = value;
        self.vars.insert(name.clone(), temp);
    }

    pub fn push_var(self: &mut Frame, name: String, typ: Type) {
        let value;
        match typ.typ[0] {
            Types::VOID => value = Value { main_type: Types::VOID, val: Values::VOID },
            Types::I8 => value = Value { main_type: Types::I8, val: Values::SIGNED(0) },
            Types::I16 => value = Value { main_type: Types::I16, val: Values::SIGNED(0) },
            Types::I32 => value = Value { main_type: Types::I32, val: Values::SIGNED(0) },
            Types::I64 => value = Value { main_type: Types::I64, val: Values::SIGNED(0) },
            Types::U8 => value = Value { main_type: Types::U8, val: Values::UNSIGNED(0) },
            Types::U16 => value = Value { main_type: Types::U16, val: Values::UNSIGNED(0) },
            Types::U32 => value = Value { main_type: Types::U32, val: Values::UNSIGNED(0) },
            Types::U64 => value = Value { main_type: Types::U64, val: Values::UNSIGNED(0) },
            Types::F16 => value = Value { main_type: Types::F16, val: Values::DECIMAL(0f64) },
            Types::F32 => value = Value { main_type: Types::F32, val: Values::DECIMAL(0f64) },
            Types::F64 => value = Value { main_type: Types::F64, val: Values::DECIMAL(0f64) },
            Types::POINTER => value = Value { main_type: Types::POINTER, val: Values::POINTER(0) },
            Types::TYPE => value = Value { main_type: Types::TYPE, val: Values::TYPE(Type { typ: todo!() }) },
            Types::STRUCT => value = Value { main_type: Types::STRUCT, val: todo!() },
            Types::NAME => value = Value { main_type: Types::NAME, val: Values::NAME("".to_string()) },
        }

        self.vars.insert(name.clone(), Variable { name, typ, value: value });
    }
}