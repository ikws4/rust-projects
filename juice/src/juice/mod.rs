mod value;
mod interpreter;

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;

// use value::Value;

// use crate::ast::{BinaryOp, Expression, Statement, UnaryOp};

// #[derive(Debug)]
// pub enum Return {
//     Value(Value),
//     None,
// }

// #[derive(Default, Clone)]
// pub struct Environment {
//     variables: HashMap<String, Value>,
// }

// impl Environment {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn define(&mut self, name: String, value: Value) {
//         self.variables.insert(name, value);
//     }

//     pub fn get(&self, name: &str) -> Option<Value> {
//         self.variables.get(name).cloned()
//     }

//     pub fn assign(&mut self, name: String, value: Value) -> Result<(), String> {
//         if self.variables.contains_key(&name) {
//             self.variables.insert(name, value);
//             Ok(())
//         } else {
//             Err(format!("Undefined variable '{}'", name))
//         }
//     }
// }

// pub struct Interpreter {
//     environment: Environment,
// }

// impl Interpreter {
//     pub fn new() -> Self {
//         let mut interpreter = Self {
//             environment: Environment::new(),
//         };

//         // Define built-in functions
//         interpreter.define_natives();

//         interpreter
//     }

//     fn define_natives(&mut self) {
//         // Define print function
//         self.environment.define(
//             "print".to_string(),
//             Value::NativeFunction(|args: Vec<Value>| {
//                 let output: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
//                 println!("{}", output.join(" "));
//                 Ok(Value::Null)
//             }),
//         );
//     }

//     pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<(), String> {
//         for stmt in statements {
//             match self.execute_statement(&stmt)? {
//                 Return::Value(_) => {
//                     // Return statements at top level are ignored
//                     continue;
//                 }
//                 Return::None => continue,
//             }
//         }
//         Ok(())
//     }

//     fn execute_statement(&mut self, statement: &Statement) -> Result<Return, String> {
//         match statement {
//             Statement::Object { name, methods, .. } => {
//                 let obj = Rc::new(RefCell::new(HashMap::new()));

//                 // Create method objects and bind them to the object
//                 for method_decl in methods {
//                     let method = Method {
//                         declaration: method_decl.clone(),
//                         object: obj.clone(),
//                     };
//                     obj.borrow_mut()
//                         .insert(method_decl.signature.name.clone(), Value::Callable(method));
//                 }

//                 self.environment.define(name.clone(), Value::Object(obj));
//                 Ok(Return::None)
//             }
//             Statement::Var {
//                 name, initializer, ..
//             } => {
//                 let value = self.evaluate_expression(initializer)?;
//                 self.environment.define(name.clone(), value);
//                 Ok(Return::None)
//             }
//             Statement::Expression(expr) => {
//                 self.evaluate_expression(expr)?;
//                 Ok(Return::None)
//             }
//             Statement::While { condition, body } => {
//                 let mut value = self.evaluate_expression(condition)?;
//                 while self.is_truthy(&value) {
//                     for stmt in body {
//                         self.execute_statement(stmt)?;
//                     }
//                     value = self.evaluate_expression(condition)?;
//                 }
//                 Ok(Return::None)
//             }
//             Statement::For {
//                 variable,
//                 iterator,
//                 body,
//             } => {
//                 let iter_value = self.evaluate_expression(iterator)?;
//                 if let Value::Array(items) = iter_value {
//                     for item in items {
//                         self.environment.define(variable.clone(), item);
//                         for stmt in body {
//                             self.execute_statement(stmt)?;
//                         }
//                     }
//                 } else {
//                     return Err("Can only iterate over arrays".to_string());
//                 }
//                 Ok(Return::None)
//             }
//             Statement::If {
//                 condition,
//                 then_branch,
//                 else_branch,
//             } => {
//                 let ret = &self.evaluate_expression(condition)?;
//                 if self.is_truthy(ret) {
//                     for stmt in then_branch {
//                         self.execute_statement(stmt)?;
//                     }
//                 } else if let Some(else_statements) = else_branch {
//                     for stmt in else_statements {
//                         self.execute_statement(stmt)?;
//                     }
//                 }
//                 Ok(Return::None)
//             }
//             Statement::Return(value) => {
//                 if let Some(expr) = value {
//                     let value = self.evaluate_expression(expr)?;
//                     Ok(Return::Value(value))
//                 } else {
//                     Ok(Return::Value(Value::Null))
//                 }
//             }
//             _ => Ok(Return::None),
//         }
//     }

