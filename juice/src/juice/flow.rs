use std::fmt::Debug;

use super::value::Value;

#[derive(Clone, PartialEq)]
pub enum Flow {
    Return(Value),
    Break,
    Continue,
    Error(String),
}

impl Debug for Flow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Flow::Return(value) => write!(f, "Return({})", value),
            Flow::Break => write!(f, "Break"),
            Flow::Continue => write!(f, "Continue"),
            Flow::Error(message) => write!(f, "Error({})", message),
        }
    }
}
