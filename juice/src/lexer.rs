use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.read_token();
            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
    fn read_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.advance() {
            match c {
                // Single-character tokens
                '(' => self.token(TokenType::LeftParen, "("),
                ')' => self.token(TokenType::RightParen, ")"),
                '{' => self.token(TokenType::LeftBrace, "{"),
                '}' => self.token(TokenType::RightBrace, "}"),
                '[' => self.token(TokenType::LeftBracket, "["),
                ']' => self.token(TokenType::RightBracket, "]"),
                '.' => self.token(TokenType::Dot, "."),
                ',' => self.token(TokenType::Comma, ","),
                ':' => self.token(TokenType::Colon, ":"),
                ';' => self.token(TokenType::Semicolon, ";"),
                '+' => self.token(TokenType::Plus, "+"),
                '-' => self.token(TokenType::Minus, "-"),
                '*' => self.token(TokenType::Star, "*"),
                '%' => self.token(TokenType::Percent, "%"),

                // Two-character tokens
                '=' => {
                    if self.match_advance('=') {
                        self.token(TokenType::EqualEqual, "==")
                    } else {
                        self.token(TokenType::Equal, "=")
                    }
                }
                '!' => {
                    if self.match_advance('=') {
                        self.token(TokenType::BangEqual, "!=")
                    } else {
                        self.token(TokenType::Bang, "!")
                    }
                }
                '>' => {
                    if self.match_advance('=') {
                        self.token(TokenType::GreaterEqual, ">=")
                    } else {
                        self.token(TokenType::Greater, ">")
                    }
                }
                '<' => {
                    if self.match_advance('=') {
                        self.token(TokenType::LessEqual, "<=")
                    } else {
                        self.token(TokenType::Less, "<")
                    }
                }
                '&' => {
                    if self.match_advance('&') {
                        self.token(TokenType::And, "&&")
                    } else {
                        self.token(TokenType::Invalid, "&")
                    }
                }
                '|' => {
                    if self.match_advance('|') {
                        self.token(TokenType::Or, "||")
                    } else {
                        self.token(TokenType::Invalid, "|")
                    }
                }
                '"' => self.read_string(),
                '/' => {
                    if self.match_advance('/') {
                        self.read_comments()
                    } else {
                        self.token(TokenType::Slash, "/")
                    }
                }
                c if c.is_digit(10) => self.read_number(c),
                c if c.is_alphabetic() || c == '_' => self.read_identifier(c),
                _ => self.token(TokenType::Invalid, &c.to_string()),
            }
        } else {
            self.token(TokenType::Eof, "")
        }
    }

    fn read_number(&mut self, c: char) -> Token {
        let mut lexeme = String::new();
        lexeme.push(c);

        // integer part
        while let Some(current_char) = self.peek() {
            if current_char.is_digit(10) {
                lexeme.push(current_char);
                self.advance();
            } else {
                break;
            }
        }

        // decimal part
        if self.match_advance('.') {
            lexeme.push('.');
            while let Some(current_char) = self.peek() {
                if current_char.is_digit(10) {
                    lexeme.push(current_char);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.token(TokenType::NumberLiteral, &lexeme)
    }

    fn read_comments(&mut self) -> Token {
        while let Some(current_char) = self.peek() {
            if current_char == '\n' {
                break;
            }
            self.advance();
        }
        self.read_token()
    }

    fn read_string(&mut self) -> Token {
        let mut lexeme = String::new();
        lexeme.push('"');
        while let Some(current_char) = self.peek() {
            if current_char == '"' {
                self.advance();
                break;
            } else {
                lexeme.push(current_char);
                self.advance();
            }
        }
        lexeme.push('"');
        self.token(TokenType::StringLiteral, &lexeme)
    }

    fn read_identifier(&mut self, c: char) -> Token {
        let mut lexeme = String::new();
        lexeme.push(c);

        while let Some(current_char) = self.peek() {
            if current_char.is_alphanumeric() || current_char == '_' {
                lexeme.push(current_char);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match lexeme.as_str() {
            "var" => TokenType::Var,
            "trait" => TokenType::Trait,
            "object" => TokenType::Object,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            _ => TokenType::Identifier,
        };

        self.token(token_type, &lexeme)
    }

    fn token(&self, token_type: TokenType, lexeme: &str) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
            column: self.column - lexeme.len(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        let current_char = self.input[self.position];
        self.position += 1;
        self.column += 1;

        if current_char == '\n' {
            self.line += 1;
            self.column = 1;
        }

        Some(current_char)
    }

    fn peek(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        Some(self.input[self.position])
    }

    fn match_advance(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(current_char) = self.peek() {
            if current_char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "var x = 42;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_token().token_type, TokenType::Var);
        assert_eq!(lexer.read_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.read_token().token_type, TokenType::Equal);
        assert_eq!(lexer.read_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.read_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.read_token().token_type, TokenType::Eof);
    }

    #[test]
    fn test_operators() {
        let input = "+-*/%==!=><>=<=&&||!";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_token().token_type, TokenType::Plus);
        assert_eq!(lexer.read_token().token_type, TokenType::Minus);
        assert_eq!(lexer.read_token().token_type, TokenType::Star);
        assert_eq!(lexer.read_token().token_type, TokenType::Slash);
        assert_eq!(lexer.read_token().token_type, TokenType::Percent);
        assert_eq!(lexer.read_token().token_type, TokenType::EqualEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::BangEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::Greater);
        assert_eq!(lexer.read_token().token_type, TokenType::Less);
        assert_eq!(lexer.read_token().token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::LessEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::And);
        assert_eq!(lexer.read_token().token_type, TokenType::Or);
        assert_eq!(lexer.read_token().token_type, TokenType::Bang);
    }

    #[test]
    fn test_string_literal() {
        let input = "\"Hello, World!\"";
        let mut lexer = Lexer::new(input);
        let token = lexer.read_token();
        assert_eq!(token.token_type, TokenType::StringLiteral);
        assert_eq!(token.lexeme, "\"Hello, World!\"");
    }

    #[test]
    fn test_numbers() {
        let input = "123 45.67";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.read_token();
        assert_eq!(token1.token_type, TokenType::NumberLiteral);
        assert_eq!(token1.lexeme, "123");

        let token2 = lexer.read_token();
        assert_eq!(token2.token_type, TokenType::NumberLiteral);
        assert_eq!(token2.lexeme, "45.67");
    }

    #[test]
    fn test_comments() {
        let input = "// this is a comment\nvar";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.read_token().token_type, TokenType::Var);
    }

    #[test]
    fn test_line_column_tracking() {
        let input = "var\nx = 5";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.read_token();
        assert_eq!(token1.line, 1);
        assert_eq!(token1.column, 1);

        let token2 = lexer.read_token();
        assert_eq!(token2.line, 2);
        assert_eq!(token2.column, 1);
    }

    #[test]
    fn test_keywords() {
        let input = "trait object while for in if else return break continue";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_token().token_type, TokenType::Trait);
        assert_eq!(lexer.read_token().token_type, TokenType::Object);
        assert_eq!(lexer.read_token().token_type, TokenType::While);
        assert_eq!(lexer.read_token().token_type, TokenType::For);
        assert_eq!(lexer.read_token().token_type, TokenType::In);
        assert_eq!(lexer.read_token().token_type, TokenType::If);
        assert_eq!(lexer.read_token().token_type, TokenType::Else);
        assert_eq!(lexer.read_token().token_type, TokenType::Return);
        assert_eq!(lexer.read_token().token_type, TokenType::Break);
        assert_eq!(lexer.read_token().token_type, TokenType::Continue);
    }

    #[test]
    fn test_identifiers() {
        let input = "foo _bar baz123";
        let mut lexer = Lexer::new(input);

        let token1 = lexer.read_token();
        assert_eq!(token1.token_type, TokenType::Identifier);
        assert_eq!(token1.lexeme, "foo");

        let token2 = lexer.read_token();
        assert_eq!(token2.token_type, TokenType::Identifier);
        assert_eq!(token2.lexeme, "_bar");

        let token3 = lexer.read_token();
        assert_eq!(token3.token_type, TokenType::Identifier);
        assert_eq!(token3.lexeme, "baz123");
    }

    #[test]
    fn test_complex_expression() {
        let input = "if (x >= 10 && y <= 20) { return true; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.read_token().token_type, TokenType::If);
        assert_eq!(lexer.read_token().token_type, TokenType::LeftParen);
        assert_eq!(lexer.read_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.read_token().token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.read_token().token_type, TokenType::And);
        assert_eq!(lexer.read_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.read_token().token_type, TokenType::LessEqual);
        assert_eq!(lexer.read_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.read_token().token_type, TokenType::RightParen);
        assert_eq!(lexer.read_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.read_token().token_type, TokenType::Return);
        assert_eq!(lexer.read_token().token_type, TokenType::True);
        assert_eq!(lexer.read_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.read_token().token_type, TokenType::RightBrace);
    }
}
