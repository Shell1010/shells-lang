use crate::lexer::enums::*;
use crate::lexer::Token;


#[derive(Debug)]
pub enum Expression {
    LiteralValue(LiteralValue),
    Identifier(String),
    MathOp(Box<Expression>, MathOperator, Box<Expression>),
    ComparisonOp(Box<Expression>, ComparisonOperator, Box<Expression>),
    LogicalOp(Box<Expression>, LogicalOperator, Box<Expression>),
    BitwiseOp(Box<Expression>, BitwiseOperator, Box<Expression>),

    IfElse(
        Box<Expression>,
        Vec<Statement>,
        Vec<(Box<Expression>, Vec<Statement>)>,
        Option<Vec<Statement>>,
    ),

    Grouping(Box<Expression>),
    Keyword(String),
    FunctionCall(String, Vec<Expression>),
    If(Box<Expression>, Vec<Statement>),
    Assignment(Box<Expression>, Box<Expression>),
    While(Box<Expression>, Vec<Statement>),
    For(String, Box<Expression>, Vec<Statement>),
}

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}



#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Block(Vec<Statement>),
    Comment(String),
    Return(Box<Option<Expression>>),
    FunctionDefinition{
        name: String,
        parameters: Vec<String>, 
        body: Vec<Statement>
    },
}
