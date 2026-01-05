use crate::lexer::{Token, TokenKind};
use crate::common::Symbol;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: crate::common::Span,
}

// Spanned type aliases for expressions, statements, and literals
pub type Expr = Spanned<ExprKind>;
pub type Stmt = Spanned<StmtKind>;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Unary { op: TokenKind, expr: Box<Expr> },
    Literal(LiteralValue),
    Identifier(Symbol),
    Assign { name: Box<Expr>, value: Box<Expr> },
    Error,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
    If { cond: Box<Expr>, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Box<Expr>, body: Box<Stmt> },
    Block(Vec<Stmt>),
    VarDeclaration { name: Token, var_type: Option<TokenKind>, initializer: Option<Box<Expr>> },
    // TODO: FuncDeclaration { name: Token, params: Vec<(Token, Option<TokenKind>)>, return_type: Option<TokenKind>, body: Box<Stmt> },
    Error,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(Symbol),
    Boolean(bool),
    Nil,
    Error,
}

// TODO: use the Spanned type to define Expr and Stmt