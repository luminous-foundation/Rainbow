use indexmap::IndexMap;

use crate::_type::Type;

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,

    pub size: usize,

    pub var_names: Vec<String>,
    pub var_types: Vec<Type>,
    pub var_offsets: IndexMap<String, usize>,
}
