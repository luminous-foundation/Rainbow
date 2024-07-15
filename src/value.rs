use crate::{_type::Type, number::Number};

#[derive(Debug)]
pub struct Value {
    pub typ: Type,
    pub value: Number,
}