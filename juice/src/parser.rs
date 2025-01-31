use std::collections::HashMap;

use crate::ast::{
    BinaryOp, Expression, MethodDeclaration, MethodSignature, Parameter, Statement, UnaryOp,
};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn statement(&mut self) -> Option<Statement> {
        match self.peek().token_type {
            TokenType::Object => Some(self.object_declaration()),
            TokenType::Trait => Some(self.trait_declaration()),
            TokenType::Var => Some(self.var_declaration()),
            TokenType::While => Some(self.while_statement()),
            TokenType::For => Some(self.for_statement()),
            TokenType::If => Some(self.if_statement()),
            TokenType::Break => Some(self.break_statement()),
            TokenType::Continue => Some(self.continue_statement()),
            TokenType::Return => Some(self.return_statement()),
            _ => Some(self.expression_statement()),
        }
    }

    fn object_declaration(&mut self) -> Statement {
        self.consume(TokenType::Object, "Expected 'object' keyword");
        let name = self.consume_identifier("Expected object name");
        let type_annotation = self.type_annotation();

        self.consume(TokenType::LeftBrace, "Expected '{' after object name");

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.method_declaration());
        }

        self.consume(TokenType::RightBrace, "Expected '}' after object body");

        Statement::Object {
            name,
            type_annotation,
            methods,
        }
    }

    fn trait_declaration(&mut self) -> Statement {
        self.consume(TokenType::Trait, "Expected 'trait' keyword");
        let name = self.consume_identifier("Expected trait name");
        let type_annotation = self.type_annotation();

        self.consume(TokenType::LeftBrace, "Expected '{' after trait name");

        let mut method_signatures = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            method_signatures.push(self.method_signature());
            self.consume(TokenType::Semicolon, "Expected ';' after method signature");
        }

        self.consume(TokenType::RightBrace, "Expected '}' after trait body");

        Statement::Trait {
            name,
            type_annotation,
            method_signatures,
        }
    }

    fn var_declaration(&mut self) -> Statement {
        self.consume(TokenType::Var, "Expected 'var' keyword");
        let name = self.consume_identifier("Expected variable name");
        let type_annotation = self.type_annotation();

        self.consume(TokenType::Equal, "Expected '=' after variable name");
        let initializer = Box::new(self.expression());

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        );

        Statement::Var {
            name,
            type_annotation,
            initializer,
        }
    }

    fn method_declaration(&mut self) -> MethodDeclaration {
        let signature = self.method_signature();
        let body = self.block();

        MethodDeclaration { signature, body }
    }

    fn method_signature(&mut self) -> MethodSignature {
        let name = self.consume_identifier("Expected method name");

        self.consume(TokenType::LeftParen, "Expected '(' after method name");
        let params = if !self.check(TokenType::RightParen) {
            self.parameter_list()
        } else {
            Vec::new()
        };
        self.consume(TokenType::RightParen, "Expected ')' after parameters");

        let return_type = self.type_annotation();

        MethodSignature {
            name,
            params,
            return_type,
        }
    }

    fn parameter_list(&mut self) -> Vec<Parameter> {
        let mut params = Vec::new();

        loop {
            let name = self.consume_identifier("Expected parameter name");
            let type_annotation = self.type_annotation();

            params.push(Parameter {
                name,
                type_annotation,
            });

            if !self.match_token(TokenType::Comma) {
                break;
            }
        }

        params
    }

    fn type_annotation(&mut self) -> Option<Vec<String>> {
        if !self.match_token(TokenType::Colon) {
            return None;
        }

        let mut types = Vec::new();
        loop {
            types.push(self.type_identifier());
            if !self.match_token(TokenType::Plus) {
                break;
            }
        }
        Some(types)
    }

    fn type_identifier(&mut self) -> String {
        let mut parts = Vec::new();
        parts.push(self.consume_identifier("Expected type name"));

        while self.match_token(TokenType::Dot) {
            parts.push(self.consume_identifier("Expected identifier after '.'"));
        }

        parts.join(".")
    }

    fn while_statement(&mut self) -> Statement {
        self.consume(TokenType::While, "Expected 'while' keyword");
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'");
        let condition = Box::new(self.expression());
        self.consume(TokenType::RightParen, "Expected ')' after condition");
        let body = self.block();

        Statement::While { condition, body }
    }

    fn for_statement(&mut self) -> Statement {
        self.consume(TokenType::For, "Expected 'for' keyword");
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'");
        self.consume(TokenType::Var, "Expected 'var' keyword in for loop");
        let variable = self.consume_identifier("Expected iteration variable name");
        self.consume(TokenType::In, "Expected 'in' keyword");
        let iterator = Box::new(self.expression());
        self.consume(
            TokenType::RightParen,
            "Expected ')' after iteration variable",
        );
        let body = self.block();

        Statement::For {
            variable,
            iterator,
            body,
        }
    }

    fn if_statement(&mut self) -> Statement {
        self.consume(TokenType::If, "Expected 'if' keyword");
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'");
        let condition = Box::new(self.expression());
        self.consume(TokenType::RightParen, "Expected ')' after condition");
        let then_branch = self.block();

        let else_branch = if self.match_token(TokenType::Else) {
            Some(self.block())
        } else {
            None
        };

        Statement::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn break_statement(&mut self) -> Statement {
        self.consume(TokenType::Break, "Expected 'break' keyword");
        self.consume(TokenType::Semicolon, "Expected ';' after break statement");
        Statement::Break
    }

    fn continue_statement(&mut self) -> Statement {
        self.consume(TokenType::Continue, "Expected 'continue' keyword");
        self.consume(TokenType::Semicolon, "Expected ';' after continue statement");
        Statement::Continue
    }

    fn return_statement(&mut self) -> Statement {
        self.consume(TokenType::Return, "Expected 'return' keyword");
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after return statement");

        Statement::Return(value)
    }

    fn block(&mut self) -> Vec<Statement> {
        self.consume(TokenType::LeftBrace, "Expected '{' before block");

        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block");
        statements
    }

    fn expression_statement(&mut self) -> Statement {
        let expr = self.expression();
        self.consume(TokenType::Semicolon, "Expected ';' after expression");
        Statement::Expression(expr)
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        let expr = self.logical_or();

        if self.match_token(TokenType::Equal) {
            let value = Box::new(self.assignment());
            return Expression::Assignment {
                target: Box::new(expr),
                value,
            };
        }

        expr
    }

    fn logical_or(&mut self) -> Expression {
        let mut expr = self.logical_and();

        while self.match_token(TokenType::Or) {
            let right = self.logical_and();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        expr
    }

    fn logical_and(&mut self) -> Expression {
        let mut expr = self.equality();

        while self.match_token(TokenType::And) {
            let right = self.equality();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }

        expr
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        loop {
            let op = if self.match_token(TokenType::EqualEqual) {
                BinaryOp::Equal
            } else if self.match_token(TokenType::BangEqual) {
                BinaryOp::NotEqual
            } else {
                break;
            };

            let right = self.comparison();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        loop {
            let op = if self.match_token(TokenType::Less) {
                BinaryOp::Less
            } else if self.match_token(TokenType::LessEqual) {
                BinaryOp::LessEqual
            } else if self.match_token(TokenType::Greater) {
                BinaryOp::Greater
            } else if self.match_token(TokenType::GreaterEqual) {
                BinaryOp::GreaterEqual
            } else {
                break;
            };

            let right = self.term();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        loop {
            let op = if self.match_token(TokenType::Plus) {
                BinaryOp::Add
            } else if self.match_token(TokenType::Minus) {
                BinaryOp::Subtract
            } else {
                break;
            };

            let right = self.factor();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        loop {
            let op = if self.match_token(TokenType::Star) {
                BinaryOp::Multiply
            } else if self.match_token(TokenType::Slash) {
                BinaryOp::Divide
            } else if self.match_token(TokenType::Percent) {
                BinaryOp::Modulo
            } else {
                break;
            };

            let right = self.unary();
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        let mut operators = Vec::new();
        while self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            operators.push(if self.previous().token_type == TokenType::Bang {
                UnaryOp::Not
            } else {
                UnaryOp::Negate
            });
        }

        let mut expr = self.postfix_expression();

        // Apply unary operators in reverse order
        for op in operators.into_iter().rev() {
            expr = Expression::Unary {
                operator: op,
                operand: Box::new(expr),
            };
        }

        expr
    }

    fn postfix_expression(&mut self) -> Expression {
        let mut expr = self.primary();

        if self.match_token(TokenType::LeftParen) {
            let arguments = if !self.check(TokenType::RightParen) {
                self.argument_list()
            } else {
                Vec::new()
            };
            self.consume(TokenType::RightParen, "Expected ')' after arguments");
            expr = Expression::Call {
                callee: Box::new(expr),
                arguments,
            };
        } else if self.match_token(TokenType::Dot) {
            let member = self.consume_identifier("Expected property name after '.'");
            if self.check(TokenType::LeftParen) {
                // Method call
                self.advance();
                let arguments = self.argument_list();
                self.consume(TokenType::RightParen, "Expected ')' after method arguments");
                expr = Expression::MethodAccess {
                    object: Box::new(expr),
                    member,
                    arguments
                };
            } else {
                // Property access
                expr = Expression::FieldAccess {
                    object: Box::new(expr),
                    member,
                };
            }
        } else if self.match_token(TokenType::LeftBracket) {
            let index = Box::new(self.expression());
            self.consume(TokenType::RightBracket, "Expected ']' after array index");
            expr = Expression::ArrayAccess {
                array: Box::new(expr),
                index,
            };
        }

        expr
    }

    fn primary(&mut self) -> Expression {
        if self.match_token_sequence_no_advance(&[TokenType::Identifier, TokenType::LeftBrace]) {
            return self.object_construction();
        } else if self.match_token(TokenType::LeftBrace) {
            return self.anonymous_object_construction();
        } else if self.match_token(TokenType::LeftBracket) {
            return self.array_construction();
        } else if self.match_token(TokenType::LeftParen) {
            return self.group();
        }

        let token = self.advance();
        match token.token_type {
            TokenType::Identifier => Expression::Identifier(token.lexeme),
            TokenType::StringLiteral => Expression::StringLiteral(token.lexeme),
            TokenType::NumberLiteral => Expression::NumberLiteral(token.lexeme),
            TokenType::True => Expression::BoolLiteral(true),
            TokenType::False => Expression::BoolLiteral(false),
            TokenType::Null => Expression::Null,
            _ => panic!("Expected expression, but found {:?}", token),
        }
    }

    fn group(&mut self) -> Expression {
        let expr = self.expression();
        self.consume(TokenType::RightParen, "Expected ')' after expression");
        expr
    }

    fn object_construction(&mut self) -> Expression {
        let type_name = Some(self.type_identifier());

        self.consume(TokenType::LeftBrace, "Expected '{' after object type");

        let mut fields = HashMap::new();
        while !self.check(TokenType::RightBrace) {
            let name = self.consume_identifier("Expected field name");

            self.consume(TokenType::Equal, "Expected '=' after field name");
            let value = self.expression();
            fields.insert(name, value);

            // Allow optional comma, including trailing comma
            if !self.match_token(TokenType::Comma) {
                // If no comma, must be end of fields
                break;
            }

            // After comma, check if we've reached the end (handles trailing comma)
            if self.check(TokenType::RightBrace) {
                break;
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after object fields");

        Expression::ObjectConstruction { type_name, fields }
    }

    fn anonymous_object_construction(&mut self) -> Expression {
        let mut fields = HashMap::new();
        while !self.check(TokenType::RightBrace) {
            let name = self.consume_identifier("Expected field name");

            self.consume(TokenType::Equal, "Expected '=' after field name");
            let value = self.expression();
            fields.insert(name, value);

            // Allow optional comma, including trailing comma
            if !self.match_token(TokenType::Comma) {
                break;
            }

            // After comma, check if we've reached the end (handles trailing comma)
            if self.check(TokenType::RightBrace) {
                break;
            }
        }

        self.consume(TokenType::RightBrace, "Expected '}' after object fields");

        Expression::ObjectConstruction {
            type_name: None,
            fields,
        }
    }

    fn array_construction(&mut self) -> Expression {
        let mut elements = Vec::new();

        if !self.check(TokenType::RightBracket) {
            loop {
                elements.push(self.expression());

                // Allow optional comma, including trailing comma
                if !self.match_token(TokenType::Comma) {
                    break;
                }

                // After comma, check if we've reached the end (handles trailing comma)
                if self.check(TokenType::RightBracket) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightBracket, "Expected ']' after array elements");

        Expression::ArrayConstruction { elements }
    }

    fn argument_list(&mut self) -> Vec<Expression> {
        let mut arguments = Vec::new();

        loop {
            arguments.push(self.expression());
            if !self.match_token(TokenType::Comma) {
                break;
            }
        }

        arguments
    }

    // Helper methods
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_token_sequence_no_advance(&mut self, token_types: &[TokenType]) -> bool {
        let current = self.current;
        for token_type in token_types {
            if !self.check(token_type.clone()) {
                self.current = current;
                return false;
            }
            self.advance();
        }
        self.current = current;
        true
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            self.advance()
        } else {
            panic!("{}", message);
        }
    }

    fn consume_identifier(&mut self, message: &str) -> String {
        let token = self.consume(TokenType::Identifier, message);
        token.lexeme
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(input: &str) -> Vec<Statement> {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        ast
    }

    #[test]
    fn test_var_declaration() {
        let input = "var x = 42;";
        let statements = parse(input);

        let expected = vec![Statement::Var {
            name: "x".to_string(),
            type_annotation: None,
            initializer: Box::new(Expression::NumberLiteral("42".to_string())),
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_var_declaration_with_type_annotation() {
        let input = "var x: number = 42;";
        let statements = parse(input);

        let expected = vec![Statement::Var {
            name: "x".to_string(),
            type_annotation: Some(vec!["number".to_string()]),
            initializer: Box::new(Expression::NumberLiteral("42".to_string())),
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_var_declaration_with_object_creation() {
        let input = "var point = Point { x = 1, y = 2 };";
        let statements = parse(input);

        let expected = vec![Statement::Var {
            name: "point".to_string(),
            type_annotation: None,
            initializer: Box::new(Expression::ObjectConstruction {
                type_name: Some("Point".to_string()),
                fields: HashMap::from([
                    ("x".to_string(), Expression::NumberLiteral("1".to_string())),
                    ("y".to_string(), Expression::NumberLiteral("2".to_string())),
                ]),
            }),
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_object_declaration() {
        let input = "object Point { x() { return 1; } }";
        let statements = parse(input);

        let expected = vec![Statement::Object {
            name: "Point".to_string(),
            type_annotation: None,
            methods: vec![MethodDeclaration {
                signature: MethodSignature {
                    name: "x".to_string(),
                    params: vec![],
                    return_type: None,
                },
                body: vec![Statement::Return(Some(Expression::NumberLiteral(
                    "1".to_string(),
                )))],
            }],
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_if_statement() {
        let input = "if (x == 1) { var y = 2; }";
        let statements = parse(input);

        let expected = vec![Statement::If {
            condition: Box::new(Expression::Binary {
                left: Box::new(Expression::Identifier("x".to_string())),
                operator: BinaryOp::Equal,
                right: Box::new(Expression::NumberLiteral("1".to_string())),
            }),
            then_branch: vec![Statement::Var {
                name: "y".to_string(),
                type_annotation: None,
                initializer: Box::new(Expression::NumberLiteral("2".to_string())),
            }],
            else_branch: None,
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_complex_expression() {
        let input = "var result = (1 + 2) * 3;";
        let statements = parse(input);

        let expected = vec![Statement::Var {
            name: "result".to_string(),
            type_annotation: None,
            initializer: Box::new(Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::NumberLiteral("1".to_string())),
                    operator: BinaryOp::Add,
                    right: Box::new(Expression::NumberLiteral("2".to_string())),
                }),
                operator: BinaryOp::Multiply,
                right: Box::new(Expression::NumberLiteral("3".to_string())),
            }),
        }];

        assert_eq!(statements, expected);
    }

    #[test]
    fn test_game_program() {
        let input = r#"
            object RenderContext {
                init() {
                    // Initialize context
                }

                deinit() {
                    // Release resources
                }
            }

            trait Renderable {
                render(context: RenderContext): void;
            }

            trait Updatable {
                update(dt: number);
            }

            object Text : Renderable + Updatable {
                init(text) {
                    this.text = text;
                }

                render(context) {
                    // Render text
                }
            }

            object Circle : Renderable {
                init(position, radius) {
                    this.position = position;
                    this.radius = radius;
                }

                render(context) {
                    // Render circle
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

            while (true) {
                for (var renderable in renderables) {
                    renderable.render(context);
                }
            }
        "#;

        let statements = parse(input);

        assert_eq!(statements.len(), 8); // RenderContext, 2 traits, 2 objects, 2 variable declarations, 1 while loop

        // Verify RenderContext object
        match &statements[0] {
            Statement::Object { name, methods, .. } => {
                assert_eq!(name, "RenderContext");
                assert_eq!(methods.len(), 2); // init and deinit methods
            }
            _ => panic!("Expected RenderContext object declaration"),
        }

        // Verify Renderable trait
        match &statements[1] {
            Statement::Trait {
                name,
                method_signatures,
                ..
            } => {
                assert_eq!(name, "Renderable");
                assert_eq!(method_signatures.len(), 1); // render method
            }
            _ => panic!("Expected Renderable trait declaration"),
        }

        // Verify Updatable trait
        match &statements[2] {
            Statement::Trait {
                name,
                method_signatures,
                ..
            } => {
                assert_eq!(name, "Updatable");
                assert_eq!(method_signatures.len(), 1); // update method
            }
            _ => panic!("Expected Updatable trait declaration"),
        }

        // Verify Text object inherits both traits
        match &statements[3] {
            Statement::Object {
                name,
                type_annotation,
                methods,
            } => {
                assert_eq!(name, "Text");
                let traits = type_annotation.as_ref().unwrap();
                assert_eq!(traits.len(), 2);
                assert_eq!(traits[0], "Renderable");
                assert_eq!(traits[1], "Updatable");
            }
            _ => panic!("Expected Text object declaration"),
        }
    }
}
