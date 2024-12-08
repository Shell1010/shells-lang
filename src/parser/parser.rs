use crate::lexer:: Token;
use crate::lexer::enums::*;
use super::enums::*;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    pub fn parse(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();
        
        let mut count = 0;
        while self.peek().is_some() && self.peek()? != &Token::EndOfInput {
            count += 1;
            if let Some(statement) = self.parse_statement() {
                println!("{statement:?}: {count}");
                statements.push(statement);
            } else {
            }
        }
        
        Some(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek()? {
            Token::Keyword(keyword) => 
                match keyword.as_str() {
                    "if" => return self.parse_if_statement(),
                    _ => None
                }
            Token::Identifier(_) => self.parse_assignment_or_expression(),
            Token::LeftBrace => self.parse_block(),
            Token::Comment(ref comment) => {
                Some(Statement::Comment(comment.clone()))
            },
            _ => self.parse_expression().map(Statement::Expression)
        }
    }

    fn parse_block(&mut self) -> Option<Statement> {
        if let Some(Token::LeftBrace) = self.advance() {
            let mut statements = Vec::new();
            
            // Parse statements until we hit a right brace
            while let Some(token) = self.peek() {
                if *token == Token::RightBrace {
                    self.advance(); // consume the right brace
                    break;
                }
                
                if let Some(statement) = self.parse_statement() {
                    statements.push(statement);
                } else {
                    return None;
                }
            }
            
            Some(Statement::Block(statements))
        } else {
            None
        }
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        // Consume 'if' keyword
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "if" {
                self.advance();
                
                // Parse condition
                let condition = self.parse_expression()?;
                
                // Parse block
                let body = match self.parse_block()? {
                    Statement::Block(stmts) => stmts,
                    _ => return None,
                };
                
                return Some(Statement::Expression(Expression::If(
                    Box::new(condition), 
                    body
                )));
            }
        }
        None
    }

    fn parse_assignment_or_expression(&mut self) -> Option<Statement> {
        // Peek at the first token without consuming
        let var = self.peek()?;
        
        if let Token::Identifier(var_name) = var {
            let var_name_other = var_name.to_owned();
            if let Some(next_token) = self.tokens.get(self.current + 1) {
                match next_token {
                    Token::OperatorToken(Operator::Equals) => {
                        self.advance(); // consume identifier
                        self.advance(); // consume '='
                        
                        let expr = self.parse_expression()?;
                        return Some(Statement::Expression(Expression::Assignment(
                            Box::new(Expression::Identifier(var_name_other)),
                            Box::new(expr)
                        )));
                    },
                    _ => {
                        let expr = self.parse_expression()?;
                        return Some(Statement::Expression(expr));
                    }
                }
            }
        }
        
        None
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_logical()
    }

    fn parse_logical(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise()?;

        while let Some(token) = self.peek() {
            match token {
                Token::LogicalOperatorToken(op @ LogicalOperator::And) | 
                Token::LogicalOperatorToken(op @ LogicalOperator::Or) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_bitwise()?;
                    left = Expression::LogicalOp(
                        Box::new(left), 
                        other_op, 
                        Box::new(right)
                    );
                },
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_bitwise(&mut self) -> Option<Expression> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            match token {
                Token::BitwiseOperatorToken(op @ BitwiseOperator::RightShift) | 
                Token::BitwiseOperatorToken(op @ BitwiseOperator::LeftShift) | 
                Token::BitwiseOperatorToken(op @ BitwiseOperator::And) | 
                Token::BitwiseOperatorToken(op @ BitwiseOperator::Or) | 
                Token::BitwiseOperatorToken(op @ BitwiseOperator::Xor) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_comparison()?;
                    left = Expression::BitwiseOp(
                        Box::new(left), 
                        other_op, 
                        Box::new(right)
                    );
                },
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut left = self.parse_math()?;

        if let Some(token) = self.peek() {
            match token {
                Token::ComparisonOperatorToken(op @ ComparisonOperator::GreaterThan) | 
                Token::ComparisonOperatorToken(op @ ComparisonOperator::LessThan) | 
                Token::ComparisonOperatorToken(op @ ComparisonOperator::Equals) | 
                Token::ComparisonOperatorToken(op @ ComparisonOperator::NotEquals) | 
                Token::ComparisonOperatorToken(op @ ComparisonOperator::GreaterThanEq) | 
                Token::ComparisonOperatorToken(op @ ComparisonOperator::LessThanEq) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_math()?;
                    left = Expression::ComparisonOp(
                        Box::new(left), 
                        other_op, 
                        Box::new(right)
                    );
                },
                _ => {}
            }
        }
        Some(left)
    }

    fn parse_math(&mut self) -> Option<Expression> {
        let mut left = self.parse_factor()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::MathOperatorToken(op @ MathOperator::Add) | 
                Token::MathOperatorToken(op @ MathOperator::Subtract) | 
                Token::MathOperatorToken(op @ MathOperator::Multiply) | 
                Token::MathOperatorToken(op @ MathOperator::Divide) | 
                Token::MathOperatorToken(op @ MathOperator::Modulus) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_factor()?;
                    left = Expression::MathOp(
                        Box::new(left), 
                        other_op, 
                        Box::new(right)
                    );
                },
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let token = self.advance()?;
        match token {
            Token::Number(n) => Some(Expression::LiteralValue(LiteralValue::Number(*n))),
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                if self.advance()? != &Token::RightParen {
                    return None;  // expect closing parenthesis
                }
                Some(Expression::Grouping(Box::new(expr)))
            },
            _ => None,
        }
    }
}


