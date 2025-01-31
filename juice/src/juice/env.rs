use super::{flow::Flow, value::Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Env {
    pub values: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(parent: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn define(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        if self.values.contains_key(&name) {
            return Err(Flow::Error("Variable already defined".to_string()));
        }

        self.values.insert(name, value);
        Ok(Value::Void)
    }

    pub fn get(&self, name: &String) -> Result<Value, Flow> {
        if self.values.contains_key(name) {
            return Ok(self.values.get(name).unwrap().clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        Err(Flow::Error(format!("Variable {} not defined", name)))
    }

    pub fn set(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(Value::Void);
        }

        if let Some(parent) = &mut self.parent {
            parent.borrow_mut().set(name, value)
        } else {
            Err(Flow::Error(format!("Variable {} not defined", name)))
        }
    }
}
