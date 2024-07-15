use crate::{_type::Type, value::Value};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub typ: Type,
    pub value: Value,
}