//     fn execute_block(&mut self, statements: &[Statement]) -> Result<Return, String> {
//         for stmt in statements {
//             match self.execute_statement(stmt)? {
//                 Return::Value(value) => return Ok(Return::Value(value)),
//                 Return::None => continue,
//             }
//         }
//         Ok(Return::None)
//     }

//     fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, String> {
//         match expression {
//             Expression::NumberLiteral(n) => Ok(Value::Number(n.parse().unwrap())),
//             Expression::StringLiteral(s) => Ok(Value::String(s[1..s.len() - 1].to_string())), // Remove quotes
//             Expression::BoolLiteral(b) => Ok(Value::Bool(*b)),
//             Expression::Null => Ok(Value::Null),
//             Expression::Identifier(name) => self
//                 .environment
//                 .get(name)
//                 .ok_or_else(|| format!("Undefined variable '{}'", name)),
//             Expression::Binary {
//                 left,
//                 operator,
//                 right,
//             } => {
//                 let left_val = self.evaluate_expression(left)?;
//                 let right_val = self.evaluate_expression(right)?;
//                 self.evaluate_binary_op(operator, &left_val, &right_val)
//             }
//             Expression::Unary { operator, operand } => {
//                 let value = self.evaluate_expression(operand)?;
//                 self.evaluate_unary_op(operator, &value)
//             }
//             Expression::ObjectConstruction {
//                 type_name, fields, ..
//             } => {
//                 let mut obj = Rc::new(RefCell::new(HashMap::new()));

//                 // If type_name provided, create from existing object
//                 if let Some(type_name) = type_name {
//                     if let Some(Value::Object(class_obj)) = self.environment.get(type_name) {
//                         obj = Rc::new(RefCell::new(class_obj.borrow().clone()));
//                     } else {
//                         return Err(format!("Undefined type '{}'", type_name));
//                     }
//                 }

//                 // Evaluate and set fields
//                 let mut params = vec![];
//                 for (name, value) in fields {
//                     let evaluated = self.evaluate_expression(value)?;
//                     params.push(evaluated);
//                 }

//                 // Call init method if it exists
//                 if let Some(Value::Callable(method)) = obj.borrow().get("init") {
//                     method.call(self, params)?;
//                 }

//                 Ok(Value::Object(obj))
//             }
//             Expression::ArrayConstruction { elements } => {
//                 let mut array = Vec::new();
//                 for expr in elements {
//                     array.push(self.evaluate_expression(expr)?);
//                 }
//                 Ok(Value::Array(array))
//             }
//             Expression::Call { callee, arguments } => {
//                 let callee = self.evaluate_expression(callee)?;
//                 let mut evaluated_args = Vec::new();
//                 for arg in arguments {
//                     evaluated_args.push(self.evaluate_expression(arg)?);
//                 }

//                 match callee {
//                     Value::NativeFunction(func) => func(evaluated_args),
//                     Value::Callable(method) => method.call(self, evaluated_args),
//                     _ => Err("Can only call functions and methods.".to_string()),
//                 }
//             }
//             Expression::MemberAccess {
//                 object,
//                 member,
//                 arguments,
//             } => {
//                 let obj = self.evaluate_expression(object)?;
//                 if let Value::Object(obj_ref) = obj {
//                     let obj = obj_ref.borrow();
//                     if let Some(value) = obj.get(member) {
//                         match arguments {
//                             Some(args) => {
//                                 let mut evaluated_args = Vec::new();
//                                 for arg in args {
//                                     evaluated_args.push(self.evaluate_expression(arg)?);
//                                 }
//                                 self.call_method(value, evaluated_args)
//                             }
//                             None => Ok(value.clone()),
//                         }
//                     } else {
//                         Err(format!("Undefined property '{}'", member))
//                     }
//                 } else {
//                     Err("Can only access properties on objects".to_string())
//                 }
//             }
//             Expression::Assignment { target, value } => {
//                 let value = self.evaluate_expression(value)?;
//                 match &**target {
//                     Expression::Identifier(name) => {
//                         self.environment.assign(name.clone(), value.clone())?;
//                         Ok(value)
//                     }
//                     Expression::MemberAccess {
//                         object,
//                         member,
//                         arguments: None,
//                     } => {
//                         if let Value::Object(obj_ref) = self.evaluate_expression(object)? {
//                             obj_ref.borrow_mut().insert(member.clone(), value.clone());
//                             Ok(value)
//                         } else {
//                             Err("Can only assign to object properties".to_string())
//                         }
//                     }
//                     _ => Err("Invalid assignment target".to_string()),
//                 }
//             }
//             Expression::ArrayAccess { array, index } => {
//                 let array_val = self.evaluate_expression(array)?;
//                 let index_val = self.evaluate_expression(index)?;

