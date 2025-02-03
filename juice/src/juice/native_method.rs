use super::{flow::Flow, interpreter::Interpreter, traits::Callable, value::Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct NativeMethod {
    pub function: fn(this: &Value, &Vec<Value>) -> Result<Value, Flow>,
    pub this: Rc<RefCell<Value>>,
    pub min_arity: usize,
    pub max_arity: usize,
}

impl NativeMethod {
    pub fn new(
        function: fn(&Value, &Vec<Value>) -> Result<Value, Flow>,
        this: Rc<RefCell<Value>>,
        min_arity: usize,
        max_arity: usize,
    ) -> Self {
        Self {
            function,
            this,
            min_arity,
            max_arity,
        }
    }
}

impl Callable for NativeMethod {
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

        (self.function)(&self.this.borrow(), arguments)
    }
}
