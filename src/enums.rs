
#[derive(Debug)]
pub enum Num {
    Int(usize),
    Float(f64)
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Len
}

#[derive(Debug)]
pub enum ConditionalOp {
    And,
    Not,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual
}

#[derive(Debug)]
pub enum Token {
    Local,
    Identifier(String),
    Assignment,
    
    NumLiteral(Num),
    StrLiteral(String),

    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // Conditional keywords
    If,
    Then,
    Else,
    Elif,
    End,

    // Conditional Ops
    CondOp(ConditionalOp),

    // Binary Ops
    BinOp(BinaryOp),

    // Unary ops
    UnOp(UnaryOp),

    Unknown(String), // String containing the unknown
    Terminator // End of Statement Semi-colon or New line
}