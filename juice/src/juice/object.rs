use super::{flow::Flow, method::Method, value::Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Object {
    pub methods: HashMap<String, Value>,
    pub fields: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            methods: HashMap::new(),
            fields: HashMap::new(),
        }
    }

    pub fn instantiate(&self) -> Self {
        Self {
            methods: self.methods.clone(),
            fields: HashMap::new(),
        }
    }

    pub fn get_method(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.methods.get(name) {
            return Ok(value.clone());
        }
        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn define_method(&mut self, name: String, method: Method) -> Result<Value, Flow> {
        let method = Value::Method(Rc::new(RefCell::new(method)));
        self.methods.insert(name, method);
        Ok(Value::Void)
    }

    pub fn get_value(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.fields.get(name) {
            return Ok(value.clone());
        }
        Err(Flow::Error(format!("Field {} not found", name)))
    }

    pub fn set_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        self.fields.insert(name, value);
        Ok(Value::Void)
    }
}
