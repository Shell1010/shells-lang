use std::str::FromStr;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MathOperator {
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Modulus,    // %
}

impl FromStr for MathOperator {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "+" => Ok(MathOperator::Add),
            "-" => Ok(MathOperator::Subtract),
            "*" => Ok(MathOperator::Multiply),
            "/" => Ok(MathOperator::Divide),
            "%" => Ok(MathOperator::Modulus),
            _   => Err(format!("Invalid MathOperator: {}", op)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOperator {
    Not,        // !
    And,        // &&
    Or,         // ||
    Xor,        // ^
}

impl FromStr for LogicalOperator {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "!"  => Ok(LogicalOperator::Not),
            "&&" => Ok(LogicalOperator::And),
            "||" => Ok(LogicalOperator::Or),
            "^^"  => Ok(LogicalOperator::Xor),
            _    => Err(format!("Invalid LogicalOperator: {}", op)),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum BitwiseOperator {
    LeftShift,  // <<
    RightShift, // >>
    And,        // &
    Or,         // |
    Xor,        // ^
}

impl FromStr for BitwiseOperator {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "<<" => Ok(BitwiseOperator::LeftShift),
            ">>" => Ok(BitwiseOperator::RightShift),
            "&"  => Ok(BitwiseOperator::And),
            "|"  => Ok(BitwiseOperator::Or),
            "^"  => Ok(BitwiseOperator::Xor),
            _    => Err(format!("Invalid BitwiseOperator: {}", op)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOperator {
    Equals,        // =
    NotEquals,     // !=
    GreaterThan,   // >
    LessThan,      // <
    GreaterThanEq, // >=
    LessThanEq,    // <=
}

impl FromStr for ComparisonOperator {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "=="  => Ok(ComparisonOperator::Equals),
            "!=" => Ok(ComparisonOperator::NotEquals),
            ">"  => Ok(ComparisonOperator::GreaterThan),
            "<"  => Ok(ComparisonOperator::LessThan),
            ">=" => Ok(ComparisonOperator::GreaterThanEq),
            "<=" => Ok(ComparisonOperator::LessThanEq),
            _    => Err(format!("Invalid ComparisonOperator: {}", op)),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equals, // =
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "=" => Ok(Operator::Equals),
            _   => Err(format!("Invalid Operator: {}", op)),
        }
    }
}

