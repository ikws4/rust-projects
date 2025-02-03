use super::{flow::Flow, native_method::NativeMethod, object::Object, value::Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Array {
    pub elements: Vec<Value>,
    pub object_wrapper: Option<Rc<RefCell<Object>>>,
}

impl Array {
    pub fn new(array: Vec<Value>) -> Self {
        Self {
            elements: array,
            object_wrapper: None,
        }
    }

    pub fn wrap(&mut self, this: Rc<RefCell<Value>>) -> Result<Value, Flow> {
        let mut object = Object::new();

        object.define_method(
            "length".to_string(),
            Value::new_native_method(NativeMethod::new(Self::length, this.clone(), 0, 0)),
        )?;
        object.define_method(
            "add".to_string(),
            Value::new_native_method(NativeMethod::new(Self::add, this.clone(), 1, 1)),
        )?;
        object.define_method(
            "insert".to_string(),
            Value::new_native_method(NativeMethod::new(Self::insert, this.clone(), 2, 2)),
        )?;
        object.define_method(
            "removeAt".to_string(),
            Value::new_native_method(NativeMethod::new(Self::remove_at, this.clone(), 1, 1)),
        )?;
        object.define_method(
            "remove".to_string(),
            Value::new_native_method(NativeMethod::new(Self::remove, this.clone(), 1, 1)),
        )?;
        object.define_method(
            "clear".to_string(),
            Value::new_native_method(NativeMethod::new(Self::clear, this.clone(), 0, 0)),
        )?;

        self.object_wrapper = Some(Rc::new(RefCell::new(object)));

        Ok(Value::Void)
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

    pub fn length(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        let length = array.borrow().elements.len() as f64;
        Ok(Value::Number(length))
    }

    pub fn add(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        for value in values {
            array.borrow_mut().elements.push(value.clone());
        }
        Ok(Value::Void)
    }

    pub fn insert(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        let index = values[0].as_number()? as i32;
        let value = values[1].clone();
        array.borrow_mut().check_index(index)?;
        array.borrow_mut().elements.insert(index as usize, value);
        Ok(Value::Void)
    }

    pub fn remove_at(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        let index = values[0].as_number()? as i32;
        array.borrow_mut().check_index(index)?;
        array.borrow_mut().elements.remove(index as usize);
        Ok(Value::Void)
    }

    pub fn remove(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        let value = &values[0];
        if let Some(index) = array.borrow().elements.iter().position(|x| x == value) {
            array.borrow_mut().elements.remove(index);
        }
        Ok(Value::Void)
    }

    pub fn clear(this: &Value, values: &Vec<Value>) -> Result<Value, Flow> {
        let array = this.as_array()?;
        array.borrow_mut().elements.clear();
        Ok(Value::Void)
    }
}
