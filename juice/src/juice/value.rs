use std::{cell::RefCell, fmt::{Debug, Display}, rc::Rc};

use super::{
    array::Array, flow::Flow, method::Method, native_function::NativeFunction, object::Object,
};

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(Rc<RefCell<String>>),
    Object(Rc<RefCell<Object>>),
    Method(Rc<RefCell<Method>>),
    NativeFunction(Rc<RefCell<NativeFunction>>),
    Array(Rc<RefCell<Array>>),
    Null,
    Void,
}

impl Value {
    pub fn as_number(&self) -> Result<f64, Flow> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(Flow::Error(
                "Invalid operands for number operation".to_string(),
            )),
        }
    }

    pub fn as_string(&self) -> Result<Rc<RefCell<String>>, Flow> {
        match self {
            Value::String(s) => Ok(s.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for string operation".to_string(),
            )),
        }
    }

    pub fn as_bool(&self) -> Result<bool, Flow> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(Flow::Error(
                "Invalid operands for boolean operation".to_string(),
            )),
        }
    }

    pub fn as_object(&self) -> Result<Rc<RefCell<Object>>, Flow> {
        match self {
            Value::Object(o) => Ok(o.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for object operation".to_string(),
            )),
        }
    }

    pub fn as_array(&self) -> Result<Rc<RefCell<Array>>, Flow> {
        match self {
            Value::Array(a) => Ok(a.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for array operation".to_string(),
            )),
        }
    }

    pub fn as_method(&self) -> Result<Rc<RefCell<Method>>, Flow> {
        match self {
            Value::Method(m) => Ok(m.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for method operation".to_string(),
            )),
        }
    }

    pub fn is_method(&self) -> bool {
        match self {
            Value::Method(_) => true,
            _ => false,
        }
    }

    pub fn is_native_method(&self) -> bool {
        match self {
            Value::NativeFunction(_) => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Value::Void => true,
            _ => false,
        }
    }

    pub fn is_truthy(&self) -> Result<bool, Flow> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(Flow::Error(
                "Invalid operands for boolean operation".to_string(),
            )),
        }
    }

    pub fn and(&self, rhs: &Value) -> Result<Value, Flow> {
        Ok(Value::Bool(self.is_truthy()? && rhs.is_truthy()?))
    }

    pub fn or(&self, rhs: &Value) -> Result<Value, Flow> {
        Ok(Value::Bool(self.is_truthy()? || rhs.is_truthy()?))
    }

    pub fn not(&self) -> Result<Value, Flow> {
        Ok(Value::Bool(self.is_truthy()?))
    }

    pub fn neg(&self) -> Result<Value, Flow> {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(Flow::Error(
                "Invalid operands for negation operation".to_string(),
            )),
        }
    }

    pub fn add(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => {
                Ok(Value::String(Rc::new(RefCell::new(format!("{}{}", a.borrow(), b.borrow())))))
            }
            _ => Err(Flow::Error(
                "Invalid operands for add operation".to_string(),
            )),
        }
    }

    pub fn sub(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            _ => Err(Flow::Error(
                "Invalid operands for subtraction operation".to_string(),
            )),
        }
    }

    pub fn mul(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            _ => Err(Flow::Error(
                "Invalid operands for multiplication operation".to_string(),
            )),
        }
    }

    pub fn div(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 {
                    Err(Flow::Error("Division by zero".to_string()))
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            _ => Err(Flow::Error(
                "Invalid operands for division operation".to_string(),
            )),
        }
    }

    pub fn rem(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
            _ => Err(Flow::Error(
                "Invalid operands for remainder operation".to_string(),
            )),
        }
    }

    pub fn eq(&self, rhs: &Value) -> Result<Value, Flow> {
        Ok(Value::Bool(self == rhs))
    }

    pub fn ne(&self, rhs: &Value) -> Result<Value, Flow> {
        Ok(Value::Bool(self != rhs))
    }

    pub fn lt(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
            _ => Err(Flow::Error(
                "Invalid operands for less than operation".to_string(),
            )),
        }
    }

    pub fn gt(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
            _ => Err(Flow::Error(
                "Invalid operands for greater than operation".to_string(),
            )),
        }
    }

    pub fn le(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
            _ => Err(Flow::Error(
                "Invalid operands for less than or equal operation".to_string(),
            )),
        }
    }

    pub fn ge(&self, rhs: &Value) -> Result<Value, Flow> {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
            _ => Err(Flow::Error(
                "Invalid operands for greater than or equal operation".to_string(),
            )),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s.borrow()),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, elem) in arr.borrow().elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{ ")?;
                let mut first = true;
                for (key, val) in obj.borrow().fields.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    write!(f, "{} = {}", key, val)?;
                }
                write!(f, " }}")
            }
            Value::Method(method) => {
                write!(f, "<method {}>", method.borrow().declaration.signature.name)
            }
            Value::NativeFunction(method) => {
                write!(f, "<native method {:p}>", method.borrow().function)
            }
            Value::Null => write!(f, "null"),
            Value::Void => write!(f, "void"),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
