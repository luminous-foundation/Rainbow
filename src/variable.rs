use crate::{_type::Type, number::Number};

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub typ: Type,
    pub value: Number,
}