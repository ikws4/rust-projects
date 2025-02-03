use super::{flow::Flow, interpreter::Interpreter, object::Object, traits::Callable, value::Value};
use crate::ast::MethodDeclaration;
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Method {
    pub declaration: MethodDeclaration,
    pub this: Option<Rc<RefCell<Object>>>,
    pub min_arity: usize,
    pub max_arity: usize,
}

impl Method {
    pub fn new(declaration: MethodDeclaration) -> Self {
        let arity = declaration.signature.params.len();
        Self {
            declaration,
            this: None,
            min_arity: arity,
            max_arity: arity,
        }
    }

    pub fn bind(&mut self, object: Rc<RefCell<Object>>) {
        self.this = Some(object);
    }
}

impl Callable for Method {
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

        if let Some(this) = self.this.clone() {
            interpreter.env.push(this.clone());
            interpreter.env.push_default();

            // Bind `this` to the object
            interpreter
                .env
                .define_value("this".to_string(), Value::Object(this))?;

            // Bind parameters to arguments
            for (param, arg) in self.declaration.signature.params.iter().zip(arguments) {
                interpreter
                    .env
                    .define_value(param.name.clone(), arg.clone())?;
            }
            let ret = interpreter.execute_statements(&self.declaration.body);

            interpreter.env.pop()?;
            interpreter.env.pop()?;

            match ret {
                Ok(_) => Ok(Value::Void),
                Err(flow) => match flow {
                    Flow::Return(value) => Ok(value),
                    Flow::Break => Err(Flow::Error("Break statement outside of loop".to_string())),
                    Flow::Continue => Err(Flow::Error(
                        "Continue statement outside of loop".to_string(),
                    )),
                    Flow::Error(err) => Err(Flow::Error(err)),
                },
            }
        } else {
            return Err(Flow::Error("Method not bound to an object".to_string()));
        }
    }
}
