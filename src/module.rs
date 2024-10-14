use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,

    pub scope: Scope,

    pub frame: usize,
}