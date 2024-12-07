use std::collections::VecDeque;


// Token: Represents all possible tokens in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),      // e.g., x, y
    Number(f64),             // e.g., 10, 2.5
    Keyword(String),         // e.g., if, print
    Operator(String),        // e.g., =, *, >, <<, >>
    MathOperator(String),    // e.g., +, -, **, //, %
    LogicalOperator(String), // e.g., NOT, OR, AND, XOR, &&, ||, !, ^
    BitwiseOperator(String), // e.g., <<, >>
    LeftBrace,               // {
    RightBrace,              // }
    LeftParen,               // (
    RightParen,              // )
    Comment(String),         // Block comment: /* ... */
    EndOfInput,              // End of file
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
        let input = input.chars().collect::<VecDeque<_>>(); // Collect input into VecDeque for efficient peeking
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
                '+' | '-' | '*' | '/' | '%'  => {
                    self.consume_math_operator();
                }


                // Parentheses and braces
                '{' => {self.tokens.push(Token::LeftBrace); self.index += 1;},
                '}' => {self.tokens.push(Token::RightBrace); self.index += 1;},
                '(' => {self.tokens.push(Token::LeftParen); self.index += 1;},
                ')' => {self.tokens.push(Token::RightParen); self.index += 1;},

                // Logical operators
                '!' | '^' | '&' | '|' => {
                    self.consume_logical_operator();
                }

                // Bitwise operators
                '<' | '>' => {
                    self.consume_bitwise_operator();
                }

                // Keywords and identifiers - variables
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.consume_keyword_and_identifier();
                }


                // GeneralOperator
                '=' => {
                    self.consume_operator();
                }

                // Numba
                '0'..='9' => self.consume_number(),


                _ => return Err(LexerError::UnexpectedCharacter(ch)),
            }
        }

        self.tokens.push(Token::EndOfInput);
        Ok(&self.tokens)
    }
    
    fn consume_bitwise_operator(&mut self) {
        let mut operator = String::new();

        if let Some(ch) = self.input.get(self.index).copied() {
            operator.push(ch);
            self.index += 1;

            if let Some(next_ch) = self.input.get(self.index).copied() {
                if (ch == '<' && next_ch == '<') || (ch == '>' && next_ch == '>') {
                    operator.push(next_ch);
                    self.index += 1;
                }
            }
        }

        self.tokens.push(Token::BitwiseOperator(operator));
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

        let keywords = ["if", "else", "while", "for", "return", "print"];
        if keywords.contains(&identifier.as_str()) {
            self.tokens.push(Token::Keyword(identifier));
        } else {
            self.tokens.push(Token::Identifier(identifier));
        }
    }

    fn consume_logical_operator(&mut self) {
        let mut operator = String::new();

        if let Some(ch) = self.input.get(self.index).copied() {
            operator.push(ch);
            self.index += 1;

            if let Some(next_ch) = self.input.get(self.index).copied() {
                if (ch == '&' && next_ch == '&') || (ch == '|' && next_ch == '|') {
                    operator.push(next_ch);
                    self.index += 1;
                }
            }
        }

        self.tokens.push(Token::LogicalOperator(operator));
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
        let mut operator = String::new();

        if let Some(ch) = self.input.get(self.index).copied() {
            operator.push(ch); 
            self.index += 1;

            if let Some(next_ch) = self.input.get(self.index).copied() {
                if (ch == '*' && next_ch == '*') || (ch == '/' && next_ch == '/') {
                    operator.push(next_ch); 
                    self.index += 1;
                }
            }
        }

        self.tokens.push(Token::MathOperator(operator));
    }

    fn consume_operator(&mut self) {
        let op = self.input[self.index];
        self.tokens.push(Token::Operator(op.to_string()));
        self.index += 1;
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
