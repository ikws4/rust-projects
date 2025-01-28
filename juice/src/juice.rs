use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::ast::{BinaryOp, Expression, MethodDeclaration, Statement, UnaryOp};

// First, define Callable trait
pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, String>;
    fn arity(&self) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method {
    pub declaration: MethodDeclaration,
    pub object: Rc<RefCell<HashMap<String, Value>>>,
}

impl Callable for Method {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, String> {
        // Create new environment for method scope
        let mut method_env = Environment::new();

        // Bind "this" to the object
        method_env.define("this".to_string(), Value::Object(self.object.clone()));

        // Bind parameters to arguments
        for (param, arg) in self.declaration.signature.params.iter().zip(arguments) {
            method_env.define(param.name.clone(), arg);
        }

        // Create new environment with method scope
        let previous_env = interpreter.environment.clone();
        interpreter.environment = method_env;

        // Execute method body
        let mut result = Value::Null;
        for stmt in &self.declaration.body {
            match interpreter.execute_statement(stmt) {
                Ok(_) => continue,
                Err(e) => {
                    interpreter.environment = previous_env;
                    return Err(e);
                }
            }
            // TODO: Handle return statements to break execution and return value
        }

        // Restore previous environment
        interpreter.environment = previous_env;

        Ok(result)
    }

    fn arity(&self) -> usize {
        self.declaration.signature.params.len()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Object(Rc<RefCell<HashMap<String, Value>>>),
    Array(Vec<Value>),
    Callable(Method),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Object(o) => {
                write!(f, "{{")?;
                let o = o.borrow();
                let mut first = true;
                for (k, v) in o.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} = {}", k, v)?;
                    first = false;
                }
                write!(f, "}}")
            }
            Value::Array(a) => {
                write!(f, "[")?;
                let mut first = true;
                for v in a {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                    first = false;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "null"),
            Value::Callable(method) => write!(f, "{:?}", method),
        }
    }
}

