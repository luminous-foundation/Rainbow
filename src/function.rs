use crate::{_type::Type, scope::Scope};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub ret_type: Type,

    pub arg_types: Vec<Type>,
    pub arg_names: Vec<String>,

    pub scope: Scope,
}

#[derive(Debug, Clone)]
pub struct Extern {
    pub name: String,
    pub access_name: String,
    pub ret_type: Type,

    pub arg_types: Vec<Type>,

    pub dll: String,
}