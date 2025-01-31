use super::{flow::Flow, value::Value};

#[derive(Clone, PartialEq)]
pub struct Array {
    pub elements: Vec<Value>,
}

impl Array {
    pub fn new(array: Vec<Value>) -> Self {
        Self {
            elements: array,
        }
    }

    fn check_index(&self, index: i32) -> Result<Value, Flow> {
        if index < 0 || index >= self.elements.len() as i32 {
            return Err(Flow::Error("Index out of bounds".to_string()));
        }
        Ok(Value::Void)
    }

    pub fn get_value(&self, index: i32) -> Result<Value, Flow> {
        self.check_index(index)?;
        Ok(self.elements[index as usize].clone())
    }

    pub fn set_value(&mut self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements[index as usize] = value;
        Ok(Value::Void)
    }

    pub fn length(&self) -> Result<Value, Flow> {
        Ok(Value::Number(self.elements.len() as f64))
    }

    pub fn add(&mut self, value: Value) -> Result<Value, Flow> {
        self.elements.push(value);
        Ok(Value::Void)
    }

    pub fn insert(&mut self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.insert(index as usize, value);
        Ok(Value::Void)
    }

    pub fn remove_at(&mut self, index: i32) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.remove(index as usize);
        Ok(Value::Void)
    }

    pub fn remove(&mut self, value: Value) -> Result<Value, Flow> {
        if let Some(index) = self.elements.iter().position(|x| x == &value) {
            self.elements.remove(index);
        }
        Ok(Value::Void)
    }

    pub fn clear(&mut self) -> Result<Value, Flow> {
        self.elements.clear();
        Ok(Value::Void)
    }
}
