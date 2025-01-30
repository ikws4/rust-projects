use crate::ast::MethodDeclaration;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::interpreter::{Env, Interpreter};

pub trait TMethod {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Flow>;
    fn arity(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Object(Object),
    Method(Method),
    NativeMethod(NativeMethod),
    Array(Array),
    Null,
    Void,
}

pub enum Flow {
    Return(Value),
    Break,
    Continue,
    Error(String),
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

    pub fn as_string(&self) -> Result<String, Flow> {
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

    pub fn as_object(&self) -> Result<Object, Flow> {
        match self {
            Value::Object(o) => Ok(o.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for object operation".to_string(),
            )),
        }
    }

    pub fn as_array(&self) -> Result<Array, Flow> {
        match self {
            Value::Array(a) => Ok(a.clone()),
            _ => Err(Flow::Error(
                "Invalid operands for array operation".to_string(),
            )),
        }
    }

    pub fn as_method(&self) -> Result<Method, Flow> {
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
            Value::NativeMethod(_) => true,
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
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
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
            (Value::String(s), Value::Number(n)) => Ok(Value::String(s.repeat(*n as usize))),
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

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    methods: Rc<RefCell<HashMap<String, Method>>>,
    fields: Rc<RefCell<HashMap<String, Value>>>,
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

    pub fn get_method(&self, name: &String) -> Result<impl TMethod, Flow> {
        if let Some(method) = self.methods.borrow().get(name) {
            return Ok(method.clone());
        }
        Err(Flow::Error(format!("Method {} not found", name)))
    }

    pub fn set_method(&mut self, name: String, method: Method) -> Result<Value, Flow> {
        self.methods.borrow_mut().insert(name, method);
        Ok(Value::Void)
    }

    pub fn get_value(&self, name: &String) -> Result<Value, Flow> {
        if let Some(value) = self.fields.borrow().get(name) {
            return Ok(value.clone());
        }
        Err(Flow::Error(format!("Field {} not found", name)))
    }

    pub fn set_value(&mut self, name: String, value: Value) -> Result<Value, Flow> {
        self.fields.borrow_mut().insert(name, value);
        Ok(Value::Void)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    pub elements: Rc<RefCell<Vec<Value>>>,
}

impl Array {
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

    pub fn set_value(&mut self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut()[index as usize] = value;
        Ok(Value::Void)
    }

    pub fn length(&self) -> Result<Value, Flow> {
        Ok(Value::Number(self.elements.borrow().len() as f64))
    }

    pub fn add(&mut self, value: Value) -> Result<Value, Flow> {
        self.elements.borrow_mut().push(value);
        Ok(Value::Void)
    }

    pub fn insert(&mut self, index: i32, value: Value) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut().insert(index as usize, value);
        Ok(Value::Void)
    }

    pub fn remove_at(&mut self, index: i32) -> Result<Value, Flow> {
        self.check_index(index)?;
        self.elements.borrow_mut().remove(index as usize);
        Ok(Value::Void)
    }

    pub fn remove(&mut self, value: Value) -> Result<Value, Flow> {
        if let Some(index) = self.elements.borrow().iter().position(|x| x == &value) {
            self.elements.borrow_mut().remove(index);
        }
        Ok(Value::Void)
    }

    pub fn clear(&mut self) -> Result<Value, Flow> {
        self.elements.borrow_mut().clear();
        Ok(Value::Void)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NativeMethod {
    pub function: Rc<fn(Vec<Value>) -> Result<Value, Flow>>,
    pub arity: usize,
}

impl TMethod for NativeMethod {
    fn call(&self, _: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Flow> {
        (self.function)(arguments)
    }

    fn arity(&self) -> usize {
        self.arity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    pub declaration: MethodDeclaration,
    pub object: Object,
}

impl TMethod for Method {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Flow> {
        let parent = interpreter.env.clone();
        let env = Rc::new(RefCell::new(Env::new(Some(parent.clone()))));

        env.borrow_mut()
            .define("this".to_string(), Value::Object(self.object.clone()));

        // Bind parameters to arguments
        for (param, arg) in self.declaration.signature.params.iter().zip(arguments) {
            env.borrow_mut().define(param.name.clone(), arg);
        }

        // Execute method body
        interpreter.env = env;
        let ret = interpreter.interpret(&self.declaration.body);
        interpreter.env = parent;

        match ret {
            Ok(value) => Ok(value),
            Err(flow) => match flow {
                Flow::Break => Err(Flow::Error("Break statement outside of loop".to_string())),
                Flow::Continue => Err(Flow::Error(
                    "Continue statement outside of loop".to_string(),
                )),
                Flow::Error(err) => Err(Flow::Error(err)),
                Flow::Return(value) => Ok(value),
            },
        }
    }

    fn arity(&self) -> usize {
        self.declaration.signature.params.len()
    }
}
