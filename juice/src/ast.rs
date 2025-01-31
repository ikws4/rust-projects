use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Object {
        name: String,
        type_annotation: Option<Vec<String>>,
        methods: Vec<MethodDeclaration>,
    },
    Trait {
        name: String,
        type_annotation: Option<Vec<String>>,
        method_signatures: Vec<MethodSignature>,
    },
    Var {
        name: String,
        type_annotation: Option<Vec<String>>,
        initializer: Box<Expression>,
    },
    While {
        condition: Box<Expression>,
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterator: Box<Expression>,
        body: Vec<Statement>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Break,
    Continue,
    Return(Option<Expression>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodDeclaration {
    pub signature: MethodSignature,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expression>,
    },
    MethodAccess {
        object: Box<Expression>,
        member: String,
    },
    FieldAccess {
        object: Box<Expression>,
        member: String,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
    },
    ObjectConstruction {
        type_name: Option<String>,
        fields: HashMap<String, Expression>,
    },
    ArrayConstruction {
        elements: Vec<Expression>,
    },
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    BoolLiteral(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}
