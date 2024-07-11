use crate::number::Number;

#[derive(Debug)]
pub enum Argument {
    IMM(Number),
    NAME(String)
}