#[derive(Default, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned()
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
            Ok(())
        } else {
            Err(format!("Undefined variable '{}'", name))
        }
    }
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
        for stmt in statements {
            self.execute_statement(&stmt)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Object { name, methods, .. } => {
                let obj = Rc::new(RefCell::new(HashMap::new()));

                // Create method objects and bind them to the object
                for method_decl in methods {
                    let method = Method {
                        declaration: method_decl.clone(),
                        object: obj.clone(),
                    };
                    obj.borrow_mut().insert(
                        method_decl.signature.name.clone(),
                        Value::Callable(method),
                    );
                }

                self.environment.define(name.clone(), Value::Object(obj));
            }
            Statement::Var {
                name, initializer, ..
            } => {
                let value = self.evaluate_expression(initializer)?;
                self.environment.define(name.clone(), value);
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr)?;
            }
            Statement::While { condition, body } => {
                let mut value = self.evaluate_expression(condition)?;
                while self.is_truthy(&value) {
                    for stmt in body {
                        self.execute_statement(stmt)?;
                    }
                    value = self.evaluate_expression(condition)?;
                }
            }
            Statement::For {
                variable,
                iterator,
                body,
            } => {
                let iter_value = self.evaluate_expression(iterator)?;
                if let Value::Array(items) = iter_value {
                    for item in items {
                        self.environment.define(variable.clone(), item);
                        for stmt in body {
                            self.execute_statement(stmt)?;
                        }
                    }
                } else {
                    return Err("Can only iterate over arrays".to_string());
                }
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let ret = &self.evaluate_expression(condition)?;
                if self.is_truthy(ret) {
                    for stmt in then_branch {
                        self.execute_statement(stmt)?;
                    }
                } else if let Some(else_statements) = else_branch {
                    for stmt in else_statements {
                        self.execute_statement(stmt)?;
                    }
                }
            }
            // Add other statement types as needed
            _ => {
                // Temporarily ignore other statement types
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::NumberLiteral(n) => Ok(Value::Number(n.parse().unwrap())),
            Expression::StringLiteral(s) => Ok(Value::String(s[1..s.len() - 1].to_string())), // Remove quotes
            Expression::BoolLiteral(b) => Ok(Value::Bool(*b)),
            Expression::Null => Ok(Value::Null),
            Expression::Identifier(name) => self
                .environment
                .get(name)
                .ok_or_else(|| format!("Undefined variable '{}'", name)),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.evaluate_binary_op(operator, &left_val, &right_val)
            }
            Expression::Unary { operator, operand } => {
                let value = self.evaluate_expression(operand)?;
                self.evaluate_unary_op(operator, &value)
            }
            Expression::ObjectConstruction { fields, .. } => {
                let mut object = HashMap::new();
                for (name, expr) in fields {
                    let value = self.evaluate_expression(expr)?;
                    object.insert(name.clone(), value);
                }
                Ok(Value::Object(Rc::new(RefCell::new(object))))
            }
            Expression::ArrayConstruction { elements } => {
                let mut array = Vec::new();
                for expr in elements {
                    array.push(self.evaluate_expression(expr)?);
                }
                Ok(Value::Array(array))
            }
            Expression::Call { callee, arguments } => {
                let callee = self.evaluate_expression(callee)?;
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    evaluated_args.push(self.evaluate_expression(arg)?);
                }
                self.call_method(&callee, evaluated_args)
            }
            Expression::MemberAccess {
                object,
                member,
                arguments,
            } => {
                let obj = self.evaluate_expression(object)?;
                if let Value::Object(obj_ref) = obj {
                    let obj = obj_ref.borrow();
                    if let Some(value) = obj.get(member) {
                        match arguments {
                            Some(args) => {
                                let mut evaluated_args = Vec::new();
                                for arg in args {
                                    evaluated_args.push(self.evaluate_expression(arg)?);
                                }
                                self.call_method(value, evaluated_args)
                            }
                            None => Ok(value.clone()),
                        }
                    } else {
                        Err(format!("Undefined property '{}'", member))
                    }
                } else {
                    Err("Can only access properties on objects".to_string())
                }
            }
            Expression::Assignment { target, value } => {
                let value = self.evaluate_expression(value)?;
                match &**target {
                    Expression::Identifier(name) => {
                        self.environment.assign(name.clone(), value.clone())?;
                        Ok(value)
                    }
                    Expression::MemberAccess {
                        object,
                        member,
                        arguments: None,
                    } => {
                        if let Value::Object(obj_ref) = self.evaluate_expression(object)? {
                            obj_ref.borrow_mut().insert(member.clone(), value.clone());
                            Ok(value)
                        } else {
                            Err("Can only assign to object properties".to_string())
                        }
                    }
                    _ => Err("Invalid assignment target".to_string()),
                }
            }
            Expression::ArrayAccess { array, index } => {
                let array_val = self.evaluate_expression(array)?;
                let index_val = self.evaluate_expression(index)?;

                match (array_val, index_val) {
                    (Value::Array(arr), Value::Number(i)) => {
                        let idx = i as usize;
                        if idx < arr.len() {
                            Ok(arr[idx].clone())
                        } else {
                            Err(format!("Index {} out of bounds", idx))
                        }
                    }
                    _ => Err("Can only index into arrays with numbers".to_string()),
                }
            }
            // Add other expression types as needed
            _ => Err("Unsupported expression type".to_string()),
        }
    }

    // Add method to handle method calls
    fn call_method(&mut self, method: &Value, arguments: Vec<Value>) -> Result<Value, String> {
        match method {
            Value::Callable(callable) => {
                if arguments.len() != callable.arity() {
                    return Err(format!(
                        "Expected {} arguments but got {}.",
                        callable.arity(),
                        arguments.len()
                    ));
                }
                callable.call(self, arguments)
            }
            _ => Err("Can only call methods and functions.".to_string()),
        }
    }

    fn evaluate_binary_op(
        &self,
        op: &BinaryOp,
        left: &Value,
        right: &Value,
    ) -> Result<Value, String> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err("Invalid operand types for addition".to_string()),
            },
            // Add other operators as needed
            _ => Err("Unsupported operator".to_string()),
        }
    }

    fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
        match op {
            UnaryOp::Negate => match operand {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err("Operand must be a number".to_string()),
            },
            UnaryOp::Not => match operand {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                _ => Err("Operand must be a boolean".to_string()),
            },
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            _ => false,
        }
    }
}
