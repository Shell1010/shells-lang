use crate::lexer::Token;
use crate::lexer::enums::*;

#[derive(Debug)]
pub enum Expression {
    LiteralValue(LiteralValue),
    Identifier(String),
    MathOp(Box<Expression>, MathOperator, Box<Expression>),
    ComparisonOp(Box<Expression>, ComparisonOperator, Box<Expression>),
    LogicalOp(Box<Expression>, LogicalOperator, Box<Expression>),
    BitwiseOp(Box<Expression>, BitwiseOperator, Box<Expression>),
    
    Grouping(Box<Expression>),  
    Keyword(String),            
    FunctionCall(String, Vec<Expression>),  // For function calls
    If(Box<Expression>, Vec<Statement>),
    Assignment(Box<Expression>, Box<Expression>),
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
    EndOfFile,       
}
