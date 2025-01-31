use super::{flow::Flow, interpreter::Interpreter, object::Object, value::Value};
use crate::ast::MethodDeclaration;
use std::rc::Rc;

pub trait TMethod {
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Flow>;
}

#[derive(Clone, PartialEq)]
pub struct NativeMethod {
    function: Rc<fn(&Vec<Value>) -> Result<Value, Flow>>,
    min_arity: usize,
    max_arity: usize,
}

#[derive(Clone, PartialEq)]
pub struct Method {
    pub declaration: MethodDeclaration,
    object: Object,
    min_arity: usize,
    max_arity: usize,
}

impl NativeMethod {
    pub fn new(
        function: Rc<fn(&Vec<Value>) -> Result<Value, Flow>>,
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

impl TMethod for NativeMethod {
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

impl Method {
    pub fn new(declaration: MethodDeclaration, object: Object) -> Self {
        let arity = declaration.signature.params.len();
        Self {
            declaration,
            object,
            min_arity: arity,
            max_arity: arity,
        }
    }
}

impl TMethod for Method {
    fn call(&self, interpreter: &mut Interpreter, arguments: &Vec<Value>) -> Result<Value, Flow> {
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

        let ret = interpreter.execute_block_with_closure(&self.declaration.body, |env| {
            // Bind `this` to the object
            env.borrow_mut()
                .define("this".to_string(), Value::Object(self.object.clone()))?;

            // Bind parameters to arguments
            for (param, arg) in self.declaration.signature.params.iter().zip(arguments) {
                env.borrow_mut().define(param.name.clone(), arg.clone())?;
            }

            Ok(Value::Void)
        });

        match ret {
            Ok(value) => Ok(value),
            Err(flow) => match flow {
                Flow::Break => Err(Flow::Error("Break statement outside of loop".to_string())),
                Flow::Continue => Err(Flow::Error(
                    "Continue statement outside of loop".to_string(),
                )),
                Flow::Error(err) => Err(Flow::Error(err)),
                Flow::Return(value) => Ok(value),
            },
        }
    }
}
