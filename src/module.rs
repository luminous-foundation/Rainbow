use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct Module {
    pub scope: Scope,

    pub frame: usize,
}