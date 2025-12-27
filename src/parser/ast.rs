use crate::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary { left: Box<Expr>, op: TokenKind, right: Box<Expr> },
    Unary { op: TokenKind, expr: Box<Expr> },
    Literal(LiteralValue),
    Variable(Token),
    Assign { name: Box<Expr>, value: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
    If { cond: Expr, then: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Expr, body: Box<Stmt> },
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    // Nil,
}

impl Expr {
    pub fn pretty(&self, indent: usize) {
        let pad = "  ".repeat(indent);

        match self {
            Expr::Binary { left, op, right } => {
                let operation = match op {
                    TokenKind::Plus => "+",
                    TokenKind::Minus => "-",
                    TokenKind::Star => "*",
                    TokenKind::Slash => "/",
                    TokenKind::Modulus => "%",
                    TokenKind::EQ => "==",
                    TokenKind::NEQ => "!=",
                    TokenKind::LT => "<",
                    TokenKind::LEQ => "<=",
                    TokenKind::GT => ">",
                    TokenKind::GEQ => ">=",
                    _ => "ERROR TOKEN",
                };
                println!("{pad}Binary Operation:");
                left.pretty(indent + 1);
                println!("{pad}  {operation}");
                right.pretty(indent + 1);
            },
            Expr::Unary { op, expr } => {
                println!("{pad}Unary Operation:");
                match op {
                    TokenKind::Plus => println!("{pad}  +"),
                    TokenKind::Minus => println!("{pad} -"),
                    _ => println!("ERROR TOKEN"),
                };
                expr.pretty(indent + 1);
            },
            Expr::Literal(literal_value) => {
                match literal_value {
                    LiteralValue::Number(val) => println!("{pad}Number: {val}"),
                    LiteralValue::String(val) => println!("{pad}String: \"{val}\""),
                    LiteralValue::Boolean(val) => println!("{pad}Boolean: {val}"),
                };
            }
            Expr::Variable(token) => println!("{pad}Identifier: {}", token.lexeme),
            Expr::Assign { name, value } =>{
                print!("{pad}Assign:");
                name.pretty(indent + 1);
                value.pretty(indent + 1);
            },
        }
    }
}

impl Stmt {
    pub fn pretty(&self, indent: usize) {
        let pad = "  ".repeat(indent);

        match self {
            Self::ExprStmt(expr) => {
                println!("{pad}Expression Stmt:");
                expr.pretty(indent + 1);
            },
            Self::PrintStmt(expr) => {
                println!("{pad}Print:");
                expr.pretty(indent + 1);
            },
            Self::If { cond, then, else_branch } => {
                println!("{pad}If Statement:");
                println!("{pad}  Condition:");
                cond.pretty(indent + 2);
                println!("{pad}  Then:");
                then.pretty(indent + 2);
                if let Some(other) = else_branch {
                    println!("{pad}  Else:");
                    other.pretty(indent + 2);

                }
            },
            Self::While { cond, body } => {
                println!("{pad}While:");
                println!("{pad}  Cond:");
                cond.pretty(indent + 2);
                body.pretty(indent + 1);
            },
            Self::Block(stmts) => {
                println!("{pad}Block:");
                for stmt in stmts {
                    stmt.pretty(indent + 1);
                }
            }
        }
    }
}