use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{flow::Flow, method::Method, value::Value};

#[derive(Clone, PartialEq)]
pub struct Object {
    pub methods: Rc<RefCell<HashMap<String, Method>>>,
    pub fields: Rc<RefCell<HashMap<String, Value>>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            methods: Rc::new(RefCell::new(HashMap::new())),
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn instantiate(&self) -> Self {
        let methods = self.methods.borrow().clone();
        Self {
            methods: Rc::new(RefCell::new(methods)),
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get_method(&self, name: &str) -> Result<Method, Flow> {
        if let Some(method) = self.methods.borrow().get(name) {
            return Ok(method.clone());
        }
        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn set_method(&self, name: String, method: Method) -> Result<Value, Flow> {
        self.methods.borrow_mut().insert(name, method);
        Ok(Value::Void)
    }

    pub fn get_value(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.fields.borrow().get(name) {
            return Ok(value.clone());
        }
        Err(Flow::Error(format!("Field {} not found", name)))
    }

    pub fn set_value(&self, name: String, value: Value) -> Result<Value, Flow> {
        self.fields.borrow_mut().insert(name, value);
        Ok(Value::Void)
    }
}
