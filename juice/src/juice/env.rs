use std::{cell::RefCell, rc::Rc};

use super::{flow::Flow, method::Method, native_function::NativeFunction, object::Object, value::Value};

pub struct Env {
    stack: Vec<Rc<RefCell<Object>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            stack: vec![Rc::new(RefCell::new(Object::new()))],
        }
    }

    pub fn push(&mut self, object: Rc<RefCell<Object>>) {
        self.stack.push(object);
    }

    pub fn push_default(&mut self) {
        self.stack.push(Rc::new(RefCell::new(Object::new())));
    }

    pub fn pop(&mut self) -> Result<Value, Flow> {
        if self.stack.len() == 1 {
            return Err(Flow::Error(
                "Cannot pop the default environment".to_string(),
            ));
        }
        self.stack.pop();
        Ok(Value::Void)
    }

    pub fn current(&self) -> Rc<RefCell<Object>> {
        self.stack.last().unwrap().clone()
    }

    pub fn define_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        self.current().borrow_mut().define_value(name, value)
    }

    pub fn set_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        for object in self.stack.iter().rev() {
            if object.borrow().values.contains_key(&name) {
                return object.borrow_mut().set_value(name, value);
            }
        }
        Err(Flow::Error(format!("Variable {} not found", name)))
    }

    pub fn get_value(&self, name: &str) -> Result<Value, Flow> {
        for object in self.stack.iter().rev() {
            if let Ok(value) = object.borrow().get_value(name) {
                return Ok(value);
            }
        }
        Err(Flow::Error(format!("Variable {} not found", name)))
    }

    pub fn get_method(&self, name: &str) -> Result<Value, Flow> {
        for object in self.stack.iter().rev() {
            if let Ok(value) = object.borrow().get_method(name) {
                return Ok(value);
            }
        }
        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn define_method(&mut self, name: String, method: Method) -> Result<Value, Flow> {
        self.current().borrow_mut().define_method(name, method)
    }

    pub fn define_native_function(&mut self, name: String, function: NativeFunction) -> Result<Value, Flow> {
        self.current().borrow_mut().define_native_function(name, function)
    }
}
