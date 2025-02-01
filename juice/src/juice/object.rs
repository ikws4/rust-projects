use super::{flow::Flow, method::Method, value::Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Object {
    pub values: HashMap<String, Value>,
    pub methods: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Object>>>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            methods: HashMap::new(),
            parent: None,
        }
    }

    pub fn instantiate(&self) -> Self {
        Self {
            values: HashMap::new(),
            methods: self.methods.clone(),
            parent: self.parent.clone(),
        }
    }

    pub fn get_method(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.methods.get(name) {
            return Ok(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_method(name);
        }

        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn set_method(&mut self, name: String, method: Method) -> Result<Value, Flow> {
        let method = Value::Method(Rc::new(RefCell::new(method)));
        self.methods.insert(name, method);
        Ok(Value::Void)
    }

    pub fn get_value(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get_value(name);
        }

        Err(Flow::Error(format!("Field {} not found", name)))
    }

    pub fn define_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        if self.values.contains_key(&name) {
            return Err(Flow::Error("Field already defined".to_string()));
        }

        self.values.insert(name, value);
        Ok(Value::Void)
    }

    pub fn set_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return Ok(Value::Void);
        }

        if let Some(parent) = &mut self.parent {
            parent.borrow_mut().set_value(name, value)
        } else {
            Err(Flow::Error(format!("Variable {} not found", name)))
        }
    }
}
