pub enum Token {
    Local,
    Identifier(String),
    Assignment,
    
    NumLiteral(usize),
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
    Not,
    Eq,
    NotEq,
    LT,
    GT,
    LTe,
    GTe,

    // Binary Ops
    Add,
    Sub,
    Mul,
    Div,
    Exp,

    // Unary ops
    Neg,
    Len, // #

    Terminator // End of Statement Semi-colon or New line
}
