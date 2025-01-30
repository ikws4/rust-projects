#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Var,
    Trait,
    Object,
    While,
    For,
    In,
    True,
    False,
    Null,
    If,
    Else,
    Break,
    Continue,
    Return,

    // Identifiers and literals
    Identifier,
    NumberLiteral,
    StringLiteral,

    // Operators and punctuation
    // Arithmetic operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    // Comparison operators
    Equal,        // =
    EqualEqual,   // ==
    BangEqual,    // !=
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // Logical operators
    And,  // &&
    Or,   // ||
    Bang, // !

    // Delimiters
    Dot,          // .
    Comma,        // ,
    Colon,        // :
    Semicolon,    // ;
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Special tokens
    Eof,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}
