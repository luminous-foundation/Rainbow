use crate::{_type::Type, value::Value};

#[derive(Debug)]
pub enum Argument {
    IMM(Value),
    NAME(String),
    TYPE(Type),
}