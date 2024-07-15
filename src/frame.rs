use crate::{value::Value, variable::Variable};

#[derive(Debug)]
pub struct Frame {
    pub vars: Vec<Variable>,
    pub stack: Vec<Value>,
}