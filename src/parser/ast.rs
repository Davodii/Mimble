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
    If { cond: Box<Expr>, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Box<Expr>, body: Box<Stmt> },
    Block{ stmts: Vec<Stmt> },
    LetStmt { 
        name: Symbol, 
        type_annotation: Option<Type>, 
        initializer: Box<Expr> 
    },
    Break,
    Continue,
    FuncDeclaration { 
        name: Symbol, // Identifier
        params: Vec<(Symbol, Option<Type>)>,  // Identifier, optional type annotation
        return_type: Option<Type>, // Optional return type annotation
        body: Box<Stmt> // Block
    },
    Return { value: Box<Expr> },
}


impl ExprKind {
    pub fn kind_to_string(&self) -> &'static str {
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

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Integer(n1), LiteralValue::Integer(n2)) => n1 == n2,
            (LiteralValue::Float(n1), LiteralValue::Float(n2)) => n1 == n2,
            (LiteralValue::String(s1), LiteralValue::String(s2)) => s1 == s2,
            (LiteralValue::Boolean(b1), LiteralValue::Boolean(b2)) => b1 == b2,
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            _ => false,
        }
    }
}

impl PartialEq for ExprKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExprKind::Binary { left: l1, op: o1, right: r1 }, ExprKind::Binary { left: l2, op: o2, right: r2 }) => {
                o1 == o2 && l1 == l2 && r1 == r2
            }
            (ExprKind::Unary { op: o1, expr: e1 }, ExprKind::Unary { op: o2, expr: e2 }) => {
                o1 == o2 && e1 == e2
            }
            (ExprKind::Literal(lit1), ExprKind::Literal(lit2)) => lit1 == lit2,
            (ExprKind::Identifier(id1), ExprKind::Identifier(id2)) => id1 == id2,
            (ExprKind::Assign { name: n1, value: v1 }, ExprKind::Assign { name: n2, value: v2 }) => {
                n1 == n2 && v1 == v2
            }
            (ExprKind::ArrayLiteral(elements1), ExprKind::ArrayLiteral(elements2)) => elements1 == elements2,
            (ExprKind::Get { object: o1, index: i1 }, ExprKind::Get { object: o2, index: i2 }) => {
                o1 == o2 && i1 == i2
            }
            (ExprKind::Set { object: o1, index: i1, value: v1 }, ExprKind::Set { object: o2, index: i2, value: v2 }) => {
                o1 == o2 && i1 == i2 && v1 == v2
            }
            (ExprKind::FunctionCall { callee: c1, arguments: a1 }, ExprKind::FunctionCall { callee: c2, arguments: a2 }) => {
                c1 == c2 && a1 == a2
            }
            _ => false,
        }
    }
}

impl PartialEq for StmtKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StmtKind::ExprStmt(e1), StmtKind::ExprStmt(e2)) => e1 == e2,
            (StmtKind::If { cond: c1, then: t1, else_branch: e1 }, StmtKind::If { cond: c2, then: t2, else_branch: e2 }) => {
                c1 == c2 && t1 == t2 && e1 == e2
            }
            (StmtKind::While { cond: c1, body: b1 }, StmtKind::While { cond: c2, body: b2 }) => {
                c1 == c2 && b1 == b2
            }
            (StmtKind::Block { stmts: s1 }, StmtKind::Block { stmts: s2 }) => s1 == s2,
            (StmtKind::LetStmt { name: n1, type_annotation: t1, initializer: i1 }, StmtKind::LetStmt { name: n2, type_annotation: t2, initializer: i2 }) => {
                n1 == n2 && t1 == t2 && i1 == i2
            }
            _ => false,
        }
    }
}