//                 match (array_val, index_val) {
//                     (Value::Array(arr), Value::Number(i)) => {
//                         let idx = i as usize;
//                         if idx < arr.len() {
//                             Ok(arr[idx].clone())
//                         } else {
//                             Err(format!("Index {} out of bounds", idx))
//                         }
//                     }
//                     _ => Err("Can only index into arrays with numbers".to_string()),
//                 }
//             }
//             // Add other expression types as needed
//             _ => Err("Unsupported expression type".to_string()),
//         }
//     }

//     // Add method to handle method calls
//     fn call_method(&mut self, method: &Value, arguments: Vec<Value>) -> Result<Value, String> {
//         match method {
//             Value::Callable(callable) => {
//                 if arguments.len() != callable.arity() {
//                     return Err(format!(
//                         "Expected {} arguments but got {}.",
//                         callable.arity(),
//                         arguments.len()
//                     ));
//                 }
//                 callable.call(self, arguments)
//             }
//             _ => Err("Can only call methods and functions.".to_string()),
//         }
//     }

//     fn evaluate_binary_op(
//         &self,
//         op: &BinaryOp,
//         left: &Value,
//         right: &Value,
//     ) -> Result<Value, String> {
//         match op {
//             BinaryOp::Add => match (left, right) {
//                 (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
//                 (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
//                 _ => Err("Invalid operand types for addition".to_string()),
//             },
//             // Add other operators as needed
//             _ => Err("Unsupported operator".to_string()),
//         }
//     }

//     fn evaluate_unary_op(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
//         match op {
//             UnaryOp::Negate => match operand {
//                 Value::Number(n) => Ok(Value::Number(-n)),
//                 _ => Err("Operand must be a number".to_string()),
//             },
//             UnaryOp::Not => match operand {
//                 Value::Bool(b) => Ok(Value::Bool(!b)),
//                 _ => Err("Operand must be a boolean".to_string()),
//             },
//         }
//     }

//     fn is_truthy(&self, value: &Value) -> bool {
//         match value {
//             Value::Bool(b) => *b,
//             _ => false,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{lexer::Lexer, parser::Parser};

//     use super::*;

//     fn interpret(source: &str) {
//         let tokens = Lexer::new(source).tokens();
//         let ast = Parser::new(tokens).parse();
//         Interpreter::new().interpret(ast);
//     }

//     #[test]
//     fn test_number_operations() {
//         let mut interpreter = Interpreter::new();
//         assert_eq!(
//             interpreter.evaluate_expression(&Expression::NumberLiteral("42".to_string())),
//             Ok(Value::Number(42.0))
//         );

//         // Test addition
//         assert_eq!(
//             interpreter.evaluate_binary_op(
//                 &BinaryOp::Add,
//                 &Value::Number(1.0),
//                 &Value::Number(2.0)
//             ),
//             Ok(Value::Number(3.0))
//         );
//     }

//     #[test]
//     fn test_string_operations() {
//         let mut interpreter = Interpreter::new();
//         assert_eq!(
//             interpreter.evaluate_expression(&Expression::StringLiteral("\"hello\"".to_string())),
//             Ok(Value::String("hello".to_string()))
//         );

