use super::{
    array::Array, builtin_function, env::Env, flow::Flow, method::{Method, NativeMethod, TMethod}, object::Object, value::Value
};
use crate::ast::{BinaryOp, Expression, MethodDeclaration, MethodSignature, Statement, UnaryOp};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

    pub fn with_std(&mut self) -> Result<&mut Self, Flow> {
        self.with_std_function("str", builtin_function::str, 1, 1)?;
        self.with_std_function("assert", builtin_function::assert, 2, 2)?;
        self.with_std_function("addr", builtin_function::addr, 1, 1)?;
        self.with_std_function("print", builtin_function::print, 0, 256)?;
        self.with_std_function("length", builtin_function::length, 1, 1)?;
        self.with_std_function("range", builtin_function::range, 2, 3)?;
        Ok(self)
    }

    fn with_std_function(
        &mut self,
        name: &str,
        function: fn(&Vec<Value>) -> Result<Value, Flow>,
        min_arity: usize,
        max_arity: usize,
    ) -> Result<&mut Self, Flow> {
        let native_method = NativeMethod::new(Rc::new(function), min_arity, max_arity);
        self.env
            .borrow_mut()
            .define(name.to_string(), Value::NativeMethod(native_method))?;
        Ok(self)
    }

    pub fn interprete(&mut self, statements: &Vec<Statement>) -> Result<Value, Flow> {
        self.execute_statements(statements)
    }

    pub fn execute_block_with_closure<Closure>(
        &mut self,
        block: &Vec<Statement>,
        closure: Closure,
    ) -> Result<Value, Flow>
    where
        Closure: FnOnce(Rc<RefCell<Env>>) -> Result<Value, Flow>,
    {
        let env = self.begin_block();
        closure(env)?;
        let result = self.execute_statements(block);
        self.end_block()?;
        result
    }

    pub fn execute_block(&mut self, block: &Vec<Statement>) -> Result<Value, Flow> {
        self.begin_block();
        let result = self.execute_statements(block);
        self.end_block()?;
        result
    }

    fn begin_block(&mut self) -> Rc<RefCell<Env>> {
        let env = Rc::new(RefCell::new(Env::new(Some(self.env.clone()))));
        self.env = env.clone();
        env
    }

    fn end_block(&mut self) -> Result<Value, Flow> {
        let parent_env = self.env.borrow().parent.clone();
        if let Some(parent_env) = parent_env {
            self.env = parent_env;
            Ok(Value::Void)
        } else {
            Err(Flow::Error("No parent environment".to_string()))
        }
    }

    pub fn execute_statements(&mut self, block: &Vec<Statement>) -> Result<Value, Flow> {
        for statement in block {
            self.execute_statement(statement)?;
        }
        Ok(Value::Void)
    }

    pub fn execute_statement(&mut self, statement: &Statement) -> Result<Value, Flow> {
        match statement {
            Statement::Object {
                name,
                type_annotation,
                methods,
            } => self.execute_object(name, type_annotation, methods),
            Statement::Trait {
                name,
                type_annotation,
                method_signatures,
            } => self.execute_trait(name, type_annotation, method_signatures),
            Statement::Var {
                name,
                type_annotation,
                initializer,
            } => self.execute_var(name, type_annotation, initializer),
            Statement::While { condition, body } => self.execute_while(condition, body),
            Statement::For {
                variable,
                iterator,
                body,
            } => self.execute_for(variable, iterator, body),
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => self.execute_if(condition, then_branch, else_branch),
            Statement::Break => Err(Flow::Break),
            Statement::Continue => Err(Flow::Continue),
            Statement::Return(expression) => self.execute_return(expression),
            Statement::Expression(expression) => self.evaluate_expression(expression),
        }
    }

    pub fn execute_object(
        &mut self,
        name: &String,
        type_annotation: &Option<Vec<String>>,
        methods: &Vec<MethodDeclaration>,
    ) -> Result<Value, Flow> {
        let object = Object::new();
        for method_decl in methods {
            let name = method_decl.signature.name.clone();
            let method = Method::new(method_decl.clone(), object.clone());
            object.set_method(name, method)?;
        }
        self.object_prototypes.insert(name.clone(), object);

        Ok(Value::Void)
    }

    pub fn execute_trait(
        &mut self,
        name: &String,
        type_annotation: &Option<Vec<String>>,
        method_signatures: &Vec<MethodSignature>,
    ) -> Result<Value, Flow> {
        // Do nothing for now
        Ok(Value::Void)
    }

    pub fn execute_var(
        &mut self,
        name: &String,
        type_annotation: &Option<Vec<String>>,
        initializer: &Expression,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(initializer)?;
        self.env.borrow_mut().define(name.clone(), value)
    }

    pub fn execute_while(
        &mut self,
        condition: &Expression,
        body: &Vec<Statement>,
    ) -> Result<Value, Flow> {
        while self.evaluate_expression(condition)?.as_bool()? {
            let returns = self.execute_block(body);
            if let Err(flow) = returns {
                match flow {
                    Flow::Break => break,
                    Flow::Continue => continue,
                    _ => {}
                }
            }
        }
        Ok(Value::Void)
    }

    pub fn execute_for(
        &mut self,
        variable: &String,
        iterator: &Expression,
        body: &Vec<Statement>,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(iterator)?;
        let iterator = value.as_array()?;
        for value in iterator.elements.borrow().iter() {
            let returns = self.execute_block_with_closure(body, |env| {
                env.borrow_mut().define(variable.clone(), value.clone())?;
                Ok(Value::Void)
            });

            if let Err(flow) = returns {
                match flow {
                    Flow::Break => break,
                    Flow::Continue => continue,
                    _ => {}
                }
            }
        }
        Ok(Value::Void)
    }

    pub fn execute_if(
        &mut self,
        condition: &Expression,
        then_branch: &Vec<Statement>,
        else_branch: &Option<Vec<Statement>>,
    ) -> Result<Value, Flow> {
        if self.evaluate_expression(condition)?.as_bool()? {
            self.execute_block(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute_block(else_branch)?;
        }
        Ok(Value::Void)
    }

    pub fn execute_return(&mut self, expression: &Option<Expression>) -> Result<Value, Flow> {
        match expression {
            Some(expression) => Err(Flow::Return(self.evaluate_expression(expression)?)),
            None => Ok(Value::Void),
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
            Expression::Identifier(name) => self.evaluate_identifier(name),
            Expression::NumberLiteral(n) => Ok(Value::Number(n.parse().unwrap())),
            Expression::StringLiteral(s) => Ok(Value::String(s[1..s.len() - 1].to_string())), // Remove quotes
            Expression::BoolLiteral(b) => Ok(Value::Bool(*b)),
            Expression::Null => Ok(Value::Null),
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
            Value::Method(method) => method.call(self, &args),
            Value::NativeMethod(native_method) => native_method.call(self, &args),
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
        let value = self.evaluate_expression(object)?;
        let object = value.as_object()?;
        let method = object.get_method(member)?;
        let mut args = Vec::new();
        for arg in arguments {
            args.push(self.evaluate_expression(arg)?);
        }
        method.call(self, &args)
    }

    fn evaluate_field_access(
        &mut self,
        object: &Expression,
        member: &String,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(object)?;
        let object = value.as_object()?;
        object.get_value(member)
    }

    fn evaluate_array_access(
        &mut self,
        array: &Expression,
        index: &Expression,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(array)?;
        let array = value.as_array()?;
        let index = self.evaluate_expression(index)?.as_number()?;
        array.get_value(index as i32)
    }

    fn evaluate_assignment(
        &mut self,
        target: &Expression,
        value: &Expression,
    ) -> Result<Value, Flow> {
        let value = self.evaluate_expression(value)?;

        if value.is_void() {
            return Err(Flow::Error("Cannot assign void".to_string()));
        }

        match target {
            Expression::Identifier(name) => {
                self.env.borrow_mut().set(name.clone(), value.clone())?;
                Ok(Value::Void)
            }
            Expression::FieldAccess { object, member } => {
                let object_value = self.evaluate_expression(object)?;
                let object = object_value.as_object()?;
                object.set_value(member.clone(), value.clone())?;
                Ok(Value::Void)
            }
            Expression::ArrayAccess { array, index } => {
                let array_value = self.evaluate_expression(array)?;
                let array = array_value.as_array()?;
                let index = self.evaluate_expression(index)?.as_number()?;
                array.set_value(index as i32, value)?;
                Ok(Value::Void)
            }
            _ => Err(Flow::Error("Invalid assignment target".to_string())),
        }
    }

    fn evaluate_object_construction(
        &mut self,
        type_name: &Option<String>,
        fields: &HashMap<String, Expression>,
    ) -> Result<Value, Flow> {
        if let Some(type_name) = type_name {
            return match self.object_prototypes.get(type_name) {
                Some(object) => {
                    let object = object.instantiate();
                    let mut init_args = Vec::new();

                    if let Ok(init_method) = object.get_method("init") {
                        let init_method_params = &init_method.declaration.signature.params;

                        if fields.len() != init_method_params.len() {
                            return Err(Flow::Error(format!(
                                "Invalid number of arguments for init method: expected {}, got {}",
                                init_method_params.len(),
                                fields.len()
                            )));
                        }

                        for param in init_method_params {
                            let name = &param.name;
                            if let Some(field) = fields.get(name) {
                                let value = self.evaluate_expression(field)?;
                                init_args.push(value);
                            } else {
                                return Err(Flow::Error(format!(
                                    "Missing argument {} for init method",
                                    name
                                )));
                            }
                        }

                        init_method.call(self, &init_args)?;
                    } else {
                        if fields.len() > 0 {
                            return Err(Flow::Error(format!(
                                "Unexpected arguments for object of type {}",
                                type_name
                            )));
                        }
                    }

                    for (name, value) in fields {
                        let value = self.evaluate_expression(value)?;
                        object.set_value(name.clone(), value)?;
                    }

                    Ok(Value::Object(object))
                }
                None => Err(Flow::Error(format!("Type {} not defined", type_name))),
            };
        } else {
            let object = Object::new();
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

        Ok(Value::Array(Array::new(array_elements)))
    }

    fn evaluate_identifier(&mut self, name: &String) -> Result<Value, Flow> {
        self.env.borrow().get(name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;

    fn eval(source: &str) {
        let tokens = Lexer::new(source).lex();
        let statements = Parser::new(tokens).parse();
        let result = Interpreter::new()
            .with_std()
            .unwrap()
            .interprete(&statements);

        if let Err(flow) = &result {
            if let Flow::Error(msg) = flow {
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn test_object_field() {
        eval(
            r#"
            object Point {
                init(x, y) { }
            }

            var point = Point {
                x = 100,
                y = 12,
            };

            assert(point.x, 100);
            assert(point.y, 12);
        "#,
        );
    }

    #[test]
    fn test_method_return() {
        eval(
            r#"
            object Math {
                add(x, y) {
                    return x + y;
                }
            }

            var math = Math {};
            print(math.add(100, 200));
            assert(math.add(10, 2), 12);
            "#,
        );
    }

    #[test]
    fn test_break_continue() {
        eval(
            r#"
            var i = 0;
            while (i < 10) {
                if (i == 5) {
                    break;
                }
                i = i + 1;
            }
            assert(i, 5);


            var sum = 0;
            for (var a in range(0, 10)) {
                if (a == 5) {
                    continue;
                }
                sum = sum + a;
            }
            assert(sum, 40);
            "#,
        );
    }

    #[test]
    fn test_game_loop() {
        eval(
            r#"
            object RenderContext {
              init() { }
            }

            trait Renderable {
              render(context: RenderContext): void;
            }

            trait Updatable {
              update(dt: number);
            }

            object Text : Renderable + Updatable {
              init(text) { }

              update(dt: number) { }

              render(context) {
                // Rendering code ...
                print("render text");
              }
            }

            object Circle : Renderable {
              init(position, radius) { }

              render(context) {
                // Rendering code ...
                print("render circle");
              }
            }

            var context = RenderContext {};
            var renderables: Renderable = [
              Text {
                text = "Hello",
              },
              Circle {
                position = {
                  x = 0,
                  y = 0,
                },
                radius = 5,
              }
            ];

            var frame = 0;
            while (frame < 3) {
              for (var renderable in renderables) {
                renderable.render(context);
              }
              frame = frame + 1;
            }

            print("Context addr:", addr(context));

            assert(frame, 3);
            "#,
        );
    }
}
