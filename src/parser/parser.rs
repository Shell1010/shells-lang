use super::enums::*;
use crate::lexer::enums::*;
use crate::lexer::Token;

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// Converting our tokens into an AST basically
// Our tokens are turned into actual syntax
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

        while self.peek().is_some() && self.peek()? != &Token::EndOfInput {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            } else {
            }
        }

        Some(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek()? {
            Token::Keyword(keyword) => match keyword.as_str() {
                "if" => return self.parse_if_statement(),
                "print" => return self.parse_print_statement(),
                "while" => return self.parse_while_statement(),
                "for" => return self.parse_for_statement(),
                "return" => return self.parse_return_statement(),
                "fn" => return self.parse_function_definition(),
                _ => None,
            },
            Token::Identifier(_) => self.parse_assignment_or_expression(),
            Token::LeftBrace => self.parse_block(),
            Token::Comment(_) => self.parse_comment(),
            _ => self.parse_expression().map(Statement::Expression),
        }
    }

    fn parse_comment(&mut self) -> Option<Statement> {
        if let Some(Token::Comment(val)) = self.peek() {
            let other_val = val.to_owned();
            self.advance();
            Some(Statement::Comment(other_val))
        } else {
            None
        }
    }

    fn parse_function_definition(&mut self) -> Option<Statement> {
        self.advance();

        let func_name = if let Token::Identifier(name) = self.advance()? {
            name.clone()
        } else {
            return None;
        };

        if self.advance()? != &Token::LeftParen {
            return None;
        }

        let mut parameters = Vec::new();
        while let Some(Token::Identifier(param)) = self.peek() {
            parameters.push(param.clone());
            self.advance();

            if let Some(Token::Comma) = self.peek() {
                self.advance();
            } else {
                break;
            }
        }

        if self.advance()? != &Token::RightParen {
            return None;
        }

        let body = match self.parse_block()? {
            Statement::Block(statements) => statements,
            _ => return None,
        };

        Some(Statement::FunctionDefinition {
            name: func_name,
            parameters,
            body,
        })
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

    fn parse_print_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "print" {
                self.advance();
                let condition: Expression = self.parse_expression()?;

                return Some(Statement::Expression(Expression::FunctionCall(
                    "print".into(),
                    vec![condition],
                )));
            }
        }
        None
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "if" {
                self.advance();

                let condition = self.parse_expression()?;
                let body = match self.parse_block()? {
                    Statement::Block(stmts) => stmts,
                    _ => return None,
                };

                let mut elif_branches = Vec::new();
                while let Some(Token::Keyword(ref keyword)) = self.peek() {
                    if keyword == "elif" {
                        self.advance();

                        let elif_condition = self.parse_expression()?;
                        let elif_body = match self.parse_block()? {
                            Statement::Block(stmts) => stmts,
                            _ => return None,
                        };

                        elif_branches.push((Box::new(elif_condition), elif_body));
                    } else {
                        break;
                    }
                }

                let else_body = if let Some(Token::Keyword(ref keyword)) = self.peek() {
                    if keyword == "else" {
                        self.advance();
                        match self.parse_block()? {
                            Statement::Block(stmts) => Some(stmts),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                return Some(Statement::Expression(Expression::IfElse(
                    Box::new(condition),
                    body,
                    elif_branches,
                    else_body,
                )));
            }
        }
        None
    }

    fn parse_assignment_or_expression(&mut self) -> Option<Statement> {
        // Parse the first identifier (function name or variable name)
        let name = if let Token::Identifier(name) = self.advance()? {
            name.clone()
        } else {
            return None; // Expect an identifier
        };

        let mut parameters = Vec::new();

        // Parse potential parameters
        while let Some(Token::Identifier(param)) = self.peek() {
            parameters.push(param.clone());
            self.advance(); // Consume the parameter

            if let Some(Token::OperatorToken(Operator::WalrusEqual)) = self.peek() {
                self.advance(); // Consume `:=`

                // Parse the function body (single expression)
                let body = self.parse_expression()?;

                return Some(Statement::FunctionDefinition {
                    name,
                    parameters,
                    body: vec![Statement::Return(Box::new(Some(body)))],
                });
            }
        }

        // If not a function shortcut, fall back to normal assignment or expression parsing
        if let Some(Token::OperatorToken(Operator::Equals)) = self.peek() {
            self.advance(); // Consume `=`

            let expr = self.parse_expression()?;
            Some(Statement::Expression(Expression::Assignment(
                Box::new(Expression::Identifier(name)),
                Box::new(expr),
            )))
        } else {
            let expr = self.parse_expression()?;
            Some(Statement::Expression(expr))
        }
    }

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_logical()
    }

    fn parse_logical(&mut self) -> Option<Expression> {
        let mut left = self.parse_bitwise()?;

        while let Some(token) = self.peek() {
            match token {
                Token::LogicalOperatorToken(op @ LogicalOperator::And)
                | Token::LogicalOperatorToken(op @ LogicalOperator::Or) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_bitwise()?;
                    left = Expression::LogicalOp(Box::new(left), other_op, Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_bitwise(&mut self) -> Option<Expression> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            match token {
                Token::BitwiseOperatorToken(op @ BitwiseOperator::RightShift)
                | Token::BitwiseOperatorToken(op @ BitwiseOperator::LeftShift)
                | Token::BitwiseOperatorToken(op @ BitwiseOperator::And)
                | Token::BitwiseOperatorToken(op @ BitwiseOperator::Or)
                | Token::BitwiseOperatorToken(op @ BitwiseOperator::Xor) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_comparison()?;
                    left = Expression::BitwiseOp(Box::new(left), other_op, Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "return" {
                self.advance(); // consume 'return'
                let expr = Box::new(self.parse_expression()); // parse optional expression
                return Some(Statement::Return(expr));
            }
        }
        None
    }

    fn parse_while_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "while" {
                self.advance(); // consume 'while'
                let condition = self.parse_expression()?; // parse condition
                let body = match self.parse_block()? {
                    Statement::Block(stmts) => stmts,
                    _ => return None,
                };
                return Some(Statement::Expression(Expression::While(
                    Box::new(condition),
                    body,
                )));
            }
        }
        None
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        if let Some(Token::Keyword(ref keyword)) = self.peek() {
            if keyword == "for" {
                self.advance(); // consume 'for'
                println!("here");
                let var = if let Token::Identifier(name) = self.advance()? {
                    name.clone()
                } else {
                    return None;
                };

                println!("here");
                if self.advance()? != &Token::Keyword("in".to_string()) {
                    return None;
                }

                println!("here");
                let iterable = self.parse_expression()?; // parse iterable expression

                println!("here");
                let body = match self.parse_block()? {
                    Statement::Block(stmts) => stmts,
                    _ => return None,
                };

                println!("here");
                return Some(Statement::Expression(Expression::For(
                    var,
                    Box::new(iterable),
                    body,
                )));
            }
        }
        None
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut left = self.parse_math()?;

        if let Some(token) = self.peek() {
            match token {
                Token::ComparisonOperatorToken(op @ ComparisonOperator::GreaterThan)
                | Token::ComparisonOperatorToken(op @ ComparisonOperator::LessThan)
                | Token::ComparisonOperatorToken(op @ ComparisonOperator::Equals)
                | Token::ComparisonOperatorToken(op @ ComparisonOperator::NotEquals)
                | Token::ComparisonOperatorToken(op @ ComparisonOperator::GreaterThanEq)
                | Token::ComparisonOperatorToken(op @ ComparisonOperator::LessThanEq) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_math()?;
                    left = Expression::ComparisonOp(Box::new(left), other_op, Box::new(right));
                }
                _ => {}
            }
        }
        Some(left)
    }

    fn parse_math(&mut self) -> Option<Expression> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            match token {
                Token::MathOperatorToken(op @ MathOperator::Add)
                | Token::MathOperatorToken(op @ MathOperator::Subtract)
                | Token::MathOperatorToken(op @ MathOperator::Multiply)
                | Token::MathOperatorToken(op @ MathOperator::Divide)
                | Token::MathOperatorToken(op @ MathOperator::Modulus) => {
                    let other_op = op.to_owned();
                    self.advance(); // consume the operator
                    let right = self.parse_factor()?;
                    left = Expression::MathOp(Box::new(left), other_op, Box::new(right));
                }
                _ => break,
            }
        }
        Some(left)
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        let token = self.advance()?;
        match token {
            Token::Number(n) => Some(Expression::LiteralValue(LiteralValue::Number(*n))),
            Token::Identifier(name) => {
                let other_name = name.clone();
                if let Some(Token::LeftParen) = self.peek() {
                    self.advance(); // consume '('
                    let mut args = Vec::new();

                    // Parse function call arguments
                    while let Some(arg) = self.parse_expression() {
                        args.push(arg);
                        if let Some(Token::Comma) = self.peek() {
                            self.advance(); // consume ','
                        } else {
                            break;
                        }
                    }

                    if self.advance()? != &Token::RightParen {
                        return None; // expect closing parenthesis
                    }

                    Some(Expression::FunctionCall(other_name, args))
                } else {
                    Some(Expression::Identifier(other_name))
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expression()?;
                if self.advance()? != &Token::RightParen {
                    return None; // expect closing parenthesis
                }
                Some(Expression::Grouping(Box::new(expr)))
            }
            _ => None,
        }
    }
}