//         // Test string concatenation
//         assert_eq!(
//             interpreter.evaluate_binary_op(
//                 &BinaryOp::Add,
//                 &Value::String("hello ".to_string()),
//                 &Value::String("world".to_string())
//             ),
//             Ok(Value::String("hello world".to_string()))
//         );
//     }

//     #[test]
//     fn test_boolean_operations() {
//         let mut interpreter = Interpreter::new();
//         assert_eq!(
//             interpreter.evaluate_expression(&Expression::BoolLiteral(true)),
//             Ok(Value::Bool(true))
//         );

//         // Test boolean negation
//         assert_eq!(
//             interpreter.evaluate_unary_op(&UnaryOp::Not, &Value::Bool(true)),
//             Ok(Value::Bool(false))
//         );
//     }

//     #[test]
//     fn test_variable_operations() {
//         let mut interpreter = Interpreter::new();

//         // Test variable definition
//         interpreter
//             .environment
//             .define("x".to_string(), Value::Number(42.0));
//         assert_eq!(
//             interpreter.evaluate_expression(&Expression::Identifier("x".to_string())),
//             Ok(Value::Number(42.0))
//         );

//         // Test variable assignment
//         assert_eq!(
//             interpreter
//                 .environment
//                 .assign("x".to_string(), Value::Number(24.0)),
//             Ok(())
//         );
//         assert_eq!(
//             interpreter.evaluate_expression(&Expression::Identifier("x".to_string())),
//             Ok(Value::Number(24.0))
//         );
//     }

//     #[test]
//     fn test_object_operations() {
//         let mut interpreter = Interpreter::new();

//         // Test object creation
//         let obj = interpreter
//             .evaluate_expression(&Expression::ObjectConstruction {
//                 type_name: None,
//                 fields: vec![
//                     ("x".to_string(), Expression::NumberLiteral("1".to_string())),
//                     ("y".to_string(), Expression::NumberLiteral("2".to_string())),
//                 ],
//             })
//             .unwrap();

//         match obj {
//             Value::Object(obj_ref) => {
//                 let obj = obj_ref.borrow();
//                 assert_eq!(obj.get("x"), Some(&Value::Number(1.0)));
//                 assert_eq!(obj.get("y"), Some(&Value::Number(2.0)));
//             }
//             _ => panic!("Expected object value"),
//         }
//     }

//     #[test]
//     fn test_array_operations() {
//         let mut interpreter = Interpreter::new();

//         // Test array creation
//         let arr = interpreter
//             .evaluate_expression(&Expression::ArrayConstruction {
//                 elements: vec![
//                     Expression::NumberLiteral("1".to_string()),
//                     Expression::NumberLiteral("2".to_string()),
//                 ],
//             })
//             .unwrap();

//         match arr {
//             Value::Array(elements) => {
//                 assert_eq!(elements.len(), 2);
//                 assert_eq!(elements[0], Value::Number(1.0));
//                 assert_eq!(elements[1], Value::Number(2.0));
//             }
//             _ => panic!("Expected array value"),
//         }
//     }

//     #[test]
//     fn test_print_function() {
//         let mut interpreter = Interpreter::new();

//         // Test print function call
//         let result = interpreter.evaluate_expression(&Expression::Call {
//             callee: Box::new(Expression::Identifier("print".to_string())),
//             arguments: vec![Expression::StringLiteral("\"hello\"".to_string())],
//         });

//         assert_eq!(result, Ok(Value::Null));
//     }

//     #[test]
//     fn test_array_loop() {
//         interpret(
//             r#"
//             var sum = 0;
//             var arr = [1, 2, 3];
//             for (var i in arr) {
//                 sum = sum + i;
//             }
//             print(sum);
//             "#,
//         );
//     }

//     #[test]
//     fn test_object_method_call() {
//         interpret(
//             r#"
//             object Counter {
//                 init() {
//                     this.count = 0;
//                 }

//                 increment() {
//                     this.count = this.count + 1;
//                 }

//                 getCount() {
//                     return this.count;
//                 }
//             }

//             var counter = Counter {};
//             print(counter.count);
//             counter.increment();
//             print(counter.getCount());
//             "#,
//         );
//     }
// }
