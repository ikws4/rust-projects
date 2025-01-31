use super::{flow::Flow, interpreter::Interpreter, traits::TCall, value::Value};

#[derive(Clone, PartialEq)]
pub struct NativeFunction {
    pub function: fn(&Vec<Value>) -> Result<Value, Flow>,
    pub min_arity: usize,
    pub max_arity: usize,
}

impl NativeFunction {
    pub fn new(
        function: fn(&Vec<Value>) -> Result<Value, Flow>,
        min_arity: usize,
        max_arity: usize,
    ) -> Self {
        Self {
            function,
            min_arity,
            max_arity,
        }
    }
}

impl TCall for NativeFunction {
    fn call(&self, _: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Flow> {
        let arity = arguments.len();
        if arity < self.min_arity || arity > self.max_arity {
            if self.min_arity != self.max_arity {
                return Err(Flow::Error(format!(
                    "Expected ({}, {}] arguments but got {}",
                    self.min_arity,
                    self.max_arity,
                    arguments.len()
                )));
            }

            return Err(Flow::Error(format!(
                "Expected {} arguments but got {}",
                arity,
                arguments.len()
            )));
        }

        (self.function)(arguments)
    }
}
