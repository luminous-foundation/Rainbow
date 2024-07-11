use crate::{_type::Type, scope::Scope};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ret_type: Type,

    pub arg_types: Vec<Type>,
    pub arg_names: Vec<String>,

    pub scope: Scope,
}