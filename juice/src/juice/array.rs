use super::{flow::Flow, value::Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Array {
    pub elements: Rc<RefCell<Vec<Value>>>,
}

impl Array {
    pub fn new(array: Vec<Value>) -> Self {
        Self {
            elements: Rc::new(RefCell::new(array)),
        }
    }

    fn check_index(&self, index: i32) -> Result<Value, Flow> {
        if index < 0 || index >= self.elements.borrow().len() as i32 {
            return Err(Flow::Error("Index out of bounds".to_string()));
        }
        Ok(Value::Void)
    }

    pub fn get_value(&self, index: i32) -> Result<Value, Flow> {
        self.check_index(index)?;
        Ok(self.elements.borrow()[index as usize].clone())
    }

    pub fn set_value(&self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut()[index as usize] = value;
        Ok(Value::Void)
    }

    pub fn length(&self) -> Result<Value, Flow> {
        Ok(Value::Number(self.elements.borrow().len() as f64))
    }

    pub fn add(&self, value: Value) -> Result<Value, Flow> {
        self.elements.borrow_mut().push(value);
        Ok(Value::Void)
    }

    pub fn insert(&self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut().insert(index as usize, value);
        Ok(Value::Void)
    }

    pub fn remove_at(&self, index: i32) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut().remove(index as usize);
        Ok(Value::Void)
    }

    pub fn remove(&self, value: Value) -> Result<Value, Flow> {
        if let Some(index) = self.elements.borrow().iter().position(|x| x == &value) {
            self.elements.borrow_mut().remove(index);
        }
        Ok(Value::Void)
    }

    pub fn clear(&self) -> Result<Value, Flow> {
        self.elements.borrow_mut().clear();
        Ok(Value::Void)
    }
}
