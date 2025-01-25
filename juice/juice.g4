grammar juice;

program
    : statement* EOF
    ;

statement
    : objectDeclaration
    | traitDeclaration
    | varDeclaration
    | whileStatement
    | forStatement
    | ifStatement
    | returnStatement
    | expressionStatement
    ;

objectDeclaration
    : OBJECT identifier typeAnnotation? '{' methodDeclaration* '}'
    ;

traitDeclaration
    : TRAIT identifier '{' (methodSignature ';')* '}'
    ;

varDeclaration
    : VAR identifier typeAnnotation? '=' expression ';'
    ;

methodDeclaration
    : methodSignature block
    ;

methodSignature
    : identifier '(' paramList? ')' typeAnnotation?
    ;

paramList
    : param (',' param)*
    ;

param
    : identifier typeAnnotation?
    ;

typeAnnotation
    : ':' type ('+' type)*
    ;

type
    : identifier ('.' identifier)*
    ;

whileStatement
    : WHILE '(' expression block
    ;

forStatement
    : FOR identifier IN expression block
    ;

ifStatement
    : IF expression block (ELSE block)?
    ;

returnStatement
    : RETURN expression ';'
    ;

block
    : '{' statement* '}'
    ;

expressionStatement
    : expression ';'
    ;

expression
    : assignmentExpression
    ;

assignmentExpression
    : logicalOrExpression ('=' assignmentExpression)?
    ;

logicalOrExpression
    : logicalAndExpression ('||' logicalAndExpression)*
    ;

logicalAndExpression
    : equalityExpression ('&&' equalityExpression)*
    ;

equalityExpression
    : relationalExpression (('==' | '!=') relationalExpression)*
    ;

relationalExpression
    : additiveExpression (('<' | '>' | '<=' | '>=') additiveExpression)*
    ;

additiveExpression
    : multiplicativeExpression (('+' | '-') multiplicativeExpression)*
    ;

multiplicativeExpression
    : unaryExpression (('*' | '/' | '%') unaryExpression)*
    ;

unaryExpression
    : ('!' | '-')* memberExpr
    ;

memberExpr
    : primary (('.' identifier ('(' argumentList? ')')?) | arrayIndexExpr)*
    ;

arrayIndexExpr
    : '[' expression ']'
    ;

argumentList
    : expression (',' expression)*
    ;

primary
    : objectConstruction
    | arrayConstruction
    | group
    | identifier
    | literal
    ;

group
    : '(' expression ')'
    ;

objectConstruction
    : type? '{' (fieldAssignment (',' fieldAssignment)* ','?)? '}'
    ;

fieldAssignment
    : identifier '=' expression
    ;

arrayConstruction
    : '[' (objectConstruction (',' objectConstruction)* ','?)? ']'
    ;

identifier : IDENTIFIER ;
literal : TRUE | FALSE | NULL | NUMBER_LITERAL | STRING_LITERAL ;

// Keywords
VAR         : 'var';
TRAIT       : 'trait';
OBJECT      : 'object';
WHILE       : 'while';
FOR         : 'for';
IN          : 'in';
TRUE        : 'true';
FALSE       : 'false';
NULL        : 'null';
IF          : 'if';
ELSE        : 'else';
RETURN      : 'return';

// Operators
PLUS        : '+';
MINUS       : '-';
MULTIPLY    : '*';
DIVIDE      : '/';
MODULO      : '%';
EQUALS      : '==';
NOT_EQUALS  : '!=';
GREATER     : '>';
LESS        : '<';
GREATER_EQ  : '>=';
LESS_EQ     : '<=';
AND         : '&&';
OR          : '||';
NOT         : '!';
SEMICOLON   : ';';

IDENTIFIER  : [a-zA-Z_] [a-zA-Z0-9_]* ;
NUMBER_LITERAL : [0-9]+ ('.' [0-9]+)? ;
STRING_LITERAL : '"' ~["\r\n]* '"' ;
WS          : [ \t\r\n]+ -> skip ;
COMMENT     : '//' ~[\r\n]* -> skip ;
