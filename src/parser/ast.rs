use crate::common::span::Spanned;
use crate::lexer::TokenKind;
use crate::common::{Symbol, Type};

// Spanned type aliases for expressions, statements, and literals
pub type Expr = Spanned<ExprKind>;
pub type Stmt = Spanned<StmtKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    /// left op right
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },

    /// op expr
    Unary { op: TokenKind, expr: Box<Expr> },

    /// 42, "hello", true, nil
    Literal(LiteralValue),

    /// variable name
    Identifier(Symbol),

    /// name = value
    Assign { 
        name: Symbol, 
        value: Box<Expr> 
    },

    // [1, 2, 3]
    ArrayLiteral(Vec<Expr>),

    /// arr[index]
    Get {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    /// arr[index] = value
    Set {
        object: Box<Expr>,
        index: Box<Expr>,
        value: Box<Expr>,
    },

    /// callee(arguments...)
    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}


#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    ExprStmt(Box<Expr>),
    // PrintStmt(Box<Expr>),
    If { cond: Box<Expr>, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Box<Expr>, body: Box<Stmt> },
    Block{ stmts: Vec<Stmt> },
    LetStmt { 
        name: Symbol, 
        type_annotation: Option<Type>, 
        initializer: Box<Expr> 
    },
    // TODO: FuncDeclaration { name: Token, params: Vec<(Token, Option<TokenKind>)>, return_type: Option<TokenKind>, body: Box<Stmt> },
}


impl ExprKind {
    pub fn type_to_string(&self) -> &'static str {
        match self {
            ExprKind::Binary { .. } => "binary expression",
            ExprKind::Unary { .. } => "unary expression",
            ExprKind::Literal(_) => "literal",
            ExprKind::Identifier(_) => "identifier",
            ExprKind::Assign { .. } => "assignment",
            ExprKind::ArrayLiteral(_) => "array literal",
            ExprKind::Get { .. } => "array access",
            ExprKind::Set { .. } => "array assignment",
            ExprKind::FunctionCall { .. } => "function call",
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Integer(n) => write!(f, "{}", n),
            LiteralValue::Float(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "\"{}\"", s),
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl LiteralValue {
    pub fn get_type(&self) -> crate::common::Type {
        match self {
            LiteralValue::Integer(_) => crate::common::Type::Integer,
            LiteralValue::Float(_) => crate::common::Type::Float,
            LiteralValue::String(_) => crate::common::Type::String,
            LiteralValue::Boolean(_) => crate::common::Type::Boolean,
            LiteralValue::Nil => crate::common::Type::Nil,
        }
    }
}

// TODO: use the Spanned type to define Expr and Stmt