use super::{flow::Flow, value::Value};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Object {
    pub values: HashMap<String, Value>,
    pub methods: HashMap<String, Value>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn instantiate(&self) -> Self {
        Self {
            values: HashMap::new(),
            methods: self.methods.clone(),
        }
    }

    pub fn get_method(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.methods.get(name) {
            return Ok(value.clone());
        }

        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn define_method(&mut self, name: String, method: Value) -> Result<Value, Flow> {
        self.methods.insert(name, method);
        Ok(Value::Void)
    }

    pub fn get_value(&self, name: &str) -> Result<Value, Flow> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
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

        Err(Flow::Error(format!("Variable {} not found", name)))
    }
}
