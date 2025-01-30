use super::value::{Array, Flow, Object, TMethod, Value};
use crate::ast::{BinaryOp, Expression, Statement, UnaryOp};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Env {
    pub values: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new(parent: Option<Rc<RefCell<Env>>>) -> Self {
        Self {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if self.values.contains_key(&name) {
            panic!("Variable {} already defined", name);
        }

        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Value {
        if self.values.contains_key(&name) {
            return self.values.get(&name).unwrap().clone();
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        panic!("Variable {} not defined", name);
    }

    pub fn set(&mut self, name: String, value: Value) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            return;
        }

        if let Some(parent) = &mut self.parent {
            parent.borrow_mut().set(name, value);
            return;
        }

        panic!("Variable {} not defined", name);
    }
}

pub struct Interpreter {
    pub env: Rc<RefCell<Env>>,
    pub object_prototypes: HashMap<String, Object>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Env::new(None))),
            object_prototypes: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, block: &Vec<Statement>) -> Result<Value, Flow> {
        for statement in block {
            self.execute_statement(statement)?;
        }
        Ok(Value::Null)
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> Result<Value, Flow> {
        match statement {
            Statement::Object {
                name,
                type_annotation,
                methods,
            } => todo!(),
            Statement::Trait {
                name,
                type_annotation,
                method_signatures,
            } => todo!(),
            Statement::Var {
                name,
                type_annotation,
                initializer,
            } => todo!(),
            Statement::While { condition, body } => todo!(),
            Statement::For {
                variable,
                iterator,
                body,
            } => todo!(),
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
            Statement::Return(expression) => todo!(),
            Statement::Expression(expression) => todo!(),
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression) -> Result<Value, Flow> {
        match expression {
            Expression::Call { callee, arguments } => self.evaluate_call(callee, arguments),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            Expression::Unary { operator, operand } => self.evaluate_unary(operator, operand),
            Expression::MethodAccess {
                object,
                member,
                arguments,
            } => self.evaluate_method_access(object, member, arguments),
            Expression::FieldAccess { object, member } => {
                self.evaluate_field_access(object, member)
            }
            Expression::ArrayAccess { array, index } => self.evaluate_array_access(array, index),
            Expression::Assignment { target, value } => self.evaluate_assignment(target, value),
            Expression::ObjectConstruction { type_name, fields } => {
                self.evaluate_object_construction(type_name, fields)
            }
            Expression::ArrayConstruction { elements } => {
                self.evaluate_array_construction(elements)
            }
            Expression::Identifier(_) => todo!(),
            Expression::StringLiteral(_) => todo!(),
            Expression::NumberLiteral(_) => todo!(),
            Expression::BoolLiteral(_) => todo!(),
            Expression::Null => todo!(),
        }
    }

    fn evaluate_call(
        &mut self,
        callee: &Expression,
        arguments: &Vec<Expression>,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(callee)?;

        let mut args = Vec::new();
        for arg in arguments {
            let value = self.evaluate_expression(arg)?;
            args.push(value);
        }

        return match value {
            Value::Method(method) => method.call(self, args),
            Value::NativeMethod(native_method) => native_method.call(self, args),
            _ => Err(Flow::Error("Can only call methods on objects".to_string())),
        };
    }

    fn evaluate_binary(
        &mut self,
        left: &Expression,
        operator: &BinaryOp,
        right: &Expression,
    ) -> Result<Value, Flow> {
        let left = self.evaluate_expression(left)?;
        let right = &self.evaluate_expression(right)?;

        match operator {
            BinaryOp::Add => left.add(right),
            BinaryOp::Subtract => left.sub(right),
            BinaryOp::Multiply => left.mul(right),
            BinaryOp::Divide => left.div(right),
            BinaryOp::Modulo => left.rem(right),
            BinaryOp::Equal => left.eq(right),
            BinaryOp::NotEqual => left.ne(right),
            BinaryOp::Greater => left.gt(right),
            BinaryOp::GreaterEqual => left.ge(right),
            BinaryOp::Less => left.lt(right),
            BinaryOp::LessEqual => left.le(right),
            BinaryOp::And => left.and(right),
            BinaryOp::Or => left.or(right),
        }
    }

    fn evaluate_unary(&mut self, operator: &UnaryOp, operand: &Expression) -> Result<Value, Flow> {
        let operand = self.evaluate_expression(operand)?;

        match operator {
            UnaryOp::Negate => operand.neg(),
            UnaryOp::Not => operand.not(),
        }
    }

    fn evaluate_method_access(
        &mut self,
        object: &Expression,
        member: &String,
        arguments: &Vec<Expression>,
    ) -> Result<Value, Flow> {
        let object = self.evaluate_expression(object)?.as_object()?;
        let method = object.get_method(member)?;
        let mut args = Vec::new();
        for arg in arguments {
            args.push(self.evaluate_expression(arg)?);
        }
        method.call(self, args)
    }

    fn evaluate_field_access(
        &mut self,
        object: &Expression,
        member: &String,
    ) -> Result<Value, Flow> {
        let object = self.evaluate_expression(object)?.as_object()?;
        object.get_value(member)
    }

    fn evaluate_array_access(
        &mut self,
        array: &Expression,
        index: &Expression,
    ) -> Result<Value, Flow> {
        let array = self.evaluate_expression(array)?.as_array()?;
        let index = self.evaluate_expression(index)?.as_number()?;
        array.get_value(index as i32)
    }

    fn evaluate_assignment(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(value)?;
        match target {
            Expression::Identifier(name) => {
                self.env.borrow_mut().set(name.clone(), value.clone());
                Ok(value)
            }
            Expression::FieldAccess { object, member } => {
                let mut object = self.evaluate_expression(object)?.as_object()?;
                object.set_value(member.clone(), value.clone())?;
                Ok(value)
            }
            Expression::ArrayAccess { array, index } => {
                let mut array = self.evaluate_expression(array)?.as_array()?;
                let index = self.evaluate_expression(index)?.as_number()? as i32;
                array.set_value(index, value.clone())?;
                Ok(value)
            }
            _ => Err(Flow::Error("Invalid assignment target".to_string())),
        }
    }

    fn evaluate_object_construction(
        &mut self,
        type_name: &Option<String>,
        fields: &Vec<(String, Expression)>,
    ) -> Result<Value, Flow> {
        if let Some(type_name) = type_name {
            return match self.object_prototypes.get(type_name) {
                Some(object) => {
                    let mut object = object.instantiate();
                    for (name, value) in fields {
                        let value = self.evaluate_expression(value)?;
                        object.set_value(name.clone(), value)?;
                    }
                    Ok(Value::Object(object))
                }
                None => Err(Flow::Error(format!("Type {} not defined", type_name))),
            };
        } else {
            let mut object = Object::new();
            for (name, value) in fields {
                let value = self.evaluate_expression(value)?;
                object.set_value(name.clone(), value)?;
            }
            Ok(Value::Object(object))
        }
    }

    fn evaluate_array_construction(&mut self, elements: &Vec<Expression>) -> Result<Value, Flow> {
        let mut array_elements = Vec::new();
        for element in elements {
            let value = self.evaluate_expression(element)?;
            array_elements.push(value);
        }

        Ok(Value::Array(Array {
            elements: Rc::new(RefCell::new(array_elements)),
        }))
    }
}
