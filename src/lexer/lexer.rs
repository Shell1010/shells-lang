
use super::enums::{BitwiseOperator, ComparisonOperator, LogicalOperator, MathOperator, Operator};
use std::collections::VecDeque;

// Token: Represents all possible tokens in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),                 
    Number(f64),                        
    Keyword(String),                       
    OperatorToken(Operator),                  
    MathOperatorToken(MathOperator),           
    LogicalOperatorToken(LogicalOperator),       
    BitwiseOperatorToken(BitwiseOperator),       
    ComparisonOperatorToken(ComparisonOperator), 
    LeftBrace,                         
    RightBrace,                       
    LeftParen,                          
    RightParen,     
    Comma,
    Comment(String),                      
    EndOfInput,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnterminatedComment,
}

// Lexer struct: Main state for the lexer
// This should parse all input into tokens
// Essentially the Syntax of the language amirite???
pub struct Lexer {
    input: VecDeque<char>,  
    tokens: Vec<Token>,
    index: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input = input.chars().collect::<VecDeque<_>>();
        Self {
            input,
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.index + 1).copied()
    }

    pub fn tokenize(&mut self) -> Result<&[Token], LexerError> {
        while let Some(ch) = self.input.get(self.index).copied() {

            match ch {
                // Whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.index += 1; // Skip whitespace
                }

                // Block comments
                // /* stuff */
                '/' if self.peek_next() == Some('*') => {
                    self.index += 2; // Skip the '/'
                    self.consume_comment()?;
                }

                // Math Operator
                '+' | '-' | '*' | '/' | '%' => {
                    self.consume_math_operator();
                }

                '{' => {self.tokens.push(Token::LeftBrace); self.index += 1;},
                '}' => {self.tokens.push(Token::RightBrace); self.index += 1;},
                '(' => {self.tokens.push(Token::LeftParen); self.index += 1;},
                ')' => {self.tokens.push(Token::RightParen); self.index += 1;},
                ',' => {self.tokens.push(Token::Comma); self.index += 1;}


                'a'..='z' | 'A'..='Z' | '_' => {
                    self.consume_keyword_and_identifier();
                }



                // Consume all operators here related to bitwise, comparison, logic(e.g., '>=', '<=', '!=')
                '>' | '<' | '=' | '^' | '&' | '|' | '!' | ':'  => {
                    self.consume_operator();

                }    
            
                // Number (0-9)
                '0'..='9' => self.consume_number(),

                _ => return Err(LexerError::UnexpectedCharacter(ch)),
            }
        }

        self.tokens.push(Token::EndOfInput);
        Ok(&self.tokens)
    }


    fn consume_operator(&mut self) {
        let ch = self.input.get(self.index).copied().unwrap_or_default();
        let mut operator = String::new();
        operator.push(ch);
        self.index += 1;

        // Handle two-character operators first
        if let Some(next_ch) = self.input.get(self.index).copied() {
            match (ch, next_ch) {
                // Comparison Operators (e.g., <=, >=, ==, !=)
                ('<', '=') | ('>', '=') | ('=', '=') | ('!', '=') => {
                    operator.push(next_ch);
                    self.index += 1;
                    match operator.parse::<ComparisonOperator>() {
                        Ok(op) => self.tokens.push(Token::ComparisonOperatorToken(op)),
                        Err(_) => {}
                    }
                    return;
                },
                // Logical Operators (e.g., &&, ||)
                ('&', '&') | ('|', '|') => {
                    operator.push(next_ch);
                    self.index += 1;
                    match operator.parse::<LogicalOperator>() {
                        Ok(op) => self.tokens.push(Token::LogicalOperatorToken(op)),
                        Err(_) => {} 
                    }
                    return;
                },
                // Bitwise Shift Operators (e.g., <<, >>)
                ('<', '<') | ('>', '>') => {
                    operator.push(next_ch);
                    self.index += 1;
                    match operator.parse::<BitwiseOperator>() {
                        Ok(op) => self.tokens.push(Token::BitwiseOperatorToken(op)),
                        Err(_) => {} 
                    }
                    return;
                },

                (':', '=') => {
                    operator.push(next_ch);
                    self.index += 1;
                    match operator.parse::<Operator>() {
                        Ok(op) => self.tokens.push(Token::OperatorToken(op)),
                        Err(_) => {}
                    }
                    return;
                }
                _ => {} 
            }
        }

        // Single operators below
        match operator.parse::<ComparisonOperator>() {
            Ok(op) => self.tokens.push(Token::ComparisonOperatorToken(op)),
            Err(_) => {} // Handle invalid comparison operators if necessary
        }

        match operator.parse::<LogicalOperator>() {
            Ok(op) => self.tokens.push(Token::LogicalOperatorToken(op)),
            Err(_) => {} // Handle invalid logical operators if necessary
        }

        match operator.parse::<BitwiseOperator>() {
            Ok(op) => self.tokens.push(Token::BitwiseOperatorToken(op)),
            Err(_) => {} // Handle invalid bitwise operators if necessary
        }

        // General Operator (e.g., '=', '=>')
        match operator.parse::<Operator>() {
            Ok(op) => self.tokens.push(Token::OperatorToken(op)),
            Err(_) => {} // Handle invalid general operators if necessary
        }
    }

    
    fn consume_keyword_and_identifier(&mut self) {
        let mut identifier = String::new();

        while let Some(ch) = self.input.get(self.index).copied() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.index += 1;
            } else {
                break;
            }
        }

        let keywords = ["if", "else", "elif", "while", "for", "in", "return", "print", "fn"];
        if keywords.contains(&identifier.as_str()) {
            self.tokens.push(Token::Keyword(identifier));
        } else {
            self.tokens.push(Token::Identifier(identifier));
        }
    }



    fn consume_comment(&mut self) -> Result<(), LexerError> {
        let mut comment = String::new();

        while let Some(ch) = self.input.get(self.index).copied() {
            if let Some(next_ch) = self.peek_next() {
                if ch == '*' && next_ch == '/' {
                    self.index += 2; 
                    self.tokens.push(Token::Comment(comment));
                    return Ok(());
                }
            }
            comment.push(ch);
            self.index += 1;

            if self.index >= self.input.len() {
                return Err(LexerError::UnterminatedComment);
            }
        }

        Err(LexerError::UnterminatedComment)
    }

    
    fn consume_math_operator(&mut self) {
        if let Some(ch) = self.input.get(self.index).copied() {
            let mut operator = ch.to_string();
            self.index += 1;

            // Handle double-character operators (e.g., "**", "//")
            if let Some(next_ch) = self.input.get(self.index).copied() {
                if (ch == '*' && next_ch == '*') || (ch == '/' && next_ch == '/') {
                    operator.push(next_ch);
                    self.index += 1;
                }
            }

            match operator.parse::<MathOperator>() {
                Ok(op) => self.tokens.push(Token::MathOperatorToken(op)),
                Err(_) => {} // Handle invalid math operators if necessary
            }
        }
    }

    
    fn consume_number(&mut self) {
        let mut number = String::new();
        let mut is_float = false;

        while let Some(ch) = self.input.get(self.index).copied() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.index += 1;
            } else if ch == '.' && !is_float {
                number.push(ch);
                self.index += 1;
                is_float = true;
            } else {
                break;
            }
        }

        if let Ok(value) = number.parse::<f64>() {
            self.tokens.push(Token::Number(value))
        }
    }
}
