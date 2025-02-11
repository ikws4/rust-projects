use crate::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    line: usize,
    column: usize,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            line: 1,
            column: 1,
            position: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }

        let c = self.input[self.position];
        self.position += 1;
        self.column += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }

        Some(c)
    }

    fn peek(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut lexeme = String::with_capacity(128);
        lexeme.push(first_char);

        while let Some(c) = self.peek() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            lexeme.push(self.advance().unwrap());
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

        let lexeme_len = lexeme.len();
        Token::new(token_type, &lexeme, self.line, self.column - lexeme_len)
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut lexeme = String::new();
        lexeme.push(first_char);

        let mut has_decimal = false;
        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                lexeme.push(self.advance().unwrap());
            } else if c == '.' && !has_decimal {
                has_decimal = true;
                lexeme.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        let lexeme_len = lexeme.len();
        Token::new(
            TokenType::NumberLiteral,
            &lexeme,
            self.line,
            self.column - lexeme_len,
        )
    }

    fn read_string(&mut self) -> Token {
        let start_column = self.column;
        let mut lexeme = String::new();
        lexeme.push('"');

        while let Some(c) = self.advance() {
            lexeme.push(c);
            if c == '"' {
                break;
            }
        }

        Token::new(TokenType::StringLiteral, &lexeme, self.line, start_column)
    }

    fn read_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.advance() {
            #[cfg_attr(rustfmt, rustfmt_skip)]
            match c {
                // Single-character tokens
                '(' => Token::new(TokenType::LeftParen, "(", self.line, self.column - 1),
                ')' => Token::new(TokenType::RightParen, ")", self.line, self.column - 1),
                '{' => Token::new(TokenType::LeftBrace, "{", self.line, self.column - 1),
                '}' => Token::new(TokenType::RightBrace, "}", self.line, self.column - 1),
                '[' => Token::new(TokenType::LeftBracket, "[", self.line, self.column - 1),
                ']' => Token::new(TokenType::RightBracket, "]", self.line, self.column - 1),
                '.' => Token::new(TokenType::Dot, ".", self.line, self.column - 1),
                ',' => Token::new(TokenType::Comma, ",", self.line, self.column - 1),
                ':' => Token::new(TokenType::Colon, ":", self.line, self.column - 1),
                ';' => Token::new(TokenType::Semicolon, ";", self.line, self.column - 1),
                '+' => Token::new(TokenType::Plus, "+", self.line, self.column - 1),
                '-' => Token::new(TokenType::Minus, "-", self.line, self.column - 1),
                '*' => Token::new(TokenType::Star, "*", self.line, self.column - 1),
                '%' => Token::new(TokenType::Percent, "%", self.line, self.column - 1),

                // Two-character tokens
                '=' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::new(TokenType::EqualEqual, "==", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Equal, "=", self.line, self.column - 1)
                    }
                }
                '!' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::new(TokenType::BangEqual, "!=", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Bang, "!", self.line, self.column - 1)
                    }
                }
                '>' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::new(TokenType::GreaterEqual, ">=", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Greater, ">", self.line, self.column - 1)
                    }
                }
                '<' => {
                    if let Some('=') = self.peek() {
                        self.advance();
                        Token::new(TokenType::LessEqual, "<=", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Less, "<", self.line, self.column - 1)
                    }
                }
                '&' => {
                    if let Some('&') = self.peek() {
                        self.advance();
                        Token::new(TokenType::And, "&&", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Invalid, "&", self.line, self.column - 1)
                    }
                }
                '|' => {
                    if let Some('|') = self.peek() {
                        self.advance();
                        Token::new(TokenType::Or, "||", self.line, self.column - 2)
                    } else {
                        Token::new(TokenType::Invalid, "|", self.line, self.column - 1)
                    }
                }
                '"' => self.read_string(),
                '/' => {
                    match self.peek() {
                        Some('/') => {
                            self.advance(); // consume second '/'
                            self.read_comment();
                            self.next_token() // skip comment and get next token
                        }
                        _ => Token::new(TokenType::Slash, "/", self.line, self.column - 1),
                    }
                }
                c if c.is_alphabetic() || c == '_' => self.read_identifier(c),
                c if c.is_digit(10) => self.read_number(c),
                _ => Token::new(TokenType::Invalid, &c.to_string(), self.line, self.column - 1),
            }
        } else {
            Token::new(TokenType::Eof, "", self.line, self.column)
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            if token.token_type == TokenType::Eof {
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "var x = 42;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::Var);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Equal);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::Eof);
    }

    #[test]
    fn test_keywords() {
        let input = "object trait while for in if else return";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::Object);
        assert_eq!(lexer.next_token().token_type, TokenType::Trait);
        assert_eq!(lexer.next_token().token_type, TokenType::While);
        assert_eq!(lexer.next_token().token_type, TokenType::For);
        assert_eq!(lexer.next_token().token_type, TokenType::In);
        assert_eq!(lexer.next_token().token_type, TokenType::If);
        assert_eq!(lexer.next_token().token_type, TokenType::Else);
        assert_eq!(lexer.next_token().token_type, TokenType::Return);
    }

    #[test]
    fn test_string_literal() {
        let input = "var name = \"hello world\";";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::Var);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Equal);
        assert_eq!(lexer.next_token().token_type, TokenType::StringLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_numbers_and_decimals() {
        let input = "123 45.67 0.89";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
    }

    #[test]
    fn test_comments() {
        let input = "var x = 5; // this is a comment\nvar y = 10;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::Var);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Equal);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::Var);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::Equal);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
    }

    #[test]
    fn test_operators_and_punctuation() {
        let input = "( ) { } [ ] . , : ; = +";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next_token().token_type, TokenType::RightParen);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBracket);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBracket);
        assert_eq!(lexer.next_token().token_type, TokenType::Dot);
        assert_eq!(lexer.next_token().token_type, TokenType::Comma);
        assert_eq!(lexer.next_token().token_type, TokenType::Colon);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::Equal);
        assert_eq!(lexer.next_token().token_type, TokenType::Plus);
    }

    #[test]
    fn test_control_flow() {
        let input = "if x == 5 { return true; } else { return false; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::If);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::EqualEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::NumberLiteral);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Return);
        assert_eq!(lexer.next_token().token_type, TokenType::True);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Else);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Return);
        assert_eq!(lexer.next_token().token_type, TokenType::False);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_for_loop() {
        let input = "for item in items { print(item); }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::For);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::In);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftBrace);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::LeftParen);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
        assert_eq!(lexer.next_token().token_type, TokenType::RightParen);
        assert_eq!(lexer.next_token().token_type, TokenType::Semicolon);
        assert_eq!(lexer.next_token().token_type, TokenType::RightBrace);
    }

    #[test]
    fn test_lexeme() {
        let input = "var name = \"John\";";
        let mut lexer = Lexer::new(input);

        let var_token = lexer.next_token();
        assert_eq!(var_token.lexeme, "var");

        let name_token = lexer.next_token();
        assert_eq!(name_token.lexeme, "name");

        let equal_token = lexer.next_token();
        assert_eq!(equal_token.lexeme, "=");

        let string_token = lexer.next_token();
        assert_eq!(string_token.lexeme, "\"John\"");

        let semicolon_token = lexer.next_token();
        assert_eq!(semicolon_token.lexeme, ";");
    }

    #[test]
    fn test_operators() {
        let input = "+-*/%==!=><>=<=&&||!";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().token_type, TokenType::Plus);
        assert_eq!(lexer.next_token().token_type, TokenType::Minus);
        assert_eq!(lexer.next_token().token_type, TokenType::Star);
        assert_eq!(lexer.next_token().token_type, TokenType::Slash);
        assert_eq!(lexer.next_token().token_type, TokenType::Percent);
        assert_eq!(lexer.next_token().token_type, TokenType::EqualEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::BangEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::Greater);
        assert_eq!(lexer.next_token().token_type, TokenType::Less);
        assert_eq!(lexer.next_token().token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::LessEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::And);
        assert_eq!(lexer.next_token().token_type, TokenType::Or);
        assert_eq!(lexer.next_token().token_type, TokenType::Bang);
    }